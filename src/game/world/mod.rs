//! World management
//!
//! This module handles world state, chunks, blocks, and world generation.

pub mod chunk;
pub mod registry;

use crate::game::entity::EntityManager;
use crate::protocol::types::Position;
use std::collections::HashMap;

/// Represents a Minecraft world
pub struct World {
    /// World name
    pub name: String,
    /// World seed
    pub seed: i64,
    /// Loaded chunks
    chunks: HashMap<ChunkPosition, chunk::Chunk>,
    /// Entity manager for this world
    entities: EntityManager,
    /// World spawn position
    spawn_position: Position,
}

/// Chunk position (x, z coordinates)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPosition {
    /// X coordinate (in chunks)
    pub x: i32,
    /// Z coordinate (in chunks)
    pub z: i32,
}

impl ChunkPosition {
    /// Create a new chunk position
    pub fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    /// Convert world coordinates to chunk position
    pub fn from_world_coords(x: f64, z: f64) -> Self {
        Self {
            x: (x as i32) >> 4, // Divide by 16
            z: (z as i32) >> 4, // Divide by 16
        }
    }

    /// Get the world X coordinate of this chunk's origin
    pub fn world_x(&self) -> i32 {
        self.x << 4 // Multiply by 16
    }

    /// Get the world Z coordinate of this chunk's origin
    pub fn world_z(&self) -> i32 {
        self.z << 4 // Multiply by 16
    }
}

impl World {
    /// Create a new world
    pub fn new(name: String, seed: i64) -> Self {
        Self {
            name,
            seed,
            chunks: HashMap::new(),
            entities: EntityManager::new(),
            spawn_position: Position::new(0, 64, 0),
        }
    }

    /// Get world name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get world seed
    pub fn seed(&self) -> i64 {
        self.seed
    }

    /// Get spawn position
    pub fn spawn_position(&self) -> Position {
        self.spawn_position
    }

    /// Set spawn position
    pub fn set_spawn_position(&mut self, position: Position) {
        self.spawn_position = position;
    }

    /// Load a chunk
    pub fn load_chunk(&mut self, position: ChunkPosition) -> &chunk::Chunk {
        self.chunks.entry(position).or_insert_with(|| {
            // For now, generate a simple flat chunk
            // In a real implementation, this would use world generation
            chunk::Chunk::generate_flat(position)
        })
    }

    /// Unload a chunk
    pub fn unload_chunk(&mut self, position: ChunkPosition) {
        self.chunks.remove(&position);
        tracing::debug!("Unloaded chunk at {:?}", position);
    }

    /// Get a chunk if it's loaded
    pub fn get_chunk(&self, position: ChunkPosition) -> Option<&chunk::Chunk> {
        self.chunks.get(&position)
    }

    /// Get a mutable reference to a chunk if it's loaded
    pub fn get_chunk_mut(&mut self, position: ChunkPosition) -> Option<&mut chunk::Chunk> {
        self.chunks.get_mut(&position)
    }

    /// Check if a chunk is loaded
    pub fn is_chunk_loaded(&self, position: ChunkPosition) -> bool {
        self.chunks.contains_key(&position)
    }

    /// Get all loaded chunks
    pub fn loaded_chunks(&self) -> impl Iterator<Item = (ChunkPosition, &chunk::Chunk)> {
        self.chunks.iter().map(|(pos, chunk)| (*pos, chunk))
    }

    /// Get loaded chunk count
    pub fn loaded_chunk_count(&self) -> usize {
        self.chunks.len()
    }

    /// Get the entity manager
    pub fn entities(&self) -> &EntityManager {
        &self.entities
    }

    /// Get mutable entity manager
    pub fn entities_mut(&mut self) -> &mut EntityManager {
        &mut self.entities
    }

    /// Get block at position
    pub fn get_block(&self, position: Position) -> Option<u32> {
        let chunk_pos = ChunkPosition::from_world_coords(position.x as f64, position.z as f64);
        let chunk = self.get_chunk(chunk_pos)?;

        // Convert world coordinates to chunk-local coordinates
        let local_x = (position.x - chunk_pos.world_x()) as usize;
        let local_z = (position.z - chunk_pos.world_z()) as usize;
        let y = position.y as usize;

        chunk.get_block(local_x, y, local_z)
    }

    /// Set block at position
    pub fn set_block(&mut self, position: Position, block_id: u32) -> bool {
        let chunk_pos = ChunkPosition::from_world_coords(position.x as f64, position.z as f64);

        // Load chunk if not loaded
        self.load_chunk(chunk_pos);

        if let Some(chunk) = self.get_chunk_mut(chunk_pos) {
            let local_x = (position.x - chunk_pos.world_x()) as usize;
            let local_z = (position.z - chunk_pos.world_z()) as usize;
            let y = position.y as usize;

            chunk.set_block(local_x, y, local_z, block_id)
        } else {
            false
        }
    }

    /// Update the world
    pub fn update(&mut self, delta_time: f64) {
        // Update entities
        self.entities.update_all(delta_time);

        // TODO: Add other world updates like:
        // - Block updates (redstone, water flow, etc.)
        // - Weather
        // - Day/night cycle
        // - Chunk generation/unloading based on player positions
    }
}
