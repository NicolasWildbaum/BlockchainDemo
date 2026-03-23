use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingTransaction {
    pub id: Uuid,
    pub from_account_id: u32,
    pub to_account_id: u32,
    pub amount: u64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmedTransaction {
    pub id: String,
    pub from_account_id: u32,
    pub to_account_id: u32,
    pub amount: u64,
    pub timestamp: DateTime<Utc>,
    pub block_index: u64,
}
