//! Entity system
//!
//! This module handles game entities including their properties, behaviors,
//! and interactions.

pub mod player;

use crate::protocol::types::McUuid;
use std::collections::HashMap;

/// Entity ID type
pub type EntityId = i32;

/// Base entity trait
pub trait Entity: Send + Sync {
    /// Get the entity ID
    fn entity_id(&self) -> EntityId;

    /// Get the entity type
    fn entity_type(&self) -> EntityType;

    /// Get the entity position
    fn position(&self) -> EntityPosition;

    /// Get the entity rotation
    fn rotation(&self) -> EntityRotation;

    /// Get the entity UUID (if applicable)
    fn uuid(&self) -> Option<McUuid>;

    /// Check if the entity is alive
    fn is_alive(&self) -> bool;

    /// Update the entity
    fn update(&mut self, delta_time: f64);
}

/// Entity types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    /// Player entity
    Player,
    /// Generic mob
    Mob(MobType),
    /// Item entity
    Item,
    /// Experience orb
    ExperienceOrb,
    /// Projectile
    Projectile(ProjectileType),
}

/// Mob types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MobType {
    /// Zombie
    Zombie,
    /// Skeleton
    Skeleton,
    /// Creeper
    Creeper,
    /// Spider
    Spider,
    /// Cow
    Cow,
    /// Pig
    Pig,
    /// Sheep
    Sheep,
    /// Chicken
    Chicken,
}

/// Projectile types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectileType {
    /// Arrow
    Arrow,
    /// Snowball
    Snowball,
    /// Fireball
    Fireball,
}

/// Entity position
#[derive(Debug, Clone, Copy)]
pub struct EntityPosition {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
    /// Z coordinate
    pub z: f64,
}

/// Entity rotation
#[derive(Debug, Clone, Copy)]
pub struct EntityRotation {
    /// Yaw (horizontal rotation)
    pub yaw: f32,
    /// Pitch (vertical rotation)
    pub pitch: f32,
}

/// Entity manager
pub struct EntityManager {
    /// Map of entity ID to entity
    entities: HashMap<EntityId, Box<dyn Entity>>,
    /// Next available entity ID
    next_entity_id: EntityId,
}

impl EntityManager {
    /// Create a new entity manager
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            next_entity_id: 1, // Start from 1, as 0 might be reserved
        }
    }

    /// Generate a new entity ID
    pub fn next_entity_id(&mut self) -> EntityId {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        id
    }

    /// Add an entity
    pub fn add_entity(&mut self, entity: Box<dyn Entity>) -> EntityId {
        let entity_id = entity.entity_id();
        self.entities.insert(entity_id, entity);
        entity_id
    }

    /// Remove an entity
    pub fn remove_entity(&mut self, entity_id: EntityId) -> Option<Box<dyn Entity>> {
        self.entities.remove(&entity_id)
    }

    /// Get an entity
    pub fn get_entity(&self, entity_id: EntityId) -> Option<&dyn Entity> {
        self.entities.get(&entity_id).map(|e| e.as_ref())
    }

    /// Get a mutable reference to an entity
    pub fn get_entity_mut(&mut self, entity_id: EntityId) -> Option<&mut Box<dyn Entity>> {
        self.entities.get_mut(&entity_id)
    }

    /// Get all entities
    pub fn entities(&self) -> impl Iterator<Item = &dyn Entity> {
        self.entities.values().map(|e| e.as_ref())
    }

    /// Update all entities
    pub fn update_all(&mut self, delta_time: f64) {
        for entity in self.entities.values_mut() {
            entity.update(delta_time);
        }

        // Remove dead entities
        self.entities.retain(|_, entity| entity.is_alive());
    }

    /// Get entity count
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }
}

impl Default for EntityManager {
    fn default() -> Self {
        Self::new()
    }
}
