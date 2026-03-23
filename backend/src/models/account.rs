use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TxHistoryKind {
    Sent,
    Received,
    CoinbaseReward,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxHistoryEntry {
    pub tx_id: String,
    pub kind: TxHistoryKind,
    /// Other party for transfers; miner account for coinbase context.
    pub counterparty_id: Option<u32>,
    pub amount: u64,
    pub block_index: u64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: u32,
    pub name: String,
    pub balance: u64,
    pub address: String,
    pub transaction_history: Vec<TxHistoryEntry>,
}

impl Account {
    pub fn new(id: u32, name: impl Into<String>, address: impl Into<String>, initial_balance: u64) -> Self {
        Self {
            id,
            name: name.into(),
            balance: initial_balance,
            address: address.into(),
            transaction_history: Vec::new(),
        }
    }
}
