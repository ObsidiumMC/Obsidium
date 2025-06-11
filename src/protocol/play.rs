//! Play state protocol implementation
//!
//! Handles packets sent during the Play state of the Minecraft protocol.

use crate::protocol::{varint::VarInt, Packet};
use std::io::{Cursor, Read, Write};

/// Client Information packet (serverbound)
/// Packet ID: 0x00
#[derive(Debug)]
pub struct ClientInformation {
    /// Client locale (e.g., "en_us")
    pub locale: String,
    /// View distance in chunks
    pub view_distance: u8,
    /// Chat mode (0 = enabled, 1 = commands only, 2 = hidden)
    pub chat_mode: VarInt,
    /// Whether chat colors are enabled
    pub chat_colors: bool,
    /// Displayed skin parts bitmask
    pub displayed_skin_parts: u8,
    /// Main hand (0 = left, 1 = right)
    pub main_hand: VarInt,
    /// Whether text filtering is enabled
    pub enable_text_filtering: bool,
    /// Whether to allow server listings
    pub allow_server_listings: bool,
}

impl Packet for ClientInformation {
    fn packet_id() -> i32 {
        0x00
    }

    fn write_data<W: Write>(&self, _writer: &mut W) -> std::io::Result<()> {
        // This is a serverbound packet, so we don't need to implement writing
        Ok(())
    }

    fn read_data<R: Read>(_reader: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        // This is handled by from_packet_data instead
        unimplemented!("Use from_packet_data instead")
    }
}

impl ClientInformation {
    /// Parse client information from packet data
    pub fn from_packet_data(data: &[u8]) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        if data.is_empty() {
            return Err("Empty packet data for Client Information".into());
        }
        
        let mut cursor = Cursor::new(data);
        
        // Read locale string with error handling
        let locale = match VarInt::read(&mut cursor) {
            Ok(locale_length_varint) => {
                let locale_length = locale_length_varint.0 as usize;
                if locale_length == 0 {
                    "en_us".to_string() // Default locale
                } else if locale_length > data.len() {
                    tracing::warn!("Invalid locale length in Client Information packet");
                    "en_us".to_string()
                } else {
                    let mut locale_bytes = vec![0u8; locale_length];
                    match cursor.read_exact(&mut locale_bytes) {
                        Ok(_) => String::from_utf8(locale_bytes).unwrap_or_else(|_| "en_us".to_string()),
                        Err(_) => {
                            tracing::warn!("Failed to read locale in Client Information packet");
                            "en_us".to_string()
                        }
                    }
                }
            },
            Err(_) => {
                tracing::warn!("Failed to read locale length in Client Information packet");
                "en_us".to_string()
            }
        };
        
        // Read view distance with default fallback
        let view_distance = {
            let mut view_distance_bytes = [0u8; 1];
            match cursor.read_exact(&mut view_distance_bytes) {
                Ok(_) => view_distance_bytes[0],
                Err(_) => {
                    tracing::warn!("Failed to read view distance in Client Information packet, using default");
                    10 // Default view distance
                }
            }
        };
        
        // Read chat mode with default fallback
        let chat_mode = VarInt::read(&mut cursor).unwrap_or(VarInt(0));
        
        // Read chat colors with default fallback
        let chat_colors = {
            let mut chat_colors_bytes = [0u8; 1];
            cursor.read_exact(&mut chat_colors_bytes).unwrap_or(());
            chat_colors_bytes[0] != 0
        };
        
        // Read displayed skin parts with default fallback
        let displayed_skin_parts = {
            let mut skin_parts_bytes = [0u8; 1];
            cursor.read_exact(&mut skin_parts_bytes).unwrap_or(());
            skin_parts_bytes[0]
        };
        
        // Read main hand with default fallback
        let main_hand = VarInt::read(&mut cursor).unwrap_or(VarInt(1));
        
        // Read text filtering with default fallback
        let enable_text_filtering = {
            let mut text_filtering_bytes = [0u8; 1];
            cursor.read_exact(&mut text_filtering_bytes).unwrap_or(());
            text_filtering_bytes[0] != 0
        };
        
        // Read server listings with default fallback
        let allow_server_listings = {
            let mut server_listings_bytes = [0u8; 1];
            cursor.read_exact(&mut server_listings_bytes).unwrap_or(());
            server_listings_bytes[0] != 0
        };
        
        Ok(ClientInformation {
            locale,
            view_distance,
            chat_mode,
            chat_colors,
            displayed_skin_parts,
            main_hand,
            enable_text_filtering,
            allow_server_listings,
        })
    }
}

/// Plugin Message packet (serverbound)
/// Packet ID: 0x02
#[derive(Debug)]
pub struct PluginMessage {
    /// Channel identifier
    pub channel: String,
    /// Message data
    pub data: Vec<u8>,
}

impl Packet for PluginMessage {
    fn packet_id() -> i32 {
        0x02
    }

    fn write_data<W: Write>(&self, _writer: &mut W) -> std::io::Result<()> {
        // This is a serverbound packet, so we don't need to implement writing
        Ok(())
    }

    fn read_data<R: Read>(_reader: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        // This is handled by from_packet_data instead
        unimplemented!("Use from_packet_data instead")
    }
}

impl PluginMessage {
    /// Parse plugin message from packet data
    pub fn from_packet_data(data: &[u8]) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        if data.is_empty() {
            return Err("Empty packet data for Plugin Message".into());
        }
        
        let mut cursor = Cursor::new(data);
        
        // Read channel string with error handling
        let channel = match VarInt::read(&mut cursor) {
            Ok(channel_length_varint) => {
                let channel_length = channel_length_varint.0 as usize;
                if channel_length == 0 {
                    "unknown".to_string()
                } else if channel_length > data.len() {
                    tracing::warn!("Invalid channel length in Plugin Message packet");
                    "unknown".to_string()
                } else {
                    let mut channel_bytes = vec![0u8; channel_length];
                    match cursor.read_exact(&mut channel_bytes) {
                        Ok(_) => String::from_utf8(channel_bytes).unwrap_or_else(|_| "unknown".to_string()),
                        Err(_) => {
                            tracing::warn!("Failed to read channel in Plugin Message packet");
                            "unknown".to_string()
                        }
                    }
                }
            },
            Err(_) => {
                tracing::warn!("Failed to read channel length in Plugin Message packet");
                "unknown".to_string()
            }
        };
        
        // Read remaining data
        let mut remaining_data = Vec::new();
        let _ = cursor.read_to_end(&mut remaining_data); // Don't fail if there's no more data
        
        Ok(PluginMessage {
            channel,
            data: remaining_data,
        })
    }
}

/// Command Suggestions Request packet (serverbound)
/// Packet ID: 0x03
#[derive(Debug)]
pub struct CommandSuggestionsRequest {
    /// Transaction ID
    pub transaction_id: VarInt,
    /// Command text
    pub text: String,
}

impl Packet for CommandSuggestionsRequest {
    fn packet_id() -> i32 {
        0x03
    }

    fn write_data<W: Write>(&self, _writer: &mut W) -> std::io::Result<()> {
        // This is a serverbound packet, so we don't need to implement writing
        Ok(())
    }

    fn read_data<R: Read>(_reader: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        // This is handled by from_packet_data instead
        unimplemented!("Use from_packet_data instead")
    }
}

impl CommandSuggestionsRequest {
    /// Parse command suggestions request from packet data
    pub fn from_packet_data(data: &[u8]) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // If the packet has no data, it might be a valid empty request
        if data.is_empty() {
            tracing::debug!("Received empty Command Suggestions Request packet, treating as default");
            return Ok(CommandSuggestionsRequest {
                transaction_id: VarInt(0),
                text: String::new(),
            });
        }
        
        let mut cursor = Cursor::new(data);
        
        // Try to read transaction ID - if this fails, the packet structure is different
        let transaction_id = match VarInt::read(&mut cursor) {
            Ok(id) => id,
            Err(_) => {
                // If we can't read the transaction ID, this might be a different packet type
                // For now, return a default to avoid crashing
                tracing::warn!("Failed to parse Command Suggestions Request packet, using default values");
                return Ok(CommandSuggestionsRequest {
                    transaction_id: VarInt(0),
                    text: String::new(),
                });
            }
        };
        
        // Try to read text string - check if we have enough data
        let text = match VarInt::read(&mut cursor) {
            Ok(text_length_varint) => {
                let text_length = text_length_varint.0 as usize;
                if text_length == 0 {
                    String::new()
                } else {
                    let mut text_bytes = vec![0u8; text_length];
                    match cursor.read_exact(&mut text_bytes) {
                        Ok(_) => String::from_utf8(text_bytes)?,
                        Err(_) => {
                            tracing::warn!("Insufficient data for Command Suggestions Request text field");
                            String::new()
                        }
                    }
                }
            },
            Err(_) => {
                tracing::warn!("Failed to read text length in Command Suggestions Request");
                String::new()
            }
        };
        
        Ok(CommandSuggestionsRequest {
            transaction_id,
            text,
        })
    }
}

/// Join Game packet (clientbound)
/// Packet ID: 0x29
#[derive(Debug)]
pub struct JoinGame {
    /// Entity ID of the player
    pub entity_id: i32,
    /// Whether hardcore mode is enabled
    pub is_hardcore: bool,
    /// Dimension names
    pub dimension_names: Vec<String>,
    /// Max players
    pub max_players: VarInt,
    /// View distance
    pub view_distance: VarInt,
    /// Simulation distance
    pub simulation_distance: VarInt,
    /// Reduced debug info
    pub reduced_debug_info: bool,
    /// Enable respawn screen
    pub enable_respawn_screen: bool,
    /// Do limited crafting
    pub do_limited_crafting: bool,
    /// Dimension type
    pub dimension_type: String,
    /// Dimension name
    pub dimension_name: String,
    /// Hashed seed
    pub hashed_seed: i64,
    /// Game mode
    pub game_mode: u8,
    /// Previous game mode
    pub previous_game_mode: i8,
    /// Is debug
    pub is_debug: bool,
    /// Is flat
    pub is_flat: bool,
    /// Death location
    pub death_location: Option<(String, i64)>,
    /// Portal cooldown
    pub portal_cooldown: VarInt,
}

impl Packet for JoinGame {
    fn packet_id() -> i32 {
        0x29
    }

    fn write_data<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        // Write entity ID
        writer.write_all(&self.entity_id.to_be_bytes())?;
        
        // Write hardcore flag
        writer.write_all(&[if self.is_hardcore { 1 } else { 0 }])?;
        
        // Write dimension names array
        VarInt(self.dimension_names.len() as i32).write(writer)?;
        for dimension in &self.dimension_names {
            VarInt(dimension.len() as i32).write(writer)?;
            writer.write_all(dimension.as_bytes())?;
        }
        
        // Write max players
        self.max_players.write(writer)?;
        
        // Write view distance
        self.view_distance.write(writer)?;
        
        // Write simulation distance
        self.simulation_distance.write(writer)?;
        
        // Write reduced debug info
        writer.write_all(&[if self.reduced_debug_info { 1 } else { 0 }])?;
        
        // Write enable respawn screen
        writer.write_all(&[if self.enable_respawn_screen { 1 } else { 0 }])?;
        
        // Write do limited crafting
        writer.write_all(&[if self.do_limited_crafting { 1 } else { 0 }])?;
        
        // Write dimension type
        VarInt(self.dimension_type.len() as i32).write(writer)?;
        writer.write_all(self.dimension_type.as_bytes())?;
        
        // Write dimension name  
        VarInt(self.dimension_name.len() as i32).write(writer)?;
        writer.write_all(self.dimension_name.as_bytes())?;
        
        // Write hashed seed
        writer.write_all(&self.hashed_seed.to_be_bytes())?;
        
        // Write game mode
        writer.write_all(&[self.game_mode])?;
        
        // Write previous game mode
        writer.write_all(&[self.previous_game_mode as u8])?;
        
        // Write is debug
        writer.write_all(&[if self.is_debug { 1 } else { 0 }])?;
        
        // Write is flat
        writer.write_all(&[if self.is_flat { 1 } else { 0 }])?;
        
        // Write death location (optional)
        if let Some((dimension, position)) = &self.death_location {
            writer.write_all(&[1])?; // Present
            VarInt(dimension.len() as i32).write(writer)?;
            writer.write_all(dimension.as_bytes())?;
            writer.write_all(&position.to_be_bytes())?;
        } else {
            writer.write_all(&[0])?; // Not present
        }
        
        // Write portal cooldown
        self.portal_cooldown.write(writer)?;
        
        Ok(())
    }

    fn read_data<R: Read>(_reader: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        // This is a clientbound packet, so we don't need to implement reading
        unimplemented!("This is a clientbound packet")
    }
}

impl JoinGame {
    /// Create a basic join game packet for testing
    pub fn new() -> Self {
        Self {
            entity_id: 1,
            is_hardcore: false,
            dimension_names: vec!["minecraft:overworld".to_string()],
            max_players: VarInt(20),
            view_distance: VarInt(10),
            simulation_distance: VarInt(10),
            reduced_debug_info: false,
            enable_respawn_screen: true,
            do_limited_crafting: false,
            dimension_type: "minecraft:overworld".to_string(),
            dimension_name: "minecraft:overworld".to_string(),
            hashed_seed: 0,
            game_mode: 1, // Creative mode
            previous_game_mode: -1,
            is_debug: false,
            is_flat: true,
            death_location: None,
            portal_cooldown: VarInt(0),
        }
    }
}
