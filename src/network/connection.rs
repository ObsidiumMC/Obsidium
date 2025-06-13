//! Connection management
//!
//! This module handles individual client connections and their lifecycle.

use crate::error::{ServerError, Result};
use crate::protocol::{ConnectionState, ProtocolState, Compression};
use crate::protocol::types::VarInt;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use std::time::{Duration, Instant};

/// Represents a single client connection
pub struct Connection {
    /// TCP stream
    stream: TcpStream,
    /// Client address
    peer_addr: SocketAddr,
    /// Protocol state
    protocol_state: ProtocolState,
    /// Compression handler
    compression: Option<Compression>,
    /// Connection start time
    connected_at: Instant,
    /// Last activity time
    last_activity: Instant,
}

impl Connection {
    /// Create a new connection from a TCP stream
    pub fn new(stream: TcpStream, peer_addr: SocketAddr) -> Self {
        let now = Instant::now();
        Self {
            stream,
            peer_addr,
            protocol_state: ProtocolState::new(),
            compression: None,
            connected_at: now,
            last_activity: now,
        }
    }
    
    /// Get the peer address
    pub fn peer_addr(&self) -> SocketAddr {
        self.peer_addr
    }
    
    /// Get the current connection state
    pub fn state(&self) -> ConnectionState {
        self.protocol_state.state
    }
    
    /// Transition to a new connection state
    pub fn set_state(&mut self, new_state: ConnectionState) {
        self.protocol_state.transition_to(new_state);
    }
    
    /// Enable compression with the given threshold
    pub fn enable_compression(&mut self, threshold: u32) -> Result<()> {
        self.protocol_state.enable_compression(threshold);
        self.compression = Some(Compression::new(threshold));
        tracing::debug!("Compression enabled for connection {}", self.peer_addr);
        Ok(())
    }
    
    /// Read a packet from the connection
    pub async fn read_packet(&mut self) -> Result<(VarInt, Vec<u8>)> {
        self.last_activity = Instant::now();
        
        // Read packet length
        let packet_length = self.read_varint().await?;
        
        if packet_length.0 < 0 {
            return Err(ServerError::Protocol("Negative packet length".to_string()));
        }
        
        let length = packet_length.0 as usize;
        if length == 0 {
            return Err(ServerError::Protocol("Zero packet length".to_string()));
        }
        
        if length > crate::protocol::MAX_PACKET_SIZE {
            return Err(ServerError::Protocol("Packet too large".to_string()));
        }
        
        // Read packet data
        let mut data = vec![0u8; length];
        self.stream.read_exact(&mut data).await?;
        
        // Debug: log the raw packet data
        if data.len() <= 32 {
            tracing::debug!("Raw packet data: {:02X?}", data);
        } else {
            tracing::debug!("Raw packet data (first 32): {:02X?}", &data[..32]);
        }
        
        // Handle compression if enabled
        if let Some(ref mut compression) = self.compression {
            compression.decompress_packet(&data)
        } else {
            // Uncompressed packet - first VarInt is packet ID
            let mut cursor = std::io::Cursor::new(&data);
            let packet_id = VarInt::read(&mut cursor)?;
            let remaining_data = data[cursor.position() as usize..].to_vec();
            Ok((packet_id, remaining_data))
        }
    }
    
    /// Write a packet to the connection
    pub async fn write_packet<P>(&mut self, packet: &P) -> Result<()>
    where
        P: crate::protocol::packets::Packet,
    {
        self.last_activity = Instant::now();
        
        // Serialize packet data (without packet ID)
        let mut packet_data = Vec::new();
        packet.write(&mut packet_data)?;
        
        tracing::debug!(
            "Writing packet ID: 0x{:02X}, data length: {}, compression: {}",
            P::ID,
            packet_data.len(),
            self.compression.is_some()
        );
        
        // Handle compression if enabled
        let final_data = if let Some(ref mut compression) = self.compression {
            compression.compress_packet(P::id(), &packet_data)?
        } else {
            // Uncompressed - manually build: length + packet_id + data
            let mut result = Vec::new();
            
            // Calculate total length (packet ID + data)
            let packet_id = P::id();
            let total_length = packet_id.len() + packet_data.len();
            
            // Write length prefix
            VarInt(total_length as i32).write(&mut result)?;
            
            // Write packet ID
            packet_id.write(&mut result)?;
            
            // Write packet data
            result.extend_from_slice(&packet_data);
            
            result
        };
        
        tracing::debug!("Final packet size: {} bytes", final_data.len());
        
        // Debug: log the first few bytes of the packet
        if final_data.len() <= 32 {
            tracing::debug!("Packet bytes: {:02X?}", final_data);
        } else {
            tracing::debug!("Packet bytes (first 32): {:02X?}", &final_data[..32]);
        }
        
        // Write to stream
        self.stream.write_all(&final_data).await?;
        self.stream.flush().await?;
        
        Ok(())
    }
    
    /// Read raw bytes from the connection
    pub async fn read_bytes(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.last_activity = Instant::now();
        let bytes_read = self.stream.read(buf).await?;
        Ok(bytes_read)
    }
    
    /// Write raw bytes to the connection
    pub async fn write_bytes(&mut self, data: &[u8]) -> Result<()> {
        self.last_activity = Instant::now();
        self.stream.write_all(data).await?;
        self.stream.flush().await?;
        Ok(())
    }
    
    /// Check if the connection has timed out
    pub fn is_timed_out(&self, timeout: Duration) -> bool {
        self.last_activity.elapsed() > timeout
    }
    
    /// Get connection uptime
    pub fn uptime(&self) -> Duration {
        self.connected_at.elapsed()
    }
    
    /// Get time since last activity
    pub fn idle_time(&self) -> Duration {
        self.last_activity.elapsed()
    }
    
    /// Set protocol version
    pub fn set_protocol_version(&mut self, version: i32) {
        self.protocol_state.set_protocol_version(version);
    }
    
    /// Get protocol version
    pub fn protocol_version(&self) -> Option<i32> {
        self.protocol_state.protocol_version
    }
    
    /// Close the connection
    pub async fn close(&mut self) -> Result<()> {
        self.stream.shutdown().await?;
        tracing::debug!("Connection {} closed", self.peer_addr);
        Ok(())
    }
    
    /// Read a VarInt from the connection
    async fn read_varint(&mut self) -> Result<VarInt> {
        let mut value = 0i32;
        let mut position = 0;
        
        loop {
            let mut byte = [0u8; 1];
            self.stream.read_exact(&mut byte).await?;
            let byte = byte[0];
            
            value |= ((byte & 0x7F) as i32) << position;
            
            if (byte & 0x80) == 0 {
                break;
            }
            
            position += 7;
            if position >= 32 {
                return Err(ServerError::Protocol("VarInt too long".to_string()));
            }
        }
        
        Ok(VarInt(value))
    }
}
