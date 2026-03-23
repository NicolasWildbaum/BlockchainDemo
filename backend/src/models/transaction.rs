use serde::{Deserialize, Serialize};

/// user-to-user transfer with a sig (payload is basically from, to, amount, time, id)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TransferTx {
    pub id: String,
    #[serde(rename = "sender")]
    pub from: String,
    #[serde(rename = "recipient")]
    pub to: String,
    pub amount: u64,
    pub timestamp_ms: i64,
    #[serde(rename = "public_key")]
    pub public_key_hex: String,
    pub signature_hex: String,
    /// in mempool usually `valid_pending` — mostly eye candy for the UI
    pub status: String,
}

/// block reward the protocol mints for the miner — no user signed this
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CoinbaseTx {
    pub label: String,
    #[serde(rename = "sender")]
    pub from: String,
    #[serde(rename = "recipient")]
    pub to: String,
    pub amount: u64,
    /// true = protocol emission, not a user signature situation
    #[serde(default)]
    pub is_protocol_coinbase: bool,
}

impl CoinbaseTx {
    pub fn nicocin(to_user_id: impl Into<String>, amount: u64) -> Self {
        Self {
            label: "NICOCIN".into(),
            from: "NICOCIN".into(),
            to: to_user_id.into(),
            amount,
            is_protocol_coinbase: true,
        }
    }
}
