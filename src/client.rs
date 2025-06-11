//! Client connection handling
//!
//! Manages individual client connections and protocol state machines.

use crate::protocol::{
    ConnectionState, HandshakePacket, Packet, PingRequest, PongResponse, StatusRequest,
    StatusResponse, packet::parse_packet_data,
};
use std::io::Cursor;
use tokio::{io::AsyncReadExt, net::TcpStream};

/// Represents a connected client
pub struct ClientConnection {
    /// TCP stream for this client
    stream: TcpStream,
    /// Current connection state
    state: ConnectionState,
    /// Client address for logging
    addr: std::net::SocketAddr,
}

impl ClientConnection {
    /// Create a new client connection
    pub fn new(stream: TcpStream, addr: std::net::SocketAddr) -> Self {
        Self {
            stream,
            state: ConnectionState::Handshaking,
            addr,
        }
    }

    /// Handle the client connection
    pub async fn handle(mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        tracing::debug!("Starting client handler for {}", self.addr);

        let mut buffer = [0u8; 4096];

        loop {
            let bytes_read = match self.stream.read(&mut buffer).await {
                Ok(0) => {
                    tracing::debug!("Client {} disconnected", self.addr);
                    break;
                }
                Ok(n) => n,
                Err(e) => {
                    tracing::error!("Failed to read from {}: {:?}", self.addr, e);
                    break;
                }
            };

            if let Err(e) = self.process_data(&buffer[..bytes_read]).await {
                tracing::error!("Error processing data from {}: {:?}", self.addr, e);
                break;
            }
        }

        Ok(())
    }

    /// Process incoming data from the client
    async fn process_data(
        &mut self,
        data: &[u8],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut offset = 0;

        while offset < data.len() {
            // Try to read packet length
            let remaining = &data[offset..];
            if remaining.is_empty() {
                break;
            }

            let (packet_length, length_size) =
                match crate::protocol::varint::VarInt::from_bytes(remaining) {
                    Ok((varint, size)) => (varint.0 as usize, size),
                    Err(_) => {
                        tracing::debug!("Incomplete packet length from {}", self.addr);
                        break;
                    }
                };

            // Check if we have the complete packet
            if remaining.len() < length_size + packet_length {
                tracing::debug!(
                    "Incomplete packet from {} (need {} bytes, have {})",
                    self.addr,
                    length_size + packet_length,
                    remaining.len()
                );
                break;
            }

            // Extract packet data
            let packet_data = &remaining[length_size..length_size + packet_length];

            tracing::debug!(
                "Processing packet from {} (length: {}, data: {:?})",
                self.addr,
                packet_length,
                packet_data
            );

            // Process the packet
            self.handle_packet(packet_data).await?;

            // Move to next packet
            offset += length_size + packet_length;
        }

        Ok(())
    }

    /// Handle a specific packet based on current state
    async fn handle_packet(
        &mut self,
        packet_data: &[u8],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let (packet_id, data) = parse_packet_data(packet_data)?;

        tracing::debug!(
            "Handling packet ID {} in state {:?} from {}",
            packet_id,
            self.state,
            self.addr
        );

        match self.state {
            ConnectionState::Handshaking => {
                if packet_id == HandshakePacket::packet_id() {
                    self.handle_handshake(&data).await?;
                } else {
                    tracing::warn!(
                        "Unexpected packet {} in handshaking state from {}",
                        packet_id,
                        self.addr
                    );
                }
            }
            ConnectionState::Status => match packet_id {
                id if id == StatusRequest::packet_id() => {
                    self.handle_status_request().await?;
                }
                id if id == PingRequest::packet_id() => {
                    self.handle_ping_request(&data).await?;
                }
                _ => {
                    tracing::warn!(
                        "Unexpected packet {} in status state from {}",
                        packet_id,
                        self.addr
                    );
                }
            },
            _ => {
                tracing::warn!(
                    "Unhandled state {:?} for packet {} from {}",
                    self.state,
                    packet_id,
                    self.addr
                );
            }
        }

        Ok(())
    }

    /// Handle handshake packet
    async fn handle_handshake(
        &mut self,
        data: &[u8],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let handshake = HandshakePacket::from_packet_data(data)?;

        tracing::debug!(
            "Handshake from {}: protocol_version={}, server_address={}, server_port={}, next_state={}",
            self.addr,
            handshake.protocol_version,
            handshake.server_address,
            handshake.server_port,
            handshake.next_state
        );

        // Update connection state based on handshake
        if let Some(next_state) = handshake.get_next_state() {
            self.state = next_state;
            tracing::debug!(
                "Client {} transitioned to state {:?}",
                self.addr,
                self.state
            );
        } else {
            tracing::warn!(
                "Invalid next_state {} in handshake from {}",
                handshake.next_state,
                self.addr
            );
        }

        Ok(())
    }

    /// Handle status request
    async fn handle_status_request(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        tracing::debug!("Status request from {}", self.addr);

        let response = StatusResponse::new();
        self.send_packet(&response).await?;

        Ok(())
    }

    /// Handle ping request
    async fn handle_ping_request(
        &mut self,
        data: &[u8],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut cursor = Cursor::new(data);
        let ping = PingRequest::read_data(&mut cursor)?;

        tracing::debug!(
            "Ping request from {} with payload {}",
            self.addr,
            ping.payload
        );

        let pong = PongResponse::from_ping(&ping);
        self.send_packet(&pong).await?;

        Ok(())
    }

    /// Send a packet to the client
    async fn send_packet<P: Packet>(
        &mut self,
        packet: &P,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use tokio::io::AsyncWriteExt;

        let mut buffer = Vec::new();
        packet.write_packet(&mut buffer)?;

        tracing::debug!(
            "Sending packet to {} (length: {}, data: {:?})",
            self.addr,
            buffer.len(),
            buffer
        );

        self.stream.write_all(&buffer).await?;
        self.stream.flush().await?;

        Ok(())
    }
}
