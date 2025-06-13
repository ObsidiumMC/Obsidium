//! Player entity implementation
//!
//! This module contains the entity implementation for players.

use super::{Entity, EntityId, EntityPosition, EntityRotation, EntityType};
use crate::game::player::Player;
use crate::protocol::types::McUuid;

/// Player entity wrapper
pub struct PlayerEntity {
    /// Entity ID
    entity_id: EntityId,
    /// Player data
    player: Player,
}

impl PlayerEntity {
    /// Create a new player entity
    pub fn new(entity_id: EntityId, player: Player) -> Self {
        Self { entity_id, player }
    }
    
    /// Get the player data
    pub fn player(&self) -> &Player {
        &self.player
    }
    
    /// Get mutable player data
    pub fn player_mut(&mut self) -> &mut Player {
        &mut self.player
    }
}

impl Entity for PlayerEntity {
    fn entity_id(&self) -> EntityId {
        self.entity_id
    }
    
    fn entity_type(&self) -> EntityType {
        EntityType::Player
    }
    
    fn position(&self) -> EntityPosition {
        EntityPosition {
            x: self.player.position.x,
            y: self.player.position.y,
            z: self.player.position.z,
        }
    }
    
    fn rotation(&self) -> EntityRotation {
        EntityRotation {
            yaw: self.player.rotation.yaw,
            pitch: self.player.rotation.pitch,
        }
    }
    
    fn uuid(&self) -> Option<McUuid> {
        Some(self.player.uuid)
    }
    
    fn is_alive(&self) -> bool {
        self.player.is_alive()
    }
    
    fn update(&mut self, _delta_time: f64) {
        // Player updates are handled separately through player manager
        // This could be used for things like fall damage, regeneration, etc.
    }
}
