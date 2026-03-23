use crate::errors::ApiError;
use crate::models::block::{Block, CoinbaseTx};
use crate::models::transaction::{ConfirmedTransaction, PendingTransaction};
use crate::services::account_service::{apply_block_to_accounts, balances_snapshot};
use crate::utils::time::now_utc;
use crate::utils::validation::{self, validate_mining_batch, validate_new_block, validate_pending_transaction};

use crate::app_state::DemoState;

/// Picks up to `max_transactions` txs from the mempool in FIFO order (`created_at`), respecting
/// balance + pending-out constraints against the current ledger and the rest of the queue.
pub fn mine_next_block(
    state: &mut DemoState,
    miner_account_id: u32,
    difficulty: u32,
    max_transactions: usize,
) -> Result<(Block, Vec<String>), ApiError> {
    if !state.accounts.contains_key(&miner_account_id) {
        return Err(ApiError::NotFound(format!("account {}", miner_account_id)));
    }

    let max_tx = max_transactions.clamp(1, 64);

    let mut queue: Vec<PendingTransaction> = std::mem::take(&mut state.mempool);
    queue.sort_by_key(|t| t.created_at);

    let mut batch: Vec<PendingTransaction> = Vec::new();
    let mut i = 0usize;
    while i < queue.len() && batch.len() < max_tx {
        let tx = queue[i].clone();
        let pending_context: Vec<PendingTransaction> = batch
            .iter()
            .chain(
                queue
                    .iter()
                    .enumerate()
                    .filter_map(|(j, t)| if j != i { Some(t) } else { None }),
            )
            .cloned()
            .collect();

        match validate_pending_transaction(&tx, &state.accounts, &pending_context) {
            Ok(()) => {
                batch.push(queue.remove(i));
            }
            Err(_) => {
                i += 1;
            }
        }
    }

    state.mempool = queue;

    let tip = state.blockchain.blocks.last();
    let next_index = tip.map(|b| b.index + 1).unwrap_or(0);
    let previous_hash = tip
        .map(|b| b.hash.as_str())
        .unwrap_or(validation::GENESIS_PREVIOUS_HASH)
        .to_string();

    let confirmed: Vec<ConfirmedTransaction> = batch
        .iter()
        .map(|p| ConfirmedTransaction {
            id: p.id.to_string(),
            from_account_id: p.from_account_id,
            to_account_id: p.to_account_id,
            amount: p.amount,
            timestamp: p.created_at,
            block_index: next_index,
        })
        .collect();

    let coinbase = CoinbaseTx {
        tx_id: format!("coinbase-{}", next_index),
        miner_account_id,
        amount: state.config.block_reward,
    };

    let balances = balances_snapshot(&state.accounts);
    validate_mining_batch(
        miner_account_id,
        state.config.block_reward,
        &balances,
        &confirmed,
    )
    .map_err(ApiError::BadRequest)?;

    let timestamp = now_utc();
    let (nonce, hash) = validation::mine_nonce(
        next_index,
        timestamp,
        &previous_hash,
        difficulty,
        &coinbase,
        &confirmed,
    );

    let block = Block {
        index: next_index,
        timestamp,
        nonce,
        previous_hash,
        hash,
        difficulty,
        coinbase,
        transactions: confirmed,
    };

    let tip_ref = state.blockchain.blocks.last();
    validate_new_block(&block, tip_ref).map_err(ApiError::BadRequest)?;

    let included_ids: Vec<String> = block.transactions.iter().map(|t| t.id.clone()).collect();

    state.blockchain.blocks.push(block.clone());
    apply_block_to_accounts(&mut state.accounts, &block);

    Ok((block, included_ids))
}
