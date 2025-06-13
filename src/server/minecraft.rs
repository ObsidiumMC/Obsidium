//! Main Minecraft server implementation
//!
//! This module contains the core server logic that ties together all
//! the other modules to create a functioning Minecraft server.

use crate::config::ServerConfig;
use crate::error::{ServerError, Result};
use crate::game::{player::PlayerManager, world::World};
use crate::network::{Connection, ServerListener};
use crate::protocol::{ConnectionState, PROTOCOL_VERSION};
use crate::protocol::packets::{
    Packet,
    handshaking::HandshakePacket,
    login::{LoginStartPacket, LoginSuccessPacket, SetCompressionPacket},
    status::{PingRequestPacket, PingResponsePacket, ServerStatus, StatusRequestPacket, StatusResponsePacket, VersionInfo, PlayersInfo, Description},
};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio::time::{interval, Duration};

/// Main Minecraft server
pub struct MinecraftServer {
    /// Server configuration
    config: ServerConfig,
    /// Player manager
    players: Arc<PlayerManager>,
    /// Main world
    world: Arc<RwLock<World>>,
    /// Server status
    status: ServerStatus,
}

impl MinecraftServer {
    /// Create a new Minecraft server
    pub async fn new(config: ServerConfig) -> Result<Self> {
        
        // Create server status
        let status = ServerStatus {
            version: VersionInfo {
                name: "1.21.5".to_string(),
                protocol: PROTOCOL_VERSION,
            },
            players: PlayersInfo {
                max: config.max_players,
                online: 0,
                sample: None,
            },
            description: Description::Text(config.motd.clone()),
            favicon: None,
            enforces_secure_chat: false,
        };
        
        Ok(Self {
            config,
            players: Arc::new(PlayerManager::new()),
            world: Arc::new(RwLock::new(World::new("world".to_string(), 12345))),
            status,
        })
    }
    
    /// Start the server
    pub async fn run(mut self) -> Result<()> {
        tracing::info!("Obsidium Minecraft Server v{}", env!("CARGO_PKG_VERSION"));
        
        // Create connection sender for the listener
        let (connection_sender, mut connection_receiver) = mpsc::unbounded_channel();
        
        // Start the network listener
        let listener = ServerListener::new(self.config.clone(), connection_sender).await?;
        let listener_addr = listener.local_addr()?;
        
        // Spawn listener task
        let _listener_handle = tokio::spawn(async move {
            if let Err(e) = listener.listen().await {
                tracing::error!("Listener error: {}", e);
            }
        });
        
        // Create update timer
        let mut update_timer = interval(Duration::from_millis(50)); // 20 TPS
        
        // Main server loop
        loop {
            tokio::select! {
                // Handle new connections
                Some(connection) = connection_receiver.recv() => {
                    let players = Arc::clone(&self.players);
                    let world = Arc::clone(&self.world);
                    let status = self.status.clone();
                    let config = self.config.clone();
                    
                    tokio::spawn(async move {
                        if let Err(e) = Self::handle_connection(connection, players, world, status, config).await {
                            tracing::error!("Connection error: {}", e);
                        }
                    });
                }
                
                // Update world and game logic
                _ = update_timer.tick() => {
                    let mut world = self.world.write().await;
                    world.update(0.05); // 50ms delta
                    
                    // Update player count in status
                    self.status.players.online = self.players.player_count().await as u32;
                }
            }
        }
    }
    
    /// Handle an individual connection
    async fn handle_connection(
        mut connection: Connection,
        players: Arc<PlayerManager>,
        _world: Arc<RwLock<World>>,
        status: ServerStatus,
        config: ServerConfig,
    ) -> Result<()> {
        tracing::debug!("Handling connection from {}", connection.peer_addr());
        
        loop {
            // Read packet
            let (packet_id, data) = match connection.read_packet().await {
                Ok((pid, pdata)) => {
                    tracing::debug!(
                        "Received packet ID: 0x{:02X}, data length: {}, state: {:?}",
                        pid.0,
                        pdata.len(),
                        connection.state()
                    );
                    (pid, pdata)
                }
                Err(e) => {
                    tracing::debug!("Connection closed: {}", e);
                    break;
                }
            };
            
            match connection.state() {
                ConnectionState::Handshaking => {
                    if packet_id.0 == HandshakePacket::ID {
                        let handshake = HandshakePacket::read(&mut std::io::Cursor::new(&data))?;
                        
                        tracing::debug!(
                            "Handshake: version={}, address={}, port={}, next_state={}",
                            handshake.protocol_version.0,
                            handshake.server_address.0,
                            handshake.server_port,
                            handshake.next_state.0
                        );
                        
                        connection.set_protocol_version(handshake.protocol_version.0);
                        
                        match handshake.next_state.0 {
                            1 => connection.set_state(ConnectionState::Status),
                            2 => connection.set_state(ConnectionState::Login),
                            _ => return Err(ServerError::Protocol("Invalid next state".to_string())),
                        }
                    }
                }
                
                ConnectionState::Status => {
                    if packet_id.0 == StatusRequestPacket::ID {
                        // Send status response
                        let json = status.to_json()?;
                        let response = StatusResponsePacket {
                            json_response: json.into(),
                        };
                        connection.write_packet(&response).await?;
                    } else if packet_id.0 == PingRequestPacket::ID {
                        let ping = PingRequestPacket::read(&mut std::io::Cursor::new(&data))?;
                        let pong = PingResponsePacket {
                            payload: ping.payload,
                        };
                        connection.write_packet(&pong).await?;
                        break; // Close connection after ping
                    }
                }
                
                ConnectionState::Login => {
                    if packet_id.0 == LoginStartPacket::ID {
                        let login_start = LoginStartPacket::read(&mut std::io::Cursor::new(&data))?;
                        
                        tracing::info!(
                            "Player {} ({}) logging in from {}",
                            login_start.name.0,
                            login_start.player_uuid,
                            connection.peer_addr()
                        );
                        
                        // Enable compression if configured
                        if let Some(threshold) = config.compression_threshold {
                            let compression_packet = SetCompressionPacket {
                                threshold: (threshold as i32).into(),
                            };
                            connection.write_packet(&compression_packet).await?;
                            connection.enable_compression(threshold)?;
                        }
                        
                        // Send login success
                        let login_success = LoginSuccessPacket {
                            uuid: login_start.player_uuid,
                            username: login_start.name.clone(),
                            properties: Vec::new(),
                            strict_error_handling: false,
                        };
                        connection.write_packet(&login_success).await?;
                        
                        // Create player
                        let player = crate::game::player::Player::new(
                            login_start.player_uuid,
                            login_start.name.0,
                        );
                        
                        players.add_player(player, connection.peer_addr()).await;
                        
                        connection.set_state(ConnectionState::Play);
                        
                        tracing::info!("Player logged in successfully");
                    }
                }
                
                ConnectionState::Play => {
                    // Handle play packets
                    tracing::debug!("Received play packet ID: 0x{:02X}", packet_id.0);
                    
                    // TODO: Implement play packet handlers
                    // For now, just log them
                }
            }
        }
        
        // Remove player when connection closes
        players.remove_player(connection.peer_addr()).await;
        
        Ok(())
    }
}

impl Drop for MinecraftServer {
    fn drop(&mut self) {
        tracing::info!("Obsidium Minecraft Server shutting down");
    }
}
