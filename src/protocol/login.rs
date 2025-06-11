//! Login protocol implementation
//!
//! This module handles login-related packets for the Minecraft protocol.

use crate::protocol::packet::Packet;
use crate::protocol::varint::VarInt;
use std::io::{Read, Result as IoResult, Write};
use uuid::Uuid;

/// Login Start packet (serverbound)
/// Sent by client to initiate login process
#[derive(Debug, Clone)]
pub struct LoginStart {
    pub name: String,
    pub player_uuid: Uuid,
}

impl LoginStart {
    pub fn new(name: String, player_uuid: Uuid) -> Self {
        Self { name, player_uuid }
    }

    /// Parse from packet data (excluding length and packet ID)
    pub fn from_packet_data(data: &[u8]) -> IoResult<Self> {
        let mut cursor = std::io::Cursor::new(data);
        Self::read_data(&mut cursor)
    }
}

impl Packet for LoginStart {
    fn packet_id() -> i32 {
        0x00
    }

    fn write_data<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        // Write player name (String)
        let name_bytes = self.name.as_bytes();
        VarInt::from(name_bytes.len() as i32).write(writer)?;
        writer.write_all(name_bytes)?;
        
        // Write player UUID (16 bytes)
        writer.write_all(self.player_uuid.as_bytes())?;
        
        Ok(())
    }

    fn read_data<R: Read>(reader: &mut R) -> IoResult<Self> {
        // Read player name
        let name_length = VarInt::read(reader)?.0 as usize;
        let mut name_bytes = vec![0u8; name_length];
        reader.read_exact(&mut name_bytes)?;
        let name = String::from_utf8(name_bytes).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
        })?;
        
        // Read player UUID
        let mut uuid_bytes = [0u8; 16];
        reader.read_exact(&mut uuid_bytes)?;
        let player_uuid = Uuid::from_bytes(uuid_bytes);
        
        Ok(LoginStart::new(name, player_uuid))
    }
}

/// Login Success packet (clientbound)
/// Sent by server after successful authentication
#[derive(Debug, Clone)]
pub struct LoginSuccess {
    pub uuid: Uuid,
    pub username: String,
    pub properties: Vec<Property>,
}

#[derive(Debug, Clone)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

impl LoginSuccess {
    pub fn new(uuid: Uuid, username: String) -> Self {
        Self {
            uuid,
            username,
            properties: Vec::new(),
        }
    }

    /// Parse from packet data (excluding length and packet ID)
    pub fn from_packet_data(data: &[u8]) -> IoResult<Self> {
        let mut cursor = std::io::Cursor::new(data);
        Self::read_data(&mut cursor)
    }
}

impl Packet for LoginSuccess {
    fn packet_id() -> i32 {
        0x02
    }

    fn write_data<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        // Write UUID (16 bytes)
        writer.write_all(self.uuid.as_bytes())?;
        
        // Write username (String)
        let username_bytes = self.username.as_bytes();
        VarInt::from(username_bytes.len() as i32).write(writer)?;
        writer.write_all(username_bytes)?;
        
        // Write properties array length
        VarInt::from(self.properties.len() as i32).write(writer)?;
        
        // Write each property
        for property in &self.properties {
            // Property name
            let name_bytes = property.name.as_bytes();
            VarInt::from(name_bytes.len() as i32).write(writer)?;
            writer.write_all(name_bytes)?;
            
            // Property value
            let value_bytes = property.value.as_bytes();
            VarInt::from(value_bytes.len() as i32).write(writer)?;
            writer.write_all(value_bytes)?;
            
            // Property signature (optional)
            if let Some(signature) = &property.signature {
                writer.write_all(&[1])?; // Has signature
                let sig_bytes = signature.as_bytes();
                VarInt::from(sig_bytes.len() as i32).write(writer)?;
                writer.write_all(sig_bytes)?;
            } else {
                writer.write_all(&[0])?; // No signature
            }
        }
        
        Ok(())
    }

    fn read_data<R: Read>(reader: &mut R) -> IoResult<Self> {
        // Read UUID
        let mut uuid_bytes = [0u8; 16];
        reader.read_exact(&mut uuid_bytes)?;
        let uuid = Uuid::from_bytes(uuid_bytes);
        
        // Read username
        let username_length = VarInt::read(reader)?.0 as usize;
        let mut username_bytes = vec![0u8; username_length];
        reader.read_exact(&mut username_bytes)?;
        let username = String::from_utf8(username_bytes).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
        })?;
        
        // Read properties
        let properties_count = VarInt::read(reader)?.0 as usize;
        let mut properties = Vec::with_capacity(properties_count);
        
        for _ in 0..properties_count {
            // Read property name
            let name_length = VarInt::read(reader)?.0 as usize;
            let mut name_bytes = vec![0u8; name_length];
            reader.read_exact(&mut name_bytes)?;
            let name = String::from_utf8(name_bytes).map_err(|e| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
            })?;
            
            // Read property value
            let value_length = VarInt::read(reader)?.0 as usize;
            let mut value_bytes = vec![0u8; value_length];
            reader.read_exact(&mut value_bytes)?;
            let value = String::from_utf8(value_bytes).map_err(|e| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
            })?;
            
            // Read signature (optional)
            let mut has_signature = [0u8; 1];
            reader.read_exact(&mut has_signature)?;
            let signature = if has_signature[0] == 1 {
                let sig_length = VarInt::read(reader)?.0 as usize;
                let mut sig_bytes = vec![0u8; sig_length];
                reader.read_exact(&mut sig_bytes)?;
                Some(String::from_utf8(sig_bytes).map_err(|e| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
                })?)
            } else {
                None
            };
            
            properties.push(Property {
                name,
                value,
                signature,
            });
        }
        
        Ok(LoginSuccess {
            uuid,
            username,
            properties,
        })
    }
}
