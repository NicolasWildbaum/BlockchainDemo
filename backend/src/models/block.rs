use serde::{Deserialize, Serialize};

use super::transaction::{CoinbaseTx, TransferTx};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub nonce: u64,
    /// freeform block note — hash still folds in txs + coinbase, not just this field
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    #[serde(default)]
    pub transactions: Vec<TransferTx>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coinbase: Option<CoinbaseTx>,
}
