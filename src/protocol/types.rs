//! Minecraft protocol data types
//!
//! This module implements all the data types used in the Minecraft protocol,
//! including VarInt, VarLong, String, and other composite types.

use crate::error::{Result, ServerError};
use serde_json::Value as JsonValue;
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

    /// Read a string with a specific maximum length
    pub fn read_with_max_length<R: Read>(reader: &mut R, max_length: usize) -> Result<Self> {
        let length = VarInt::read(reader)?;

        if length.0 < 0 {
            return Err(ServerError::Protocol("Negative string length".to_string()));
        }

        let length = length.0 as usize;
        if length > max_length {
            return Err(ServerError::Protocol(format!(
                "String too long: {} > {}",
                length, max_length
            )));
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

    /// Validate string length against a maximum
    pub fn validate_length(&self, max_length: usize) -> Result<()> {
        if self.0.len() > max_length {
            return Err(ServerError::Protocol(format!(
                "String too long: {} > {}",
                self.0.len(),
                max_length
            )));
        }
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

/// Read an unsigned short (u16) from a reader
pub fn read_unsigned_short<R: Read>(reader: &mut R) -> Result<u16> {
    let mut bytes = [0u8; 2];
    reader.read_exact(&mut bytes)?;
    Ok(u16::from_be_bytes(bytes))
}

/// Write an unsigned short (u16) to a writer
pub fn write_unsigned_short<W: Write>(value: u16, writer: &mut W) -> Result<()> {
    writer.write_all(&value.to_be_bytes())?;
    Ok(())
}

/// Read an unsigned byte (u8) from a reader
pub fn read_unsigned_byte<R: Read>(reader: &mut R) -> Result<u8> {
    let mut byte = [0u8; 1];
    reader.read_exact(&mut byte)?;
    Ok(byte[0])
}

/// Write an unsigned byte (u8) to a writer
pub fn write_unsigned_byte<W: Write>(value: u8, writer: &mut W) -> Result<()> {
    writer.write_all(&[value])?;
    Ok(())
}

/// Read a long (i64) from a reader
pub fn read_long<R: Read>(reader: &mut R) -> Result<i64> {
    let mut bytes = [0u8; 8];
    reader.read_exact(&mut bytes)?;
    Ok(i64::from_be_bytes(bytes))
}

/// Write a long (i64) to a writer
pub fn write_long<W: Write>(value: i64, writer: &mut W) -> Result<()> {
    writer.write_all(&value.to_be_bytes())?;
    Ok(())
}

/// Read an int (i32) from a reader
pub fn read_int<R: Read>(reader: &mut R) -> Result<i32> {
    let mut bytes = [0u8; 4];
    reader.read_exact(&mut bytes)?;
    Ok(i32::from_be_bytes(bytes))
}

/// Write an int (i32) to a writer
pub fn write_int<W: Write>(value: i32, writer: &mut W) -> Result<()> {
    writer.write_all(&value.to_be_bytes())?;
    Ok(())
}

/// A byte array with VarInt length prefix
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ByteArray(pub Vec<u8>);

impl ByteArray {
    /// Read a byte array from a reader
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let length = VarInt::read(reader)?;

        if length.0 < 0 {
            return Err(ServerError::Protocol(
                "Negative byte array length".to_string(),
            ));
        }

        let mut bytes = vec![0u8; length.0 as usize];
        reader.read_exact(&mut bytes)?;
        Ok(ByteArray(bytes))
    }

    /// Write a byte array to a writer
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        VarInt(self.0.len() as i32).write(writer)?;
        writer.write_all(&self.0)?;
        Ok(())
    }

    /// Read a byte array with a maximum length
    pub fn read_with_max_length<R: Read>(reader: &mut R, max_length: usize) -> Result<Self> {
        let length = VarInt::read(reader)?;

        if length.0 < 0 {
            return Err(ServerError::Protocol(
                "Negative byte array length".to_string(),
            ));
        }

        if length.0 as usize > max_length {
            return Err(ServerError::Protocol(format!(
                "Byte array too long: {} > {}",
                length.0, max_length
            )));
        }

        let mut bytes = vec![0u8; length.0 as usize];
        reader.read_exact(&mut bytes)?;
        Ok(ByteArray(bytes))
    }
}

impl From<Vec<u8>> for ByteArray {
    fn from(bytes: Vec<u8>) -> Self {
        ByteArray(bytes)
    }
}

impl From<ByteArray> for Vec<u8> {
    fn from(array: ByteArray) -> Self {
        array.0
    }
}

/// A JSON text component
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonTextComponent(pub String);

impl JsonTextComponent {
    /// Read a JSON text component from a reader
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let json_string = McString::read(reader)?;

        // Validate that it's valid JSON
        serde_json::from_str::<JsonValue>(&json_string.0)
            .map_err(|_| ServerError::Protocol("Invalid JSON text component".to_string()))?;

        Ok(JsonTextComponent(json_string.0))
    }

    /// Write a JSON text component to a writer
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        let mc_string = McString(self.0.clone());
        mc_string.write(writer)
    }

    /// Create a simple text component
    pub fn text(text: &str) -> Self {
        let json = serde_json::json!({
            "text": text
        });
        JsonTextComponent(json.to_string())
    }
}

impl From<String> for JsonTextComponent {
    fn from(text: String) -> Self {
        JsonTextComponent::text(&text)
    }
}

impl From<&str> for JsonTextComponent {
    fn from(text: &str) -> Self {
        JsonTextComponent::text(text)
    }
}

/// An identifier (namespaced string)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier(pub String);

impl Identifier {
    /// Read an identifier from a reader
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let string = McString::read(reader)?;
        Ok(Identifier(string.0))
    }

    /// Write an identifier to a writer
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        let mc_string = McString(self.0.clone());
        mc_string.write(writer)
    }

    /// Create a new identifier with namespace and path
    pub fn new(namespace: &str, path: &str) -> Self {
        Identifier(format!("{}:{}", namespace, path))
    }

    /// Get the namespace part
    pub fn namespace(&self) -> &str {
        self.0.split(':').next().unwrap_or("")
    }

    /// Get the path part
    pub fn path(&self) -> &str {
        self.0.split(':').nth(1).unwrap_or(&self.0)
    }
}

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Identifier(value)
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Identifier(value.to_string())
    }
}

/// An optional value that may or may not be present
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Optional<T> {
    /// The optional value
    pub value: Option<T>,
}

impl<T> Optional<T> {
    /// Create a new optional value
    pub fn some(value: T) -> Self {
        Optional { value: Some(value) }
    }

    /// Create an empty optional value
    pub fn none() -> Self {
        Optional { value: None }
    }

    /// Check if the optional has a value
    pub fn is_present(&self) -> bool {
        self.value.is_some()
    }
}

impl<T> From<Option<T>> for Optional<T> {
    fn from(option: Option<T>) -> Self {
        Optional { value: option }
    }
}

impl<T> From<Optional<T>> for Option<T> {
    fn from(optional: Optional<T>) -> Self {
        optional.value
    }
}

/// Specialized string types with length limits as defined in the protocol
/// Server address string (max 255 characters)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerAddress(pub String);

impl ServerAddress {
    /// Maximum length for server address strings
    pub const MAX_LENGTH: usize = 255;

    /// Read a server address from a reader
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let string = McString::read_with_max_length(reader, Self::MAX_LENGTH)?;
        Ok(ServerAddress(string.0))
    }

    /// Write a server address to a writer
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        let mc_string = McString(self.0.clone());
        mc_string.validate_length(Self::MAX_LENGTH)?;
        mc_string.write(writer)
    }
}

impl From<String> for ServerAddress {
    fn from(value: String) -> Self {
        ServerAddress(value)
    }
}

impl From<&str> for ServerAddress {
    fn from(value: &str) -> Self {
        ServerAddress(value.to_string())
    }
}

/// Username string (max 16 characters)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Username(pub String);

impl Username {
    /// Maximum length for username strings
    pub const MAX_LENGTH: usize = 16;

    /// Read a username from a reader
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let string = McString::read_with_max_length(reader, Self::MAX_LENGTH)?;
        Ok(Username(string.0))
    }

    /// Write a username to a writer
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        let mc_string = McString(self.0.clone());
        mc_string.validate_length(Self::MAX_LENGTH)?;
        mc_string.write(writer)
    }
}

impl From<String> for Username {
    fn from(value: String) -> Self {
        Username(value)
    }
}

impl From<&str> for Username {
    fn from(value: &str) -> Self {
        Username(value.to_string())
    }
}

/// Server ID string (max 20 characters)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerId(pub String);

impl ServerId {
    /// Maximum length for server ID strings
    pub const MAX_LENGTH: usize = 20;

    /// Read a server ID from a reader
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let string = McString::read_with_max_length(reader, Self::MAX_LENGTH)?;
        Ok(ServerId(string.0))
    }

    /// Write a server ID to a writer
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        let mc_string = McString(self.0.clone());
        mc_string.validate_length(Self::MAX_LENGTH)?;
        mc_string.write(writer)
    }
}

impl From<String> for ServerId {
    fn from(value: String) -> Self {
        ServerId(value)
    }
}

impl From<&str> for ServerId {
    fn from(value: &str) -> Self {
        ServerId(value.to_string())
    }
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

    #[test]
    fn test_byte_array_roundtrip() {
        let test_data = vec![1, 2, 3, 4, 5, 255, 0];
        let byte_array = ByteArray::from(test_data.clone());

        let mut buffer = Vec::new();
        byte_array.write(&mut buffer).unwrap();

        let mut cursor = Cursor::new(buffer);
        let decoded = ByteArray::read(&mut cursor).unwrap();

        assert_eq!(byte_array, decoded);
        assert_eq!(test_data, decoded.0);
    }

    #[test]
    fn test_json_text_component() {
        let component = JsonTextComponent::text("Hello, world!");

        let mut buffer = Vec::new();
        component.write(&mut buffer).unwrap();

        let mut cursor = Cursor::new(buffer);
        let decoded = JsonTextComponent::read(&mut cursor).unwrap();

        assert_eq!(component, decoded);
    }

    #[test]
    fn test_identifier() {
        let identifier = Identifier::new("minecraft", "stone");

        let mut buffer = Vec::new();
        identifier.write(&mut buffer).unwrap();

        let mut cursor = Cursor::new(buffer);
        let decoded = Identifier::read(&mut cursor).unwrap();

        assert_eq!(identifier, decoded);
        assert_eq!("minecraft", identifier.namespace());
        assert_eq!("stone", identifier.path());
    }

    #[test]
    fn test_primitive_types() {
        // Test unsigned short
        let value = 12345u16;
        let mut buffer = Vec::new();
        write_unsigned_short(value, &mut buffer).unwrap();
        let mut cursor = Cursor::new(buffer);
        let decoded = read_unsigned_short(&mut cursor).unwrap();
        assert_eq!(value, decoded);

        // Test long
        let value = 1234567890123456789i64;
        let mut buffer = Vec::new();
        write_long(value, &mut buffer).unwrap();
        let mut cursor = Cursor::new(buffer);
        let decoded = read_long(&mut cursor).unwrap();
        assert_eq!(value, decoded);

        // Test int
        let value = -123456789i32;
        let mut buffer = Vec::new();
        write_int(value, &mut buffer).unwrap();
        let mut cursor = Cursor::new(buffer);
        let decoded = read_int(&mut cursor).unwrap();
        assert_eq!(value, decoded);
    }
}
