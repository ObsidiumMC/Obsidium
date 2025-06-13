//! Play state packets
//!
//! Play packets handle the main gameplay functionality.
//! This is where the bulk of the game packets are defined.

use crate::error::Result;
use crate::protocol::packets::{ClientboundPacket, Packet, ServerboundPacket};
use crate::protocol::types::{McString, Position, VarInt};
use std::io::{Read, Write};

/// Keep alive packet (bidirectional)
#[derive(Debug, Clone)]
pub struct KeepAlivePacket {
    /// Keep alive ID
    pub keep_alive_id: i64,
}

impl Packet for KeepAlivePacket {
    const ID: i32 = 0x26; // Clientbound ID, serverbound is 0x18

    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let mut bytes = [0u8; 8];
        reader.read_exact(&mut bytes)?;
        let keep_alive_id = i64::from_be_bytes(bytes);
        Ok(KeepAlivePacket { keep_alive_id })
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.keep_alive_id.to_be_bytes())?;
        Ok(())
    }
}

impl ClientboundPacket for KeepAlivePacket {}
impl ServerboundPacket for KeepAlivePacket {}

/// Disconnect packet (clientbound)
#[derive(Debug, Clone)]
pub struct DisconnectPacket {
    /// Disconnect reason (JSON chat component)
    pub reason: McString,
}

impl Packet for DisconnectPacket {
    const ID: i32 = 0x1D;

    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let reason = McString::read(reader)?;
        Ok(DisconnectPacket { reason })
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.reason.write(writer)?;
        Ok(())
    }
}

impl ClientboundPacket for DisconnectPacket {}

/// Chat message packet (serverbound)
#[derive(Debug, Clone)]
pub struct ChatMessagePacket {
    /// Message content
    pub message: McString,
    /// Timestamp
    pub timestamp: i64,
    /// Salt for message signing
    pub salt: i64,
    /// Optional signature
    pub signature: Option<Vec<u8>>,
    /// Message count
    pub message_count: VarInt,
    /// Acknowledged messages
    pub acknowledged: Vec<u8>,
}

impl Packet for ChatMessagePacket {
    const ID: i32 = 0x06;

    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let message = McString::read(reader)?;

        let mut timestamp_bytes = [0u8; 8];
        reader.read_exact(&mut timestamp_bytes)?;
        let timestamp = i64::from_be_bytes(timestamp_bytes);

        let mut salt_bytes = [0u8; 8];
        reader.read_exact(&mut salt_bytes)?;
        let salt = i64::from_be_bytes(salt_bytes);

        let has_signature = crate::protocol::types::read_bool(reader)?;
        let signature = if has_signature {
            let signature_length = VarInt::read(reader)?;
            let mut signature_bytes = vec![0u8; signature_length.0 as usize];
            reader.read_exact(&mut signature_bytes)?;
            Some(signature_bytes)
        } else {
            None
        };

        let message_count = VarInt::read(reader)?;

        let acknowledged_length = VarInt::read(reader)?;
        let mut acknowledged = vec![0u8; acknowledged_length.0 as usize];
        reader.read_exact(&mut acknowledged)?;

        Ok(ChatMessagePacket {
            message,
            timestamp,
            salt,
            signature,
            message_count,
            acknowledged,
        })
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.message.write(writer)?;
        writer.write_all(&self.timestamp.to_be_bytes())?;
        writer.write_all(&self.salt.to_be_bytes())?;

        crate::protocol::types::write_bool(self.signature.is_some(), writer)?;
        if let Some(ref signature) = self.signature {
            VarInt(signature.len() as i32).write(writer)?;
            writer.write_all(signature)?;
        }

        self.message_count.write(writer)?;
        VarInt(self.acknowledged.len() as i32).write(writer)?;
        writer.write_all(&self.acknowledged)?;

        Ok(())
    }
}

impl ServerboundPacket for ChatMessagePacket {}

/// Player position packet (serverbound)
#[derive(Debug, Clone)]
pub struct PlayerPositionPacket {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
    /// Z coordinate
    pub z: f64,
    /// Whether the player is on ground
    pub on_ground: bool,
}

impl Packet for PlayerPositionPacket {
    const ID: i32 = 0x1A;

    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let mut x_bytes = [0u8; 8];
        reader.read_exact(&mut x_bytes)?;
        let x = f64::from_be_bytes(x_bytes);

        let mut y_bytes = [0u8; 8];
        reader.read_exact(&mut y_bytes)?;
        let y = f64::from_be_bytes(y_bytes);

        let mut z_bytes = [0u8; 8];
        reader.read_exact(&mut z_bytes)?;
        let z = f64::from_be_bytes(z_bytes);

        let on_ground = crate::protocol::types::read_bool(reader)?;

        Ok(PlayerPositionPacket { x, y, z, on_ground })
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.x.to_be_bytes())?;
        writer.write_all(&self.y.to_be_bytes())?;
        writer.write_all(&self.z.to_be_bytes())?;
        crate::protocol::types::write_bool(self.on_ground, writer)?;
        Ok(())
    }
}

impl ServerboundPacket for PlayerPositionPacket {}

/// Block change packet (clientbound)
#[derive(Debug, Clone)]
pub struct BlockChangePacket {
    /// Block position
    pub position: Position,
    /// New block state ID
    pub block_id: VarInt,
}

impl Packet for BlockChangePacket {
    const ID: i32 = 0x09;

    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let position = Position::read(reader)?;
        let block_id = VarInt::read(reader)?;
        Ok(BlockChangePacket { position, block_id })
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.position.write(writer)?;
        self.block_id.write(writer)?;
        Ok(())
    }
}

impl ClientboundPacket for BlockChangePacket {}

// TODO: Add more play packets as needed
// - Chunk data packets
// - Entity packets
// - Inventory packets
// - etc.
