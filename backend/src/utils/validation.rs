use std::collections::{HashMap, HashSet};

use crate::models::block::Block;
use crate::models::blockchain::Blockchain;
use crate::models::transaction::{ConfirmedTransaction, PendingTransaction};
use crate::utils::hashing::{hash_block, hash_for_block, hash_matches_difficulty};

pub const GENESIS_PREVIOUS_HASH: &str =
    "0000000000000000000000000000000000000000000000000000000000000000";

/// Baseline balances used when replaying the chain from genesis (must match demo reset).
pub fn initial_balances_map() -> HashMap<u32, u64> {
    let initial = 1000u64;
    (1..=6).map(|id| (id, initial)).collect()
}

pub fn validate_pending_transaction(
    tx: &PendingTransaction,
    accounts: &HashMap<u32, crate::models::Account>,
    mempool: &[PendingTransaction],
) -> Result<(), String> {
    if tx.amount == 0 {
        return Err("amount must be greater than 0".into());
    }
    if tx.from_account_id == tx.to_account_id {
        return Err("from and to must differ".into());
    }
    if !accounts.contains_key(&tx.from_account_id) {
        return Err(format!("unknown from_account_id {}", tx.from_account_id));
    }
    if !accounts.contains_key(&tx.to_account_id) {
        return Err(format!("unknown to_account_id {}", tx.to_account_id));
    }
    let pending_out: u64 = mempool
        .iter()
        .filter(|p| p.from_account_id == tx.from_account_id && p.id != tx.id)
        .map(|p| p.amount)
        .sum();
    let acc = accounts
        .get(&tx.from_account_id)
        .ok_or_else(|| "from account missing".to_string())?;
    let available = acc.balance.saturating_sub(pending_out);
    if available < tx.amount {
        return Err(format!(
            "insufficient available balance (balance {} pending_out {} need {})",
            acc.balance, pending_out, tx.amount
        ));
    }
    Ok(())
}

pub fn validate_block_pow(block: &Block) -> Result<(), String> {
    if !hash_matches_difficulty(&block.hash, block.difficulty) {
        return Err(format!(
            "proof-of-work failed for block {} (hash does not meet difficulty {})",
            block.index, block.difficulty
        ));
    }
    Ok(())
}

pub fn validate_block_integrity(block: &Block) -> Result<(), String> {
    let recomputed = hash_for_block(block);
    if recomputed != block.hash {
        return Err(format!(
            "stored hash does not match recomputed hash for block {}",
            block.index
        ));
    }
    Ok(())
}

pub fn validate_block_transactions_against_balances(
    block: &Block,
    balances: &mut HashMap<u32, u64>,
) -> Result<(), String> {
    // Apply coinbase first
    let miner = balances
        .get_mut(&block.coinbase.miner_account_id)
        .ok_or_else(|| format!("miner {} missing in replay", block.coinbase.miner_account_id))?;
    *miner = miner.saturating_add(block.coinbase.amount);

    for tx in &block.transactions {
        apply_confirmed_tx_to_balances(tx, balances).map_err(|e| format!("block {}: {}", block.index, e))?;
    }
    Ok(())
}

fn apply_confirmed_tx_to_balances(
    tx: &ConfirmedTransaction,
    balances: &mut HashMap<u32, u64>,
) -> Result<(), String> {
    let from = balances
        .get_mut(&tx.from_account_id)
        .ok_or_else(|| format!("unknown sender {}", tx.from_account_id))?;
    if *from < tx.amount {
        return Err(format!(
            "insufficient balance for tx {} (need {})",
            tx.id, tx.amount
        ));
    }
    *from -= tx.amount;
    let to = balances
        .get_mut(&tx.to_account_id)
        .ok_or_else(|| format!("unknown receiver {}", tx.to_account_id))?;
    *to += tx.amount;
    Ok(())
}

pub fn validate_block_links(block: &Block, previous: Option<&Block>) -> Result<(), String> {
    match previous {
        None => {
            if block.index != 0 {
                return Err("first block must be genesis index 0".into());
            }
            if block.previous_hash != GENESIS_PREVIOUS_HASH {
                return Err("genesis previous_hash must match constant".into());
            }
        }
        Some(prev) => {
            if block.index != prev.index + 1 {
                return Err(format!(
                    "block index discontinuity: expected {} got {}",
                    prev.index + 1,
                    block.index
                ));
            }
            if block.previous_hash != prev.hash {
                return Err(format!(
                    "block {} previous_hash does not match previous block hash",
                    block.index
                ));
            }
        }
    }
    Ok(())
}

/// Full structural validation: links, PoW, hash integrity, no duplicate tx ids inside a block.
pub fn validate_block_structure(block: &Block, previous: Option<&Block>) -> Result<(), String> {
    validate_block_links(block, previous)?;
    validate_block_pow(block)?;
    validate_block_integrity(block)?;

    let mut ids: HashSet<&str> = HashSet::new();
    if !ids.insert(block.coinbase.tx_id.as_str()) {
        return Err("duplicate coinbase tx_id".into());
    }
    for tx in &block.transactions {
        if !ids.insert(tx.id.as_str()) {
            return Err(format!("duplicate transaction id in block: {}", tx.id));
        }
    }
    Ok(())
}

/// Validates the whole chain: structure per block + economic replay from initial balances.
pub fn validate_blockchain_full(blockchain: &Blockchain) -> (bool, Vec<String>) {
    let mut issues = Vec::new();
    let mut balances = initial_balances_map();

    if blockchain.blocks.is_empty() {
        issues.push("chain has no blocks".into());
        return (false, issues);
    }

    for (i, block) in blockchain.blocks.iter().enumerate() {
        let prev = i.checked_sub(1).and_then(|j| blockchain.blocks.get(j));
        if let Err(e) = validate_block_structure(block, prev) {
            issues.push(e);
        }
        if let Err(e) = validate_block_transactions_against_balances(block, &mut balances) {
            issues.push(e);
        }
    }

    (issues.is_empty(), issues)
}

/// Validate a candidate block before appending (links against current tip, PoW, integrity).
pub fn validate_new_block(block: &Block, tip: Option<&Block>) -> Result<(), String> {
    validate_block_structure(block, tip)
}

/// Checks that selected mempool txs can be applied in order on top of current account balances
/// after hypothetically applying the coinbase first.
pub fn validate_mining_batch(
    miner_id: u32,
    block_reward: u64,
    current_balances: &HashMap<u32, u64>,
    txs: &[ConfirmedTransaction],
) -> Result<(), String> {
    let mut b = current_balances.clone();
    let m = b.get_mut(&miner_id).ok_or_else(|| format!("unknown miner {}", miner_id))?;
    *m = m.saturating_add(block_reward);
    for tx in txs {
        apply_confirmed_tx_to_balances(tx, &mut b)?;
    }
    Ok(())
}

/// Recompute hash with nonce (used by mining loop).
pub fn mine_nonce(
    index: u64,
    timestamp: chrono::DateTime<chrono::Utc>,
    previous_hash: &str,
    difficulty: u32,
    coinbase: &crate::models::block::CoinbaseTx,
    transactions: &[ConfirmedTransaction],
) -> (u64, String) {
    let mut nonce = 0u64;
    loop {
        let h = hash_block(
            index,
            timestamp,
            nonce,
            previous_hash,
            difficulty,
            coinbase,
            transactions,
        );
        if hash_matches_difficulty(&h, difficulty) {
            return (nonce, h);
        }
        nonce += 1;
    }
}
