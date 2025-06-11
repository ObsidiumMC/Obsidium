//! Packet handling for Minecraft protocol
//!
//! This module provides utilities for reading and writing Minecraft protocol packets.

use crate::protocol::varint::VarInt;
use std::io::{Cursor, Read, Result as IoResult, Write};

/// A Minecraft protocol packet
pub trait Packet {
    /// Get the packet ID
    fn packet_id() -> i32;

    /// Write packet data to a writer (without length prefix)
    fn write_data<W: Write>(&self, writer: &mut W) -> IoResult<()>;

    /// Read packet data from a reader
    fn read_data<R: Read>(reader: &mut R) -> IoResult<Self>
    where
        Self: Sized;

    /// Write the complete packet (with length prefix) to a writer
    fn write_packet<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        // First, write packet data to a buffer to calculate length
        let mut data_buffer = Vec::new();
        VarInt::from(Self::packet_id()).write(&mut data_buffer)?;
        self.write_data(&mut data_buffer)?;

        // Write length prefix and then data
        VarInt::from(data_buffer.len() as i32).write(writer)?;
        writer.write_all(&data_buffer)?;

        Ok(())
    }
}

/// Utility for reading packet data
pub struct PacketReader<R: Read> {
    reader: R,
}

impl<R: Read> PacketReader<R> {
    /// Create a new packet reader
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    /// Read the next packet length
    pub fn read_packet_length(&mut self) -> IoResult<usize> {
        let length = VarInt::read(&mut self.reader)?;
        Ok(length.0 as usize)
    }

    /// Read packet data of specified length
    pub fn read_packet_data(&mut self, length: usize) -> IoResult<Vec<u8>> {
        let mut buffer = vec![0u8; length];
        self.reader.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    /// Read a VarInt
    pub fn read_varint(&mut self) -> IoResult<VarInt> {
        VarInt::read(&mut self.reader)
    }

    /// Read a string (length-prefixed UTF-8)
    pub fn read_string(&mut self) -> IoResult<String> {
        let length = VarInt::read(&mut self.reader)?.0 as usize;
        let mut buffer = vec![0u8; length];
        self.reader.read_exact(&mut buffer)?;
        String::from_utf8(buffer)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    /// Read a u16
    pub fn read_u16(&mut self) -> IoResult<u16> {
        let mut buffer = [0u8; 2];
        self.reader.read_exact(&mut buffer)?;
        Ok(u16::from_be_bytes(buffer))
    }

    /// Read a u8
    pub fn read_u8(&mut self) -> IoResult<u8> {
        let mut buffer = [0u8; 1];
        self.reader.read_exact(&mut buffer)?;
        Ok(buffer[0])
    }

    /// Read a long (i64)
    pub fn read_long(&mut self) -> IoResult<i64> {
        let mut buffer = [0u8; 8];
        self.reader.read_exact(&mut buffer)?;
        Ok(i64::from_be_bytes(buffer))
    }
}

/// Utility for writing packet data
pub struct PacketWriter<W: Write> {
    writer: W,
}

impl<W: Write> PacketWriter<W> {
    /// Create a new packet writer
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    /// Write a VarInt
    pub fn write_varint(&mut self, varint: VarInt) -> IoResult<()> {
        varint.write(&mut self.writer)
    }

    /// Write a string (length-prefixed UTF-8)
    pub fn write_string(&mut self, s: &str) -> IoResult<()> {
        let bytes = s.as_bytes();
        VarInt::from(bytes.len()).write(&mut self.writer)?;
        self.writer.write_all(bytes)?;
        Ok(())
    }

    /// Write a u16
    pub fn write_u16(&mut self, value: u16) -> IoResult<()> {
        self.writer.write_all(&value.to_be_bytes())
    }

    /// Write a u8
    pub fn write_u8(&mut self, value: u8) -> IoResult<()> {
        self.writer.write_all(&[value])
    }

    /// Write a long (i64)
    pub fn write_long(&mut self, value: i64) -> IoResult<()> {
        self.writer.write_all(&value.to_be_bytes())
    }

    /// Get the inner writer
    pub fn into_inner(self) -> W {
        self.writer
    }
}

/// Parse a packet from raw bytes
pub fn parse_packet_data(data: &[u8]) -> IoResult<(i32, Vec<u8>)> {
    let mut cursor = Cursor::new(data);
    let packet_id = VarInt::read(&mut cursor)?.0;

    let remaining = data.len() - cursor.position() as usize;
    let mut packet_data = vec![0u8; remaining];
    cursor.read_exact(&mut packet_data)?;

    Ok((packet_id, packet_data))
}
