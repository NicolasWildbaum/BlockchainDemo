use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::models::{CoinbaseTx, TransferTx};
use crate::models::Block;

pub const DIFFICULTY_PREFIX: &str = "0000";
pub const GENESIS_PREVIOUS_HASH: &str = "0000000000000000";

// slap coinbase + txs into json for the tail of the hash string
fn json_tail(coinbase: &Option<CoinbaseTx>, transactions: &[TransferTx]) -> String {
    #[derive(Serialize)]
    struct Tail<'a> {
        coinbase: &'a Option<CoinbaseTx>,
        transactions: &'a [TransferTx],
    }
    let tail = Tail {
        coinbase,
        transactions,
    };
    serde_json::to_string(&tail).expect("tail serializes")
}

/// SHA-256 hex, roughly index|nonce|data|prev|json(coinbase + txs)
pub fn hash_block(
    index: u64,
    nonce: u64,
    data: &str,
    previous_hash: &str,
    coinbase: &Option<CoinbaseTx>,
    transactions: &[TransferTx],
) -> String {
    let tail = json_tail(coinbase, transactions);
    let payload = format!(
        "{}|{}|{}|{}|{}",
        index, nonce, data, previous_hash, tail
    );
    let mut hasher = Sha256::new();
    hasher.update(payload.as_bytes());
    hasher
        .finalize()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

pub fn hash_matches_difficulty(hash: &str) -> bool {
    hash.starts_with(DIFFICULTY_PREFIX)
}

pub fn recompute_block_hash(block: &mut Block) {
    block.hash = hash_block(
        block.index,
        block.nonce,
        &block.data,
        &block.previous_hash,
        &block.coinbase,
        &block.transactions,
    );
}

pub fn mine_nonce(
    index: u64,
    data: &str,
    previous_hash: &str,
    coinbase: &Option<CoinbaseTx>,
    transactions: &[TransferTx],
) -> (u64, String) {
    let mut nonce = 0u64;
    loop {
        let h = hash_block(
            index,
            nonce,
            data,
            previous_hash,
            coinbase,
            transactions,
        );
        if hash_matches_difficulty(&h) {
            return (nonce, h);
        }
        nonce += 1;
    }
}
