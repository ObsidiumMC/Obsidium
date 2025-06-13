//! Packet codec
//!
//! This module provides utilities for encoding and decoding Minecraft packets.

use crate::error::{Result, ServerError};
use crate::protocol::types::VarInt;
use std::io::Cursor;

/// Packet codec for reading and writing Minecraft packets
pub struct PacketCodec;

impl PacketCodec {
    /// Encode a packet with length prefix
    pub fn encode<P>(packet: &P) -> Result<Vec<u8>>
    where
        P: crate::protocol::packets::Packet,
    {
        let mut packet_data = Vec::new();

        // Write packet ID
        P::id().write(&mut packet_data)?;

        // Write packet data
        packet.write(&mut packet_data)?;

        // Create final buffer with length prefix
        let mut result = Vec::new();
        VarInt(packet_data.len() as i32).write(&mut result)?;
        result.extend_from_slice(&packet_data);

        Ok(result)
    }

    /// Decode a packet from raw bytes
    pub fn decode<P>(data: &[u8]) -> Result<P>
    where
        P: crate::protocol::packets::Packet,
    {
        let mut cursor = Cursor::new(data);

        // Read and verify packet ID
        let packet_id = VarInt::read(&mut cursor)?;
        if packet_id.0 != P::ID {
            return Err(ServerError::Protocol(format!(
                "Packet ID mismatch: expected {}, got {}",
                P::ID,
                packet_id.0
            )));
        }

        // Read packet data
        P::read(&mut cursor)
    }

    /// Get the packet ID from raw packet data
    pub fn get_packet_id(data: &[u8]) -> Result<VarInt> {
        let mut cursor = Cursor::new(data);
        VarInt::read(&mut cursor)
    }

    /// Calculate the total size of an encoded packet
    pub fn calculate_packet_size<P>(packet: &P) -> Result<usize>
    where
        P: crate::protocol::packets::Packet,
    {
        let mut size = 0;

        // Packet ID size
        size += P::id().len();

        // Packet data size (we need to serialize to get accurate size)
        let mut temp_buffer = Vec::new();
        packet.write(&mut temp_buffer)?;
        size += temp_buffer.len();

        // Length prefix size
        let length_prefix = VarInt(size as i32);
        size += length_prefix.len();

        Ok(size)
    }
}
