use serde::{Deserialize, Serialize};

use super::account::Account;
use super::block::Block;
use super::transaction::PendingTransaction;

#[derive(Debug, Deserialize)]
pub struct CreateTransactionRequest {
    pub from_account_id: u32,
    pub to_account_id: u32,
    pub amount: u64,
}

#[derive(Debug, Deserialize)]
pub struct MineRequest {
    pub miner_account_id: u32,
    #[serde(default)]
    pub difficulty: Option<u32>,
    #[serde(default)]
    pub max_transactions: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct PendingTransactionResponse {
    pub id: String,
    pub from_account_id: u32,
    pub to_account_id: u32,
    pub amount: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<&PendingTransaction> for PendingTransactionResponse {
    fn from(t: &PendingTransaction) -> Self {
        Self {
            id: t.id.to_string(),
            from_account_id: t.from_account_id,
            to_account_id: t.to_account_id,
            amount: t.amount,
            created_at: t.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AccountResponse {
    pub id: u32,
    pub name: String,
    pub balance: u64,
    pub address: String,
    pub transaction_history: Vec<super::account::TxHistoryEntry>,
}

impl From<&Account> for AccountResponse {
    fn from(a: &Account) -> Self {
        Self {
            id: a.id,
            name: a.name.clone(),
            balance: a.balance,
            address: a.address.clone(),
            transaction_history: a.transaction_history.clone(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct BlockchainValidateResponse {
    pub valid: bool,
    pub issues: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct MineResponse {
    pub block: Block,
    pub included_transaction_ids: Vec<String>,
}
