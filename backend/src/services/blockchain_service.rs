use crate::models::block::Block;
use crate::models::blockchain::Blockchain;

pub fn get_block_by_index(chain: &Blockchain, index: u64) -> Option<&Block> {
    chain.blocks.get(index as usize).filter(|b| b.index == index)
}
