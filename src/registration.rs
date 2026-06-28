use crate::types::BlockType;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct BlockRegistry {
    blocks: HashMap<String, BlockType>,
}

impl BlockRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_block_type(&mut self, block: BlockType) {
        self.blocks.insert(block.name.clone(), block);
    }

    pub fn get_block_type(&self, name: &str) -> Option<&BlockType> {
        self.blocks.get(name)
    }
}
