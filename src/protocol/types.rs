//! Minecraft protocol data types
//!
//! This module implements all the data types used in the Minecraft protocol,
//! including VarInt, VarLong, String, and other composite types.

use crate::error::{Result, ServerError};
use std::io::{Read, Write};
use uuid::Uuid;

/// A variable-length integer as defined by the Minecraft protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VarInt(pub i32);

impl VarInt {
    /// The maximum number of bytes a VarInt can occupy
    pub const MAX_SIZE: usize = 5;

    /// Read a VarInt from a reader
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let mut value = 0i32;
        let mut position = 0;

        loop {
            let mut byte = [0u8; 1];
            reader.read_exact(&mut byte)?;
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

    /// Write a VarInt to a writer
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        let mut value = self.0 as u32;

        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7;

            if value != 0 {
                byte |= 0x80;
            }

            writer.write_all(&[byte])?;

            if value == 0 {
                break;
            }
        }

        Ok(())
    }

    /// Get the number of bytes this VarInt will occupy when written
    pub fn len(&self) -> usize {
        let mut value = self.0 as u32;
        let mut size = 0;

        loop {
            value >>= 7;
            size += 1;
            if value == 0 {
                break;
            }
        }

        size
    }

    /// Check if the VarInt is empty (represents 0)
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

impl From<i32> for VarInt {
    fn from(value: i32) -> Self {
        VarInt(value)
    }
}

impl From<VarInt> for i32 {
    fn from(varint: VarInt) -> Self {
        varint.0
    }
}

/// A variable-length long integer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VarLong(pub i64);

impl VarLong {
    /// The maximum number of bytes a VarLong can occupy
    pub const MAX_SIZE: usize = 10;

    /// Read a VarLong from a reader
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let mut value = 0i64;
        let mut position = 0;

        loop {
            let mut byte = [0u8; 1];
            reader.read_exact(&mut byte)?;
            let byte = byte[0];

            value |= ((byte & 0x7F) as i64) << position;

            if (byte & 0x80) == 0 {
                break;
            }

            position += 7;
            if position >= 64 {
                return Err(ServerError::Protocol("VarLong too long".to_string()));
            }
        }

        Ok(VarLong(value))
    }

    /// Write a VarLong to a writer
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        let mut value = self.0 as u64;

        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7;

            if value != 0 {
                byte |= 0x80;
            }

            writer.write_all(&[byte])?;

            if value == 0 {
                break;
            }
        }

        Ok(())
    }
}

impl From<i64> for VarLong {
    fn from(value: i64) -> Self {
        VarLong(value)
    }
}

impl From<VarLong> for i64 {
    fn from(varlong: VarLong) -> Self {
        varlong.0
    }
}

/// A Minecraft protocol string (UTF-8 with VarInt length prefix)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct McString(pub String);

impl McString {
    /// Maximum string length in characters
    pub const MAX_LENGTH: usize = 32767;

    /// Read a string from a reader
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let length = VarInt::read(reader)?;

        if length.0 < 0 {
            return Err(ServerError::Protocol("Negative string length".to_string()));
        }

        let length = length.0 as usize;
        if length > Self::MAX_LENGTH {
            return Err(ServerError::Protocol("String too long".to_string()));
        }

        let mut bytes = vec![0u8; length];
        reader.read_exact(&mut bytes)?;

        let string = String::from_utf8(bytes)
            .map_err(|_| ServerError::Protocol("Invalid UTF-8 in string".to_string()))?;

        Ok(McString(string))
    }

    /// Write a string to a writer
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        let bytes = self.0.as_bytes();

        if bytes.len() > Self::MAX_LENGTH {
            return Err(ServerError::Protocol("String too long".to_string()));
        }

        VarInt(bytes.len() as i32).write(writer)?;
        writer.write_all(bytes)?;

        Ok(())
    }
}

impl From<String> for McString {
    fn from(value: String) -> Self {
        McString(value)
    }
}

impl From<&str> for McString {
    fn from(value: &str) -> Self {
        McString(value.to_string())
    }
}

impl From<McString> for String {
    fn from(mc_string: McString) -> Self {
        mc_string.0
    }
}

/// A Minecraft position (3D coordinates packed into a single i64)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    /// X coordinate
    pub x: i32,
    /// Y coordinate
    pub y: i32,
    /// Z coordinate
    pub z: i32,
}

impl Position {
    /// Create a new position
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// Read a position from a reader
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let mut bytes = [0u8; 8];
        reader.read_exact(&mut bytes)?;
        let val = i64::from_be_bytes(bytes);

        let x = (val >> 38) as i32;
        let y = (val << 52 >> 52) as i32;
        let z = (val << 26 >> 38) as i32;

        Ok(Position::new(x, y, z))
    }

    /// Write a position to a writer
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        let val = ((self.x as i64 & 0x3FFFFFF) << 38)
            | ((self.z as i64 & 0x3FFFFFF) << 12)
            | (self.y as i64 & 0xFFF);

        writer.write_all(&val.to_be_bytes())?;
        Ok(())
    }
}

/// A Minecraft UUID
pub type McUuid = Uuid;

/// Read a UUID from a reader
pub fn read_uuid<R: Read>(reader: &mut R) -> Result<McUuid> {
    let mut bytes = [0u8; 16];
    reader.read_exact(&mut bytes)?;
    Ok(Uuid::from_bytes(bytes))
}

/// Write a UUID to a writer
pub fn write_uuid<W: Write>(uuid: &McUuid, writer: &mut W) -> Result<()> {
    writer.write_all(uuid.as_bytes())?;
    Ok(())
}

/// Read a boolean from a reader
pub fn read_bool<R: Read>(reader: &mut R) -> Result<bool> {
    let mut byte = [0u8; 1];
    reader.read_exact(&mut byte)?;
    Ok(byte[0] != 0)
}

/// Write a boolean to a writer
pub fn write_bool<W: Write>(value: bool, writer: &mut W) -> Result<()> {
    writer.write_all(&[if value { 1 } else { 0 }])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_varint_roundtrip() {
        let values = [0, 1, 127, 128, 255, 25565, -1, -2147483648, 2147483647];

        for &value in &values {
            let varint = VarInt(value);
            let mut buffer = Vec::new();
            varint.write(&mut buffer).unwrap();

            let mut cursor = Cursor::new(buffer);
            let decoded = VarInt::read(&mut cursor).unwrap();

            assert_eq!(varint, decoded);
        }
    }

    #[test]
    fn test_string_roundtrip() {
        let test_strings = ["", "Hello", "Hello, 世界!", "🚀"];

        for &s in &test_strings {
            let mc_string = McString::from(s);
            let mut buffer = Vec::new();
            mc_string.write(&mut buffer).unwrap();

            let mut cursor = Cursor::new(buffer);
            let decoded = McString::read(&mut cursor).unwrap();

            assert_eq!(mc_string, decoded);
        }
    }
}
