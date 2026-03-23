use super::block::Block;

#[derive(Debug, Clone, Default)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    pub fn tip_hash(&self) -> Option<&str> {
        self.blocks.last().map(|b| b.hash.as_str())
    }

    pub fn latest_index(&self) -> Option<u64> {
        self.blocks.last().map(|b| b.index)
    }
}
