//! Minecraft protocol implementation
//!
//! This module implements the complete Minecraft Java Edition protocol
//! for version 1.21.5 (protocol 770). It handles packet serialization,
//! compression, and state management.
//!
//! # Protocol States
//!
//! The Minecraft protocol has four main states:
//! - **Handshaking**: Initial connection state
//! - **Status**: Server list ping
//! - **Login**: Player authentication
//! - **Play**: Active gameplay
//!
//! # Packet Format
//!
//! All packets follow this format:
//! - Length (VarInt) - Length of packet ID + data
//! - Packet ID (VarInt) - Identifies the packet type
//! - Data - Packet-specific data

pub mod compression;
pub mod packets;
pub mod state;
pub mod types;

pub use compression::Compression;
pub use state::{ConnectionState, ProtocolState};
pub use types::{McString, McUuid, Position, VarInt, VarLong};

/// Protocol version constant
pub const PROTOCOL_VERSION: i32 = 770;

/// Maximum packet size (2^21 - 1 bytes)
pub const MAX_PACKET_SIZE: usize = 2_097_151;

/// Maximum uncompressed packet size for serverbound packets (2^23 bytes)
pub const MAX_UNCOMPRESSED_PACKET_SIZE: usize = 8_388_608;

/// Maximum VarInt length field size (3 bytes)
pub const MAX_VARINT_LENGTH_FIELD_SIZE: usize = 3;
