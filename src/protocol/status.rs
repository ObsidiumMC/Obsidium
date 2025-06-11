//! Status packets implementation
//! 
//! Handles server status requests and ping/pong functionality.

use crate::protocol::{
    packet::{Packet, PacketReader, PacketWriter}, 
    varint::VarInt,
    PROTOCOL_VERSION, SERVER_VERSION, SERVER_DESCRIPTION
};
use std::io::{Read, Write, Result as IoResult};

/// Status request packet (empty packet)
#[derive(Debug, Clone)]
pub struct StatusRequest;

impl Packet for StatusRequest {
    fn packet_id() -> i32 {
        0x00
    }
    
    fn write_data<W: Write>(&self, _writer: &mut W) -> IoResult<()> {
        // Status request has no data
        Ok(())
    }
    
    fn read_data<R: Read>(_reader: &mut R) -> IoResult<Self> {
        // Status request has no data
        Ok(StatusRequest)
    }
}

/// Status response packet containing server information
#[derive(Debug, Clone)]
pub struct StatusResponse {
    /// JSON response containing server information
    pub json_response: String,
}

impl StatusResponse {
    /// Create a new status response with default server information
    pub fn new() -> Self {
        let json = format!(
            r#"{{
                "version": {{
                    "name": "{SERVER_VERSION}",
                    "protocol": {PROTOCOL_VERSION}
                }},
                "players": {{
                    "max": 20,
                    "online": 0,
                    "sample": []
                }},
                "description": {{
                    "text": "{SERVER_DESCRIPTION}"
                }},
                "favicon": "",
                "enforcesSecureChat": false,
                "previewsChat": false
            }}"#
        );
        
        Self {
            json_response: json,
        }
    }
    
    /// Create a status response with custom JSON
    pub fn with_json(json: String) -> Self {
        Self {
            json_response: json,
        }
    }
}

impl Default for StatusResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl Packet for StatusResponse {
    fn packet_id() -> i32 {
        0x00
    }
    
    fn write_data<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        let mut packet_writer = PacketWriter::new(writer);
        packet_writer.write_string(&self.json_response)?;
        Ok(())
    }
    
    fn read_data<R: Read>(reader: &mut R) -> IoResult<Self> {
        let mut packet_reader = PacketReader::new(reader);
        let json_response = packet_reader.read_string()?;
        Ok(Self { json_response })
    }
}

/// Ping request packet
#[derive(Debug, Clone)]
pub struct PingRequest {
    /// Timestamp payload
    pub payload: i64,
}

impl PingRequest {
    /// Create a new ping request
    pub fn new(payload: i64) -> Self {
        Self { payload }
    }
}

impl Packet for PingRequest {
    fn packet_id() -> i32 {
        0x01
    }
    
    fn write_data<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        let mut packet_writer = PacketWriter::new(writer);
        packet_writer.write_long(self.payload)?;
        Ok(())
    }
    
    fn read_data<R: Read>(reader: &mut R) -> IoResult<Self> {
        let mut packet_reader = PacketReader::new(reader);
        let payload = packet_reader.read_long()?;
        Ok(Self::new(payload))
    }
}

/// Pong response packet
#[derive(Debug, Clone)]
pub struct PongResponse {
    /// Timestamp payload (should match the ping request)
    pub payload: i64,
}

impl PongResponse {
    /// Create a new pong response
    pub fn new(payload: i64) -> Self {
        Self { payload }
    }
    
    /// Create a pong response from a ping request
    pub fn from_ping(ping: &PingRequest) -> Self {
        Self::new(ping.payload)
    }
}

impl Packet for PongResponse {
    fn packet_id() -> i32 {
        0x01
    }
    
    fn write_data<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        let mut packet_writer = PacketWriter::new(writer);
        packet_writer.write_long(self.payload)?;
        Ok(())
    }
    
    fn read_data<R: Read>(reader: &mut R) -> IoResult<Self> {
        let mut packet_reader = PacketReader::new(reader);
        let payload = packet_reader.read_long()?;
        Ok(Self::new(payload))
    }
}
