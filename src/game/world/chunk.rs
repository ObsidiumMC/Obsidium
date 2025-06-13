//! Chunk management
//!
//! This module handles individual chunks and their block data.

use super::ChunkPosition;

/// Chunk size constants
pub const CHUNK_SIZE: usize = 16;
/// Maximum chunk height (1.18+ world height)
pub const CHUNK_HEIGHT: usize = 384;
/// Minimum Y coordinate in chunks
pub const CHUNK_MIN_Y: i32 = -64;
/// Maximum Y coordinate in chunks
pub const CHUNK_MAX_Y: i32 = 319;

/// Represents a single chunk in the world
pub struct Chunk {
    /// Chunk position
    position: ChunkPosition,
    /// Block data [y][z][x]
    blocks: Vec<Vec<Vec<u32>>>,
    /// Whether the chunk has been modified
    modified: bool,
}

impl Chunk {
    /// Create a new empty chunk
    pub fn new(position: ChunkPosition) -> Self {
        let mut blocks = Vec::with_capacity(CHUNK_HEIGHT);
        for _ in 0..CHUNK_HEIGHT {
            let mut layer = Vec::with_capacity(CHUNK_SIZE);
            for _ in 0..CHUNK_SIZE {
                let row = vec![0; CHUNK_SIZE]; // Air block
                layer.push(row);
            }
            blocks.push(layer);
        }

        Self {
            position,
            blocks,
            modified: false,
        }
    }

    /// Generate a simple flat chunk for testing
    pub fn generate_flat(position: ChunkPosition) -> Self {
        let mut chunk = Self::new(position);

        // Generate flat terrain
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                // Bedrock at bottom
                chunk.set_block(x, 0, z, 7); // Bedrock

                // Stone layers
                for y in 1..60 {
                    chunk.set_block(x, y, z, 1); // Stone
                }

                // Dirt layers
                for y in 60..63 {
                    chunk.set_block(x, y, z, 3); // Dirt
                }

                // Grass on top
                chunk.set_block(x, 63, z, 2); // Grass
            }
        }

        chunk.modified = false; // Reset modified flag after generation
        chunk
    }

    /// Get the chunk position
    pub fn position(&self) -> ChunkPosition {
        self.position
    }

    /// Get block at local coordinates
    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Option<u32> {
        if x >= CHUNK_SIZE || z >= CHUNK_SIZE || y >= CHUNK_HEIGHT {
            return None;
        }

        Some(self.blocks[y][z][x])
    }

    /// Set block at local coordinates
    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block_id: u32) -> bool {
        if x >= CHUNK_SIZE || z >= CHUNK_SIZE || y >= CHUNK_HEIGHT {
            return false;
        }

        self.blocks[y][z][x] = block_id;
        self.modified = true;
        true
    }

    /// Check if the chunk has been modified
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    /// Mark the chunk as saved (clears modified flag)
    pub fn mark_saved(&mut self) {
        self.modified = false;
    }

    /// Get the highest non-air block at the given coordinates
    pub fn get_height(&self, x: usize, z: usize) -> Option<usize> {
        if x >= CHUNK_SIZE || z >= CHUNK_SIZE {
            return None;
        }

        (0..CHUNK_HEIGHT).rev().find(|&y| self.blocks[y][z][x] != 0)
    }

    /// Check if a position is within chunk bounds
    pub fn is_valid_position(x: usize, y: usize, z: usize) -> bool {
        x < CHUNK_SIZE && y < CHUNK_HEIGHT && z < CHUNK_SIZE
    }

    /// Get all block data (for serialization)
    pub fn blocks(&self) -> &Vec<Vec<Vec<u32>>> {
        &self.blocks
    }

    /// Count non-air blocks in the chunk
    pub fn count_blocks(&self) -> usize {
        let mut count = 0;
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    if self.blocks[y][z][x] != 0 {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    /// Check if the chunk is empty (all air)
    pub fn is_empty(&self) -> bool {
        self.count_blocks() == 0
    }
}
