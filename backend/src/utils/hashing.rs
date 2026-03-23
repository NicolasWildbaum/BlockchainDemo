use crate::models::block::{Block, CoinbaseTx};
use crate::models::transaction::ConfirmedTransaction;
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Serialize)]
struct CoinbaseHashView<'a> {
    tx_id: &'a str,
    miner_account_id: u32,
    amount: u64,
}

#[derive(Serialize)]
struct TxHashView<'a> {
    id: &'a str,
    from_account_id: u32,
    to_account_id: u32,
    amount: u64,
    timestamp: String,
}

#[derive(Serialize)]
struct BlockHashPayload<'a> {
    index: u64,
    timestamp: String,
    nonce: u64,
    previous_hash: &'a str,
    difficulty: u32,
    coinbase: CoinbaseHashView<'a>,
    transactions: Vec<TxHashView<'a>>,
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

pub fn sha256_hex(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    bytes_to_hex(&hasher.finalize())
}

/// Canonical JSON payload used for PoW and integrity checks (excludes `hash` field).
pub fn block_hash_payload(
    index: u64,
    timestamp: chrono::DateTime<chrono::Utc>,
    nonce: u64,
    previous_hash: &str,
    difficulty: u32,
    coinbase: &CoinbaseTx,
    transactions: &[ConfirmedTransaction],
) -> String {
    let coinbase = CoinbaseHashView {
        tx_id: coinbase.tx_id.as_str(),
        miner_account_id: coinbase.miner_account_id,
        amount: coinbase.amount,
    };
    let transactions: Vec<TxHashView> = transactions
        .iter()
        .map(|t| TxHashView {
            id: t.id.as_str(),
            from_account_id: t.from_account_id,
            to_account_id: t.to_account_id,
            amount: t.amount,
            timestamp: t.timestamp.to_rfc3339(),
        })
        .collect();
    let payload = BlockHashPayload {
        index,
        timestamp: timestamp.to_rfc3339(),
        nonce,
        previous_hash,
        difficulty,
        coinbase,
        transactions,
    };
    serde_json::to_string(&payload).expect("block hash payload serializes")
}

pub fn hash_block(
    index: u64,
    timestamp: chrono::DateTime<chrono::Utc>,
    nonce: u64,
    previous_hash: &str,
    difficulty: u32,
    coinbase: &CoinbaseTx,
    transactions: &[ConfirmedTransaction],
) -> String {
    let payload = block_hash_payload(
        index,
        timestamp,
        nonce,
        previous_hash,
        difficulty,
        coinbase,
        transactions,
    );
    sha256_hex(&payload)
}

pub fn hash_matches_difficulty(hash: &str, difficulty: u32) -> bool {
    let prefix_len = difficulty as usize;
    if prefix_len == 0 {
        return true;
    }
    if hash.len() < prefix_len {
        return false;
    }
    hash.chars()
        .take(prefix_len)
        .all(|c| c == '0')
}

/// Recompute hash for a stored block (uses its current fields except `hash` is ignored for input).
pub fn hash_for_block(block: &Block) -> String {
    hash_block(
        block.index,
        block.timestamp,
        block.nonce,
        &block.previous_hash,
        block.difficulty,
        &block.coinbase,
        &block.transactions,
    )
}
