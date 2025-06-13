//! Status state packets
//!
//! Status packets are used for server list ping functionality.

use crate::error::Result;
use crate::protocol::packets::{ClientboundPacket, Packet, ServerboundPacket};
use crate::protocol::types::McString;
use std::io::{Read, Write};

/// Status request packet (serverbound)
#[derive(Debug, Clone)]
pub struct StatusRequestPacket;

impl Packet for StatusRequestPacket {
    const ID: i32 = 0x00;

    fn read<R: Read>(_reader: &mut R) -> Result<Self> {
        Ok(StatusRequestPacket)
    }

    fn write<W: Write>(&self, _writer: &mut W) -> Result<()> {
        Ok(())
    }
}

impl ServerboundPacket for StatusRequestPacket {}

/// Status response packet (clientbound)
#[derive(Debug, Clone)]
pub struct StatusResponsePacket {
    /// JSON response string
    pub json_response: McString,
}

impl Packet for StatusResponsePacket {
    const ID: i32 = 0x00;

    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let json_response = McString::read(reader)?;
        Ok(StatusResponsePacket { json_response })
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.json_response.write(writer)?;
        Ok(())
    }
}

impl ClientboundPacket for StatusResponsePacket {}

/// Ping request packet (serverbound)
#[derive(Debug, Clone)]
pub struct PingRequestPacket {
    /// Payload to echo back
    pub payload: i64,
}

impl Packet for PingRequestPacket {
    const ID: i32 = 0x01;

    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let mut bytes = [0u8; 8];
        reader.read_exact(&mut bytes)?;
        let payload = i64::from_be_bytes(bytes);
        Ok(PingRequestPacket { payload })
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.payload.to_be_bytes())?;
        Ok(())
    }
}

impl ServerboundPacket for PingRequestPacket {}

/// Ping response packet (clientbound)
#[derive(Debug, Clone)]
pub struct PingResponsePacket {
    /// Payload echoed back
    pub payload: i64,
}

impl Packet for PingResponsePacket {
    const ID: i32 = 0x01;

    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let mut bytes = [0u8; 8];
        reader.read_exact(&mut bytes)?;
        let payload = i64::from_be_bytes(bytes);
        Ok(PingResponsePacket { payload })
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.payload.to_be_bytes())?;
        Ok(())
    }
}

impl ClientboundPacket for PingResponsePacket {}

/// Server status JSON structure
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServerStatus {
    /// Version information
    pub version: VersionInfo,
    /// Player information
    pub players: PlayersInfo,
    /// Server description
    pub description: Description,
    /// Optional server favicon (base64 encoded PNG)
    pub favicon: Option<String>,
    /// Whether the server enforces secure chat
    #[serde(rename = "enforcesSecureChat")]
    pub enforces_secure_chat: bool,
}

/// Version information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VersionInfo {
    /// Version name (e.g., "1.21.5")
    pub name: String,
    /// Protocol version number
    pub protocol: i32,
}

/// Player information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlayersInfo {
    /// Maximum number of players
    pub max: u32,
    /// Current number of online players
    pub online: u32,
    /// Sample of online players (optional)
    pub sample: Option<Vec<PlayerSample>>,
}

/// Sample player information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlayerSample {
    /// Player name
    pub name: String,
    /// Player UUID
    pub id: String,
}

/// Server description
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Description {
    /// Simple text description
    Text(String),
    /// Rich text description with formatting
    Rich(serde_json::Value),
}

impl ServerStatus {
    /// Convert to JSON string
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(self).map_err(|e| {
            crate::error::ServerError::Protocol(format!("JSON serialization error: {}", e))
        })
    }

    /// Create from JSON string
    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(|e| {
            crate::error::ServerError::Protocol(format!("JSON deserialization error: {}", e))
        })
    }
}
