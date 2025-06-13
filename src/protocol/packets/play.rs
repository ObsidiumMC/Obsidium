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

/// Login (play) packet (clientbound)
///
/// This is the first packet sent when transitioning from configuration to play state.
/// It contains essential world and gameplay configuration that the client needs
/// to properly initialize its game state.
///
/// Packet ID: 0x2B
#[derive(Debug, Clone)]
pub struct LoginPlayPacket {
    /// The player's Entity ID (EID)
    pub entity_id: i32,
    /// Whether hardcore mode is enabled
    pub is_hardcore: bool,
    /// Identifiers for all dimensions on the server
    pub dimension_names: Vec<McString>,
    /// Maximum players (legacy field, now ignored by client)
    pub max_players: VarInt,
    /// Render distance in chunks (2-32)
    pub view_distance: VarInt,
    /// Simulation distance in chunks (distance for entity processing)
    pub simulation_distance: VarInt,
    /// Whether to show reduced debug information in F3 screen
    pub reduced_debug_info: bool,
    /// Whether to show the respawn screen when the player dies
    pub enable_respawn_screen: bool,
    /// Whether crafting is limited (used for recipe book functionality)
    pub do_limited_crafting: bool,
    /// The ID of the dimension type in the minecraft:dimension_type registry
    pub dimension_type: VarInt,
    /// Name of the dimension being spawned into
    pub dimension_name: McString,
    /// First 8 bytes of SHA-256 hash of world seed (for client-side biome noise)
    pub hashed_seed: i64,
    /// Current game mode (0=Survival, 1=Creative, 2=Adventure, 3=Spectator)
    pub game_mode: u8,
    /// Previous game mode (-1=Undefined, 0=Survival, 1=Creative, 2=Adventure, 3=Spectator)
    pub previous_game_mode: i8,
    /// Whether this is a debug world
    pub is_debug: bool,
    /// Whether this is a flat/superflat world
    pub is_flat: bool,
    /// Whether the player has a death location
    pub has_death_location: bool,
    /// Death dimension name (if has_death_location is true)
    pub death_dimension_name: Option<McString>,
    /// Death location (if has_death_location is true)
    pub death_location: Option<Position>,
    /// Portal cooldown in ticks
    pub portal_cooldown: VarInt,
    /// Sea level
    pub sea_level: VarInt,
    /// Whether the server enforces secure chat (cryptographically signed messages)
    pub enforces_secure_chat: bool,
}

impl Packet for LoginPlayPacket {
    const ID: i32 = 0x2B;

    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let mut entity_id_bytes = [0u8; 4];
        reader.read_exact(&mut entity_id_bytes)?;
        let entity_id = i32::from_be_bytes(entity_id_bytes);

        let is_hardcore = crate::protocol::types::read_bool(reader)?;

        let dimension_count = VarInt::read(reader)?;
        let mut dimension_names = Vec::new();
        for _ in 0..dimension_count.0 {
            dimension_names.push(McString::read(reader)?);
        }

        let max_players = VarInt::read(reader)?;
        let view_distance = VarInt::read(reader)?;
        let simulation_distance = VarInt::read(reader)?;
        let reduced_debug_info = crate::protocol::types::read_bool(reader)?;
        let enable_respawn_screen = crate::protocol::types::read_bool(reader)?;
        let do_limited_crafting = crate::protocol::types::read_bool(reader)?;
        let dimension_type = VarInt::read(reader)?;
        let dimension_name = McString::read(reader)?;

        let mut hashed_seed_bytes = [0u8; 8];
        reader.read_exact(&mut hashed_seed_bytes)?;
        let hashed_seed = i64::from_be_bytes(hashed_seed_bytes);

        let mut game_mode_byte = [0u8; 1];
        reader.read_exact(&mut game_mode_byte)?;
        let game_mode = game_mode_byte[0];

        let mut previous_game_mode_byte = [0u8; 1];
        reader.read_exact(&mut previous_game_mode_byte)?;
        let previous_game_mode = previous_game_mode_byte[0] as i8;

        let is_debug = crate::protocol::types::read_bool(reader)?;
        let is_flat = crate::protocol::types::read_bool(reader)?;
        let has_death_location = crate::protocol::types::read_bool(reader)?;

        let (death_dimension_name, death_location) = if has_death_location {
            let dimension = McString::read(reader)?;
            let position = Position::read(reader)?;
            (Some(dimension), Some(position))
        } else {
            (None, None)
        };

        let portal_cooldown = VarInt::read(reader)?;
        let sea_level = VarInt::read(reader)?;
        let enforces_secure_chat = crate::protocol::types::read_bool(reader)?;

        Ok(LoginPlayPacket {
            entity_id,
            is_hardcore,
            dimension_names,
            max_players,
            view_distance,
            simulation_distance,
            reduced_debug_info,
            enable_respawn_screen,
            do_limited_crafting,
            dimension_type,
            dimension_name,
            hashed_seed,
            game_mode,
            previous_game_mode,
            is_debug,
            is_flat,
            has_death_location,
            death_dimension_name,
            death_location,
            portal_cooldown,
            sea_level,
            enforces_secure_chat,
        })
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.entity_id.to_be_bytes())?;
        crate::protocol::types::write_bool(self.is_hardcore, writer)?;

        VarInt(self.dimension_names.len() as i32).write(writer)?;
        for dimension in &self.dimension_names {
            dimension.write(writer)?;
        }

        self.max_players.write(writer)?;
        self.view_distance.write(writer)?;
        self.simulation_distance.write(writer)?;
        crate::protocol::types::write_bool(self.reduced_debug_info, writer)?;
        crate::protocol::types::write_bool(self.enable_respawn_screen, writer)?;
        crate::protocol::types::write_bool(self.do_limited_crafting, writer)?;
        self.dimension_type.write(writer)?;
        self.dimension_name.write(writer)?;
        writer.write_all(&self.hashed_seed.to_be_bytes())?;
        writer.write_all(&[self.game_mode])?;
        writer.write_all(&[self.previous_game_mode as u8])?;
        crate::protocol::types::write_bool(self.is_debug, writer)?;
        crate::protocol::types::write_bool(self.is_flat, writer)?;
        crate::protocol::types::write_bool(self.has_death_location, writer)?;

        if self.has_death_location {
            if let (Some(dimension), Some(position)) =
                (&self.death_dimension_name, &self.death_location)
            {
                dimension.write(writer)?;
                position.write(writer)?;
            }
        }

        self.portal_cooldown.write(writer)?;
        self.sea_level.write(writer)?;
        crate::protocol::types::write_bool(self.enforces_secure_chat, writer)?;

        Ok(())
    }
}

impl ClientboundPacket for LoginPlayPacket {}

impl LoginPlayPacket {
    /// Create a new login play packet with default values
    pub fn new() -> Self {
        Self {
            entity_id: 1,
            is_hardcore: false,
            dimension_names: vec!["minecraft:overworld".into()],
            max_players: VarInt(20),
            view_distance: VarInt(10),
            simulation_distance: VarInt(10),
            reduced_debug_info: false,
            enable_respawn_screen: true,
            do_limited_crafting: false,
            dimension_type: VarInt(0),
            dimension_name: "minecraft:overworld".into(),
            hashed_seed: 0,
            game_mode: 0,           // Survival
            previous_game_mode: -1, // None
            is_debug: false,
            is_flat: false,
            has_death_location: false,
            death_dimension_name: None,
            death_location: None,
            portal_cooldown: VarInt(0),
            sea_level: VarInt(63),
            enforces_secure_chat: false,
        }
    }

    /// Create login play packet from server config
    pub fn from_server_config(config: &crate::config::ServerConfig, entity_id: i32) -> Self {
        Self {
            entity_id,
            is_hardcore: false,
            dimension_names: vec!["minecraft:overworld".into()],
            max_players: VarInt(config.max_players as i32),
            view_distance: VarInt(config.view_distance as i32),
            simulation_distance: VarInt(config.simulation_distance as i32),
            reduced_debug_info: false,
            enable_respawn_screen: true,
            do_limited_crafting: false,
            dimension_type: VarInt(0),
            dimension_name: "minecraft:overworld".into(),
            hashed_seed: 12345,     // Use world seed hash
            game_mode: 0,           // Survival mode
            previous_game_mode: -1, // No previous game mode
            is_debug: config.debug,
            is_flat: false,
            has_death_location: false,
            death_dimension_name: None,
            death_location: None,
            portal_cooldown: VarInt(0),
            sea_level: VarInt(63),
            enforces_secure_chat: false,
        }
    }
}

// TODO: Add more play packets as needed
// - Chunk data packets
// - Entity packets
// - Inventory packets
// - etc.

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_login_play_packet_roundtrip() {
        let packet = LoginPlayPacket::new();

        let mut buffer = Vec::new();
        packet.write(&mut buffer).unwrap();

        let mut cursor = Cursor::new(buffer);
        let decoded = LoginPlayPacket::read(&mut cursor).unwrap();

        assert_eq!(packet.entity_id, decoded.entity_id);
        assert_eq!(packet.view_distance.0, decoded.view_distance.0);
        assert_eq!(packet.simulation_distance.0, decoded.simulation_distance.0);
        assert_eq!(packet.reduced_debug_info, decoded.reduced_debug_info);
        assert_eq!(packet.enable_respawn_screen, decoded.enable_respawn_screen);
        assert_eq!(packet.do_limited_crafting, decoded.do_limited_crafting);
        assert_eq!(packet.dimension_name.0, decoded.dimension_name.0);
        assert_eq!(packet.hashed_seed, decoded.hashed_seed);
        assert_eq!(packet.game_mode, decoded.game_mode);
        assert_eq!(packet.previous_game_mode, decoded.previous_game_mode);
        assert_eq!(packet.is_debug, decoded.is_debug);
        assert_eq!(packet.is_flat, decoded.is_flat);
        assert_eq!(packet.has_death_location, decoded.has_death_location);
        assert_eq!(packet.portal_cooldown.0, decoded.portal_cooldown.0);
        assert_eq!(packet.enforces_secure_chat, decoded.enforces_secure_chat);
    }

    #[test]
    fn test_login_play_packet_with_death_location() {
        let mut packet = LoginPlayPacket::new();
        packet.has_death_location = true;
        packet.death_dimension_name = Some("minecraft:the_nether".into());
        packet.death_location = Some(Position::new(100, 64, -200));

        let mut buffer = Vec::new();
        packet.write(&mut buffer).unwrap();

        let mut cursor = Cursor::new(buffer);
        let decoded = LoginPlayPacket::read(&mut cursor).unwrap();

        assert!(decoded.has_death_location);
        assert!(decoded.death_dimension_name.is_some());
        assert!(decoded.death_location.is_some());

        let dimension = decoded.death_dimension_name.unwrap();
        let position = decoded.death_location.unwrap();

        assert_eq!(dimension.0, "minecraft:the_nether");
        assert_eq!(position.x, 100);
        assert_eq!(position.y, 64);
        assert_eq!(position.z, -200);
    }

    #[test]
    fn test_login_play_packet_from_server_config() {
        use crate::config::ServerConfig;

        let config = ServerConfig::new().with_debug(true);
        let packet = LoginPlayPacket::from_server_config(&config, 42);

        assert_eq!(packet.entity_id, 42);
        assert_eq!(packet.view_distance.0, config.view_distance as i32);
        assert_eq!(
            packet.simulation_distance.0,
            config.simulation_distance as i32
        );
        assert_eq!(packet.is_debug, config.debug);
        assert_eq!(packet.max_players.0, config.max_players as i32);
    }
}
