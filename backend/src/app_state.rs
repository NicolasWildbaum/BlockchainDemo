use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::models::account::Account;
use crate::models::block::Block;
use crate::models::blockchain::Blockchain;
use crate::models::transaction::PendingTransaction;
use crate::services::account_service::{apply_block_to_accounts, default_accounts};
use crate::utils::time::now_utc;
use crate::utils::validation::{self, GENESIS_PREVIOUS_HASH};

/// Global demo configuration (in-memory).
#[derive(Debug, Clone)]
pub struct DemoConfig {
    pub block_reward: u64,
    pub default_difficulty: u32,
    pub max_tx_per_block: usize,
    pub initial_balance: u64,
}

impl Default for DemoConfig {
    fn default() -> Self {
        Self {
            block_reward: 50,
            default_difficulty: 3,
            max_tx_per_block: 8,
            initial_balance: 1000,
        }
    }
}

#[derive(Debug)]
pub struct DemoState {
    pub accounts: HashMap<u32, Account>,
    pub blockchain: Blockchain,
    pub mempool: Vec<PendingTransaction>,
    pub config: DemoConfig,
}

impl DemoState {
    pub fn new(config: DemoConfig) -> Self {
        let accounts = default_accounts(config.initial_balance);
        let genesis = Self::genesis_block(config.default_difficulty);
        let mut blockchain = Blockchain::default();
        blockchain.blocks.push(genesis.clone());

        let mut state = Self {
            accounts,
            blockchain,
            mempool: Vec::new(),
            config,
        };
        apply_block_to_accounts(&mut state.accounts, &genesis);
        state
    }

    fn genesis_block(difficulty: u32) -> Block {
        let coinbase = crate::models::block::CoinbaseTx {
            tx_id: "coinbase-0".into(),
            miner_account_id: 1,
            amount: 0,
        };
        let transactions = vec![];
        let timestamp = now_utc();
        let (nonce, hash) = validation::mine_nonce(
            0,
            timestamp,
            GENESIS_PREVIOUS_HASH,
            difficulty,
            &coinbase,
            &transactions,
        );
        Block {
            index: 0,
            timestamp,
            nonce,
            previous_hash: GENESIS_PREVIOUS_HASH.to_string(),
            hash,
            difficulty,
            coinbase,
            transactions,
        }
    }

    pub fn reset(&mut self) {
        *self = DemoState::new(self.config.clone());
    }
}

#[derive(Clone)]
pub struct AppState {
    pub inner: Arc<RwLock<DemoState>>,
}

impl AppState {
    pub fn new(config: DemoConfig) -> Self {
        Self {
            inner: Arc::new(RwLock::new(DemoState::new(config))),
        }
    }
}
