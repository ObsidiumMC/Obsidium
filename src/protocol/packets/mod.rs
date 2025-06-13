//! Minecraft protocol packets
//!
//! This module contains all packet definitions organized by protocol state.
//! Each state has its own submodule with clientbound and serverbound packets.

pub mod configuration;
pub mod handshaking;
pub mod login;
pub mod play;
pub mod status;

use crate::error::Result;
use crate::protocol::types::VarInt;
use std::io::{Read, Write};

/// Trait for all Minecraft packets
pub trait Packet: Sized {
    /// The packet ID
    const ID: i32;

    /// Read packet data from a reader
    fn read<R: Read>(reader: &mut R) -> Result<Self>;

    /// Write packet data to a writer
    fn write<W: Write>(&self, writer: &mut W) -> Result<()>;

    /// Get the packet ID as a VarInt
    fn id() -> VarInt {
        VarInt(Self::ID)
    }
}

/// Trait for clientbound packets (server -> client)
pub trait ClientboundPacket: Packet {}

/// Trait for serverbound packets (client -> server)
pub trait ServerboundPacket: Packet {}

/// Helper function to read packet length
pub fn read_packet_length<R: Read>(reader: &mut R) -> Result<VarInt> {
    VarInt::read(reader)
}

/// Helper function to write packet with length prefix
pub fn write_packet_with_length<W: Write, P: Packet>(packet: &P, writer: &mut W) -> Result<()> {
    // First, serialize the packet to get its length
    let mut packet_data = Vec::new();
    P::id().write(&mut packet_data)?;
    packet.write(&mut packet_data)?;

    // Write length prefix
    VarInt(packet_data.len() as i32).write(writer)?;

    // Write packet data
    writer.write_all(&packet_data)?;

    Ok(())
}
