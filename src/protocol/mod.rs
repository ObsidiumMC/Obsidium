//! Minecraft protocol implementation
//! 
//! This module handles the Minecraft network protocol including packet parsing,
//! handshaking, status, and play state management.

pub mod varint;
pub mod packet;
pub mod handshake;
pub mod status;

pub use varint::VarInt;
pub use packet::{Packet, PacketReader, PacketWriter};
pub use handshake::HandshakePacket;
pub use status::{StatusRequest, StatusResponse, PingRequest, PongResponse};

/// Connection states in the Minecraft protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    /// Initial handshaking state
    Handshaking,
    /// Status/ping state
    Status,
    /// Login state
    Login,
    /// Play state (actual gameplay)
    Play,
}

/// Protocol version constants
pub const PROTOCOL_VERSION: i32 = 770; // Minecraft 1.21.5
pub const SERVER_VERSION: &str = "1.21.5";
pub const SERVER_DESCRIPTION: &str = "A high-performance Minecraft server implementation written in Rust.";
