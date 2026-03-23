use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::transaction::ConfirmedTransaction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinbaseTx {
    pub tx_id: String,
    pub miner_account_id: u32,
    pub amount: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub nonce: u64,
    pub previous_hash: String,
    pub hash: String,
    pub difficulty: u32,
    pub coinbase: CoinbaseTx,
    pub transactions: Vec<ConfirmedTransaction>,
}
