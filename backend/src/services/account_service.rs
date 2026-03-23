use std::collections::HashMap;

use chrono::Utc;

use crate::models::account::{Account, TxHistoryEntry, TxHistoryKind};
use crate::models::block::Block;
use crate::models::transaction::ConfirmedTransaction;

pub fn default_accounts(initial_balance: u64) -> HashMap<u32, Account> {
    let defs: [(u32, &str, &str); 6] = [
        (1, "Alice", "addr_alice"),
        (2, "Bob", "addr_bob"),
        (3, "Charlie", "addr_charlie"),
        (4, "Diana", "addr_diana"),
        (5, "Emma", "addr_emma"),
        (6, "Frank", "addr_frank"),
    ];
    defs
        .into_iter()
        .map(|(id, name, addr)| (id, Account::new(id, name, addr, initial_balance)))
        .collect()
}

pub fn apply_block_to_accounts(accounts: &mut HashMap<u32, Account>, block: &Block) {
    let ts = block.timestamp;

    if block.coinbase.amount > 0 {
        if let Some(miner) = accounts.get_mut(&block.coinbase.miner_account_id) {
            miner.balance = miner.balance.saturating_add(block.coinbase.amount);
            miner.transaction_history.push(TxHistoryEntry {
                tx_id: block.coinbase.tx_id.clone(),
                kind: TxHistoryKind::CoinbaseReward,
                counterparty_id: None,
                amount: block.coinbase.amount,
                block_index: block.index,
                timestamp: ts,
            });
        }
    }

    for tx in &block.transactions {
        apply_confirmed_transfer(accounts, tx, block.index, ts);
    }
}

fn apply_confirmed_transfer(
    accounts: &mut HashMap<u32, Account>,
    tx: &ConfirmedTransaction,
    block_index: u64,
    ts: chrono::DateTime<Utc>,
) {
    if let Some(from) = accounts.get_mut(&tx.from_account_id) {
        from.balance = from.balance.saturating_sub(tx.amount);
        from.transaction_history.push(TxHistoryEntry {
            tx_id: tx.id.clone(),
            kind: TxHistoryKind::Sent,
            counterparty_id: Some(tx.to_account_id),
            amount: tx.amount,
            block_index,
            timestamp: ts,
        });
    }
    if let Some(to) = accounts.get_mut(&tx.to_account_id) {
        to.balance = to.balance.saturating_add(tx.amount);
        to.transaction_history.push(TxHistoryEntry {
            tx_id: tx.id.clone(),
            kind: TxHistoryKind::Received,
            counterparty_id: Some(tx.from_account_id),
            amount: tx.amount,
            block_index,
            timestamp: ts,
        });
    }
}

pub fn balances_snapshot(accounts: &HashMap<u32, Account>) -> HashMap<u32, u64> {
    accounts.iter().map(|(k, v)| (*k, v.balance)).collect()
}
