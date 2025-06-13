//! Player management
//!
//! This module handles player state, authentication, and player-specific logic.

use crate::protocol::types::McUuid;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Represents a connected player
#[derive(Debug, Clone)]
pub struct Player {
    /// Player UUID
    pub uuid: McUuid,
    /// Player username
    pub username: String,
    /// Player position
    pub position: PlayerPosition,
    /// Player rotation
    pub rotation: PlayerRotation,
    /// Player game mode
    pub game_mode: GameMode,
    /// Player health
    pub health: f32,
    /// Player food level
    pub food: i32,
    /// Player experience
    pub experience: PlayerExperience,
    /// Whether the player is on ground
    pub on_ground: bool,
}

/// Player position in the world
#[derive(Debug, Clone, Copy)]
pub struct PlayerPosition {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
    /// Z coordinate
    pub z: f64,
}

/// Player rotation (look direction)
#[derive(Debug, Clone, Copy)]
pub struct PlayerRotation {
    /// Yaw (horizontal rotation)
    pub yaw: f32,
    /// Pitch (vertical rotation)
    pub pitch: f32,
}

/// Player game mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    /// Survival mode
    Survival = 0,
    /// Creative mode
    Creative = 1,
    /// Adventure mode
    Adventure = 2,
    /// Spectator mode
    Spectator = 3,
}

/// Player experience information
#[derive(Debug, Clone, Copy)]
pub struct PlayerExperience {
    /// Current experience points
    pub points: i32,
    /// Current level
    pub level: i32,
    /// Progress to next level (0.0 - 1.0)
    pub progress: f32,
}

impl Player {
    /// Create a new player
    pub fn new(uuid: McUuid, username: String) -> Self {
        Self {
            uuid,
            username,
            position: PlayerPosition {
                x: 0.0,
                y: 64.0,
                z: 0.0,
            },
            rotation: PlayerRotation {
                yaw: 0.0,
                pitch: 0.0,
            },
            game_mode: GameMode::Survival,
            health: 20.0,
            food: 20,
            experience: PlayerExperience {
                points: 0,
                level: 0,
                progress: 0.0,
            },
            on_ground: true,
        }
    }

    /// Update player position
    pub fn set_position(&mut self, x: f64, y: f64, z: f64) {
        self.position.x = x;
        self.position.y = y;
        self.position.z = z;
    }

    /// Update player rotation
    pub fn set_rotation(&mut self, yaw: f32, pitch: f32) {
        self.rotation.yaw = yaw;
        self.rotation.pitch = pitch;
    }

    /// Set game mode
    pub fn set_game_mode(&mut self, mode: GameMode) {
        self.game_mode = mode;
    }

    /// Set health
    pub fn set_health(&mut self, health: f32) {
        self.health = health.clamp(0.0, 20.0);
    }

    /// Set food level
    pub fn set_food(&mut self, food: i32) {
        self.food = food.clamp(0, 20);
    }

    /// Check if player is alive
    pub fn is_alive(&self) -> bool {
        self.health > 0.0
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new(McUuid::nil(), "Unknown".to_string())
    }
}

/// Player manager for handling all connected players
pub struct PlayerManager {
    /// Map of UUID to player data
    players: Arc<RwLock<HashMap<McUuid, Player>>>,
    /// Map of connection address to player UUID
    connections: Arc<RwLock<HashMap<SocketAddr, McUuid>>>,
}

impl PlayerManager {
    /// Create a new player manager
    pub fn new() -> Self {
        Self {
            players: Arc::new(RwLock::new(HashMap::new())),
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add a new player
    pub async fn add_player(&self, player: Player, connection_addr: SocketAddr) {
        let uuid = player.uuid;

        {
            let mut players = self.players.write().await;
            players.insert(uuid, player);
        }

        {
            let mut connections = self.connections.write().await;
            connections.insert(connection_addr, uuid);
        }

        tracing::info!("Player {} connected from {}", uuid, connection_addr);
    }

    /// Remove a player
    pub async fn remove_player(&self, connection_addr: SocketAddr) -> Option<Player> {
        let uuid = {
            let mut connections = self.connections.write().await;
            connections.remove(&connection_addr)
        };

        if let Some(uuid) = uuid {
            let mut players = self.players.write().await;
            let player = players.remove(&uuid);

            if let Some(ref player) = player {
                tracing::info!(
                    "Player {} disconnected from {}",
                    player.username,
                    connection_addr
                );
            }

            player
        } else {
            None
        }
    }

    /// Get a player by UUID
    pub async fn get_player(&self, uuid: &McUuid) -> Option<Player> {
        let players = self.players.read().await;
        players.get(uuid).cloned()
    }

    /// Get a player by connection address
    pub async fn get_player_by_addr(&self, addr: &SocketAddr) -> Option<Player> {
        let uuid = {
            let connections = self.connections.read().await;
            connections.get(addr).copied()
        };

        if let Some(uuid) = uuid {
            self.get_player(&uuid).await
        } else {
            None
        }
    }

    /// Update a player
    pub async fn update_player(&self, uuid: &McUuid, player: Player) {
        let mut players = self.players.write().await;
        players.insert(*uuid, player);
    }

    /// Get all connected players
    pub async fn get_all_players(&self) -> Vec<Player> {
        let players = self.players.read().await;
        players.values().cloned().collect()
    }

    /// Get player count
    pub async fn player_count(&self) -> usize {
        let players = self.players.read().await;
        players.len()
    }
}

impl Default for PlayerManager {
    fn default() -> Self {
        Self::new()
    }
}
