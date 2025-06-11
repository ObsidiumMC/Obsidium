//! Handshake packet implementation
//!
//! Handles the initial handshake between client and server.

use crate::protocol::{
    ConnectionState,
    packet::{Packet, PacketReader, PacketWriter},
    varint::VarInt,
};
use std::io::{Cursor, Read, Result as IoResult, Write};

/// Handshake packet sent by client to initiate connection
#[derive(Debug, Clone)]
pub struct HandshakePacket {
    /// Protocol version used by the client
    pub protocol_version: i32,
    /// Server address (hostname or IP) used to connect
    pub server_address: String,
    /// Server port used to connect
    pub server_port: u16,
    /// Next connection state (1 for status, 2 for login)
    pub next_state: i32,
}

impl HandshakePacket {
    /// Create a new handshake packet
    pub fn new(
        protocol_version: i32,
        server_address: String,
        server_port: u16,
        next_state: i32,
    ) -> Self {
        Self {
            protocol_version,
            server_address,
            server_port,
            next_state,
        }
    }

    /// Get the connection state from next_state field
    pub fn get_next_state(&self) -> Option<ConnectionState> {
        match self.next_state {
            1 => Some(ConnectionState::Status),
            2 => Some(ConnectionState::Login),
            _ => None,
        }
    }

    /// Parse handshake from raw packet data
    pub fn from_packet_data(data: &[u8]) -> IoResult<Self> {
        let mut cursor = Cursor::new(data);
        let mut reader = PacketReader::new(&mut cursor);

        let protocol_version = reader.read_varint()?.0;
        let server_address = reader.read_string()?;
        let server_port = reader.read_u16()?;
        let next_state = reader.read_varint()?.0;

        Ok(Self::new(
            protocol_version,
            server_address,
            server_port,
            next_state,
        ))
    }
}

impl Packet for HandshakePacket {
    fn packet_id() -> i32 {
        0x00
    }

    fn write_data<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        let mut packet_writer = PacketWriter::new(writer);

        packet_writer.write_varint(VarInt::from(self.protocol_version))?;
        packet_writer.write_string(&self.server_address)?;
        packet_writer.write_u16(self.server_port)?;
        packet_writer.write_varint(VarInt::from(self.next_state))?;

        Ok(())
    }

    fn read_data<R: Read>(reader: &mut R) -> IoResult<Self> {
        let mut packet_reader = PacketReader::new(reader);

        let protocol_version = packet_reader.read_varint()?.0;
        let server_address = packet_reader.read_string()?;
        let server_port = packet_reader.read_u16()?;
        let next_state = packet_reader.read_varint()?.0;

        Ok(Self::new(
            protocol_version,
            server_address,
            server_port,
            next_state,
        ))
    }
}
