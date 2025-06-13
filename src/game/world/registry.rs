//! Block and item registries
//!
//! This module manages the registries for blocks, items, and other game objects.

use std::collections::HashMap;

/// Block registry managing block types and their properties
pub struct BlockRegistry {
    /// Map of block ID to block info
    blocks: HashMap<u32, BlockInfo>,
    /// Map of block name to block ID
    name_to_id: HashMap<String, u32>,
}

/// Information about a block type
#[derive(Debug, Clone)]
pub struct BlockInfo {
    /// Block ID
    pub id: u32,
    /// Block name (e.g., "minecraft:stone")
    pub name: String,
    /// Whether the block is solid
    pub solid: bool,
    /// Whether the block is transparent
    pub transparent: bool,
    /// Block hardness (mining time)
    pub hardness: f32,
    /// Block resistance (explosion resistance)
    pub resistance: f32,
}

impl BlockRegistry {
    /// Create a new block registry with default blocks
    pub fn new() -> Self {
        let mut registry = Self {
            blocks: HashMap::new(),
            name_to_id: HashMap::new(),
        };

        // Register default blocks
        registry.register_default_blocks();
        registry
    }

    /// Register a new block
    pub fn register_block(&mut self, info: BlockInfo) {
        self.name_to_id.insert(info.name.clone(), info.id);
        self.blocks.insert(info.id, info);
    }

    /// Get block info by ID
    pub fn get_block(&self, id: u32) -> Option<&BlockInfo> {
        self.blocks.get(&id)
    }

    /// Get block ID by name
    pub fn get_block_id(&self, name: &str) -> Option<u32> {
        self.name_to_id.get(name).copied()
    }

    /// Get block info by name
    pub fn get_block_by_name(&self, name: &str) -> Option<&BlockInfo> {
        let id = self.get_block_id(name)?;
        self.get_block(id)
    }

    /// Get all registered blocks
    pub fn all_blocks(&self) -> impl Iterator<Item = &BlockInfo> {
        self.blocks.values()
    }

    /// Get block count
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    /// Register default Minecraft blocks
    fn register_default_blocks(&mut self) {
        let default_blocks = [
            BlockInfo {
                id: 0,
                name: "minecraft:air".to_string(),
                solid: false,
                transparent: true,
                hardness: 0.0,
                resistance: 0.0,
            },
            BlockInfo {
                id: 1,
                name: "minecraft:stone".to_string(),
                solid: true,
                transparent: false,
                hardness: 1.5,
                resistance: 6.0,
            },
            BlockInfo {
                id: 2,
                name: "minecraft:grass_block".to_string(),
                solid: true,
                transparent: false,
                hardness: 0.6,
                resistance: 0.6,
            },
            BlockInfo {
                id: 3,
                name: "minecraft:dirt".to_string(),
                solid: true,
                transparent: false,
                hardness: 0.5,
                resistance: 0.5,
            },
            BlockInfo {
                id: 4,
                name: "minecraft:cobblestone".to_string(),
                solid: true,
                transparent: false,
                hardness: 2.0,
                resistance: 6.0,
            },
            BlockInfo {
                id: 5,
                name: "minecraft:oak_planks".to_string(),
                solid: true,
                transparent: false,
                hardness: 2.0,
                resistance: 3.0,
            },
            BlockInfo {
                id: 6,
                name: "minecraft:oak_sapling".to_string(),
                solid: false,
                transparent: true,
                hardness: 0.0,
                resistance: 0.0,
            },
            BlockInfo {
                id: 7,
                name: "minecraft:bedrock".to_string(),
                solid: true,
                transparent: false,
                hardness: -1.0, // Unbreakable
                resistance: 3600000.0,
            },
        ];

        for block in default_blocks {
            self.register_block(block);
        }
    }
}

impl Default for BlockRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Item registry managing item types and their properties
pub struct ItemRegistry {
    /// Map of item ID to item info
    items: HashMap<u32, ItemInfo>,
    /// Map of item name to item ID
    name_to_id: HashMap<String, u32>,
}

/// Information about an item type
#[derive(Debug, Clone)]
pub struct ItemInfo {
    /// Item ID
    pub id: u32,
    /// Item name (e.g., "minecraft:diamond_sword")
    pub name: String,
    /// Maximum stack size
    pub max_stack_size: u32,
    /// Whether the item is damageable
    pub damageable: bool,
    /// Maximum durability (if damageable)
    pub max_durability: Option<u32>,
}

impl ItemRegistry {
    /// Create a new item registry
    pub fn new() -> Self {
        let mut registry = Self {
            items: HashMap::new(),
            name_to_id: HashMap::new(),
        };

        // Register default items
        registry.register_default_items();
        registry
    }

    /// Register a new item
    pub fn register_item(&mut self, info: ItemInfo) {
        self.name_to_id.insert(info.name.clone(), info.id);
        self.items.insert(info.id, info);
    }

    /// Get item info by ID
    pub fn get_item(&self, id: u32) -> Option<&ItemInfo> {
        self.items.get(&id)
    }

    /// Get item ID by name
    pub fn get_item_id(&self, name: &str) -> Option<u32> {
        self.name_to_id.get(name).copied()
    }

    /// Get all registered items
    pub fn all_items(&self) -> impl Iterator<Item = &ItemInfo> {
        self.items.values()
    }

    /// Get item count
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Register default Minecraft items
    fn register_default_items(&mut self) {
        let default_items = [
            ItemInfo {
                id: 1,
                name: "minecraft:stone".to_string(),
                max_stack_size: 64,
                damageable: false,
                max_durability: None,
            },
            ItemInfo {
                id: 276,
                name: "minecraft:diamond_sword".to_string(),
                max_stack_size: 1,
                damageable: true,
                max_durability: Some(1561),
            },
            ItemInfo {
                id: 364,
                name: "minecraft:bread".to_string(),
                max_stack_size: 64,
                damageable: false,
                max_durability: None,
            },
        ];

        for item in default_items {
            self.register_item(item);
        }
    }
}

impl Default for ItemRegistry {
    fn default() -> Self {
        Self::new()
    }
}
