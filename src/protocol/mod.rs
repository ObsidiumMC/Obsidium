//! Minecraft protocol implementation
//!
//! This module handles the Minecraft network protocol including packet parsing,
//! handshaking, status, and play state management.

pub mod handshake;
pub mod login;
pub mod packet;
pub mod play;
pub mod status;
pub mod varint;

pub use handshake::HandshakePacket;
pub use login::{LoginStart, LoginSuccess};
pub use packet::Packet;
pub use play::{ClientInformation, CommandSuggestionsRequest, JoinGame, PluginMessage};
pub use status::{PingRequest, PongResponse, StatusRequest, StatusResponse};

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
pub const SERVER_DESCRIPTION: &str =
    "A high-performance Minecraft server implementation written in Rust.";
