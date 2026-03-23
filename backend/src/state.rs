use std::collections::HashMap;

use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use uuid::Uuid;

use crate::models::{Block, CoinbaseTx, TransferTx, User};
use crate::utils::crypto::{
    demo_address_from_verifying_key, signature_hex_from_signing, transfer_signing_payload,
    verify_transfer_signature,
};
use crate::utils::hashing::{
    hash_block, hash_matches_difficulty, mine_nonce, recompute_block_hash, GENESIS_PREVIOUS_HASH,
};

pub const CHAIN_LEN: usize = 5;
pub const COINBASE_REWARD: u64 = 50;
pub const INITIAL_BALANCE: u64 = 1000;

pub struct ChainState {
    pub blocks: Vec<Block>,
    pub mempool: Vec<TransferTx>,
    pub user_catalog: Vec<User>,
    /// Claves privadas demo por usuario (solo backend; conceptualmente el emisor firma con esto).
    pub user_signing_keys: HashMap<String, SigningKey>,
}

impl ChainState {
    pub fn new() -> Self {
        let (user_catalog, user_signing_keys) = build_user_catalog_and_keys();
        Self {
            blocks: build_valid_chain(CHAIN_LEN),
            mempool: Vec::new(),
            user_catalog,
            user_signing_keys,
        }
    }

    pub fn reset(&mut self) {
        let (user_catalog, user_signing_keys) = build_user_catalog_and_keys();
        self.blocks = build_valid_chain(CHAIN_LEN);
        self.mempool.clear();
        self.user_catalog = user_catalog;
        self.user_signing_keys = user_signing_keys;
    }

    pub fn users_snapshot(&self) -> Vec<User> {
        let balances = replay_balances_best_effort(&self.blocks);
        self.user_catalog
            .iter()
            .map(|u| User {
                id: u.id.clone(),
                name: u.name.clone(),
                balance: *balances.get(&u.id).unwrap_or(&0),
                public_key_hex: u.public_key_hex.clone(),
                address: u.address.clone(),
            })
            .collect()
    }

    pub fn update_block(
        &mut self,
        index: usize,
        data: Option<String>,
        nonce: Option<u64>,
    ) -> Result<(), String> {
        let block = self
            .blocks
            .get_mut(index)
            .ok_or_else(|| format!("block index {} out of range", index))?;
        if let Some(d) = data {
            block.data = d;
        }
        if let Some(n) = nonce {
            block.nonce = n;
        }
        recompute_block_hash(block);
        Ok(())
    }

    /// Crea una transferencia firmada en nombre del emisor (demo), verifica firma y saldo, y la encola.
    pub fn add_transaction(
        &mut self,
        from: String,
        to: String,
        amount: u64,
        demo_invalid_signature: bool,
    ) -> Result<TransferTx, String> {
        validate_transfer_shape(&from, &to, amount)?;
        if !self.known_user(&from) {
            return Err(format!("el emisor «{}» no existe", from));
        }
        if !self.known_user(&to) {
            return Err(format!("el receptor «{}» no existe", to));
        }
        let signing_key = self
            .user_signing_keys
            .get(&from)
            .ok_or_else(|| "no hay material de firma demo para el emisor".to_string())?;

        let pending_out: u64 = self
            .mempool
            .iter()
            .filter(|t| t.from == from)
            .map(|t| t.amount)
            .sum();
        let bal = replay_balances_best_effort(&self.blocks);
        let confirmed = bal.get(&from).copied().unwrap_or(0);
        let available = confirmed.saturating_sub(pending_out);
        if available < amount {
            let who = self
                .user_catalog
                .iter()
                .find(|u| u.id == from)
                .map(|u| u.name.as_str())
                .unwrap_or(&from);
            return Err(format!(
                "saldo insuficiente: {} no tiene fondos suficientes para enviar {} (confirmado {}, reservado en mempool {}, disponible {})",
                who, amount, confirmed, pending_out, available
            ));
        }

        let id = Uuid::new_v4().to_string();
        let timestamp_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0);
        let vk = signing_key.verifying_key();
        let public_key_hex = hex::encode(vk.as_bytes());

        let payload = transfer_signing_payload(&from, &to, amount, timestamp_ms, &id);
        let mut signature_hex = signature_hex_from_signing(&payload, signing_key);
        if demo_invalid_signature {
            let mut raw = hex::decode(&signature_hex).map_err(|_| "firma inválida (demo)".to_string())?;
            if raw.is_empty() {
                return Err("firma inválida (demo)".into());
            }
            raw[0] ^= 0xFF;
            signature_hex = hex::encode(raw);
        }

        self.assert_public_key_belongs_to_sender(&from, &public_key_hex)?;

        verify_transfer_signature(
            &from,
            &to,
            amount,
            timestamp_ms,
            &id,
            &public_key_hex,
            &signature_hex,
        )?;

        let tx = TransferTx {
            id,
            from,
            to,
            amount,
            timestamp_ms,
            public_key_hex,
            signature_hex,
            status: "valid_pending".into(),
        };

        self.mempool.push(tx.clone());
        Ok(tx)
    }

    pub fn mine_at(&mut self, index: usize, miner_id: &str) -> Result<(), String> {
        if index >= self.blocks.len() {
            return Err(format!("block index {} out of range", index));
        }

        let mempool_txs: Vec<TransferTx> = std::mem::take(&mut self.mempool);
        let had_pending = !mempool_txs.is_empty();

        if had_pending && !self.known_user(miner_id) {
            self.mempool = mempool_txs;
            return Err(format!("minero desconocido: {}", miner_id));
        }

        for tx in &mempool_txs {
            if let Err(e) = validate_transfer_shape(&tx.from, &tx.to, tx.amount) {
                self.mempool = mempool_txs;
                return Err(e);
            }
            if !self.known_user(&tx.from) || !self.known_user(&tx.to) {
                self.mempool = mempool_txs;
                return Err("una transacción del mempool referencia un usuario inexistente".into());
            }
            if let Err(e) = self.assert_public_key_belongs_to_sender(&tx.from, &tx.public_key_hex) {
                self.mempool = mempool_txs;
                return Err(e);
            }
            if let Err(e) = verify_transfer_signature(
                &tx.from,
                &tx.to,
                tx.amount,
                tx.timestamp_ms,
                &tx.id,
                &tx.public_key_hex,
                &tx.signature_hex,
            ) {
                self.mempool = mempool_txs;
                return Err(e);
            }
        }

        let (final_txs, final_coinbase) = if mempool_txs.is_empty() {
            let cur = &self.blocks[index];
            (cur.transactions.clone(), cur.coinbase.clone())
        } else {
            (
                mempool_txs,
                Some(CoinbaseTx::nicocin(miner_id, COINBASE_REWARD)),
            )
        };

        let mut bal = match replay_balances(&self.blocks[..index]) {
            Ok(b) => b,
            Err(e) => {
                if had_pending {
                    self.mempool = final_txs;
                }
                return Err(e);
            }
        };

        if let Some(ref cb) = final_coinbase {
            if let Err(e) = apply_coinbase(&mut bal, cb) {
                if had_pending {
                    self.mempool = final_txs;
                }
                return Err(e);
            }
        }
        for tx in &final_txs {
            if let Err(e) = apply_transfer(&mut bal, tx) {
                if had_pending {
                    self.mempool = final_txs.clone();
                }
                return Err(e);
            }
        }

        let b = &self.blocks[index];
        let (nonce, hash) = mine_nonce(
            b.index,
            &b.data,
            &b.previous_hash,
            &final_coinbase,
            &final_txs,
        );

        self.blocks[index].nonce = nonce;
        self.blocks[index].hash = hash;
        self.blocks[index].transactions = final_txs;
        self.blocks[index].coinbase = final_coinbase;

        for j in index + 1..self.blocks.len() {
            self.blocks[j].previous_hash = self.blocks[j - 1].hash.clone();
            recompute_block_hash(&mut self.blocks[j]);
        }
        Ok(())
    }

    pub fn validate(&self) -> ValidationReport {
        let mut blocks = Vec::new();
        let mut chain_ok = true;
        let mut ledger = default_balances_map();
        for i in 0..self.blocks.len() {
            let (mut ok, mut reasons) = validate_structural(&self.blocks, i);
            if let Err(e) = apply_block_to_ledger(&mut ledger, &self.blocks[i]) {
                ok = false;
                reasons.push(e);
            }
            if !ok {
                chain_ok = false;
            }
            blocks.push(BlockValidity {
                index: self.blocks[i].index,
                valid: ok,
                reasons,
            });
        }
        ValidationReport {
            chain_valid: chain_ok,
            blocks,
        }
    }

    fn known_user(&self, id: &str) -> bool {
        self.user_catalog.iter().any(|u| u.id == id)
    }

    fn assert_public_key_belongs_to_sender(&self, sender_id: &str, public_key_hex: &str) -> Result<(), String> {
        let u = self
            .user_catalog
            .iter()
            .find(|u| u.id == sender_id)
            .ok_or_else(|| format!("emisor «{}» no encontrado en catálogo", sender_id))?;
        if u.public_key_hex != public_key_hex {
            return Err(
                "la clave pública no corresponde al emisor esperado (registrada en la cuenta)".into(),
            );
        }
        Ok(())
    }
}

fn build_user_catalog_and_keys() -> (Vec<User>, HashMap<String, SigningKey>) {
    let mut users = Vec::new();
    let mut keys = HashMap::new();
    for (id, name) in [
        ("nico", "Nico"),
        ("martin", "Martin"),
        ("sofia", "Sofía"),
    ] {
        let signing_key = SigningKey::generate(&mut OsRng);
        let vk = signing_key.verifying_key();
        let public_key_hex = hex::encode(vk.as_bytes());
        let address = demo_address_from_verifying_key(&vk);
        users.push(User {
            id: id.to_string(),
            name: name.to_string(),
            balance: 0,
            public_key_hex,
            address,
        });
        keys.insert(id.to_string(), signing_key);
    }
    (users, keys)
}

#[derive(serde::Serialize)]
pub struct BlockValidity {
    pub index: u64,
    pub valid: bool,
    pub reasons: Vec<String>,
}

#[derive(serde::Serialize)]
pub struct ValidationReport {
    pub chain_valid: bool,
    pub blocks: Vec<BlockValidity>,
}

fn default_balances_map() -> HashMap<String, u64> {
    ["nico", "martin", "sofia"]
        .into_iter()
        .map(|id| (id.to_string(), INITIAL_BALANCE))
        .collect()
}

fn replay_balances(blocks: &[Block]) -> Result<HashMap<String, u64>, String> {
    let mut ledger = default_balances_map();
    for block in blocks {
        apply_block_to_ledger(&mut ledger, block)?;
    }
    Ok(ledger)
}

fn replay_balances_best_effort(blocks: &[Block]) -> HashMap<String, u64> {
    let mut ledger = default_balances_map();
    for block in blocks {
        if apply_block_to_ledger(&mut ledger, block).is_err() {
            break;
        }
    }
    ledger
}

fn validate_transfer_shape(from: &str, to: &str, amount: u64) -> Result<(), String> {
    if amount == 0 {
        return Err("el monto debe ser mayor que 0".into());
    }
    if from == to {
        return Err("el emisor y el receptor deben ser distintos".into());
    }
    Ok(())
}

fn apply_coinbase(ledger: &mut HashMap<String, u64>, cb: &CoinbaseTx) -> Result<(), String> {
    *ledger.entry(cb.to.clone()).or_insert(0) += cb.amount;
    Ok(())
}

fn apply_transfer(ledger: &mut HashMap<String, u64>, tx: &TransferTx) -> Result<(), String> {
    validate_transfer_shape(&tx.from, &tx.to, tx.amount)?;
    let from = ledger
        .get_mut(&tx.from)
        .ok_or_else(|| format!("emisor desconocido en libro: {}", tx.from))?;
    if *from < tx.amount {
        return Err(format!(
            "saldo insuficiente al aplicar la transacción {} en el bloque",
            tx.id
        ));
    }
    *from -= tx.amount;
    *ledger.entry(tx.to.clone()).or_insert(0) += tx.amount;
    Ok(())
}

fn apply_block_to_ledger(ledger: &mut HashMap<String, u64>, block: &Block) -> Result<(), String> {
    if let Some(ref cb) = block.coinbase {
        apply_coinbase(ledger, cb)?;
    }
    for tx in &block.transactions {
        apply_transfer(ledger, tx)?;
    }
    Ok(())
}

fn validate_structural(blocks: &[Block], i: usize) -> (bool, Vec<String>) {
    let mut reasons = Vec::new();
    let b = &blocks[i];
    let expected = hash_block(
        b.index,
        b.nonce,
        &b.data,
        &b.previous_hash,
        &b.coinbase,
        &b.transactions,
    );
    if expected != b.hash {
        reasons.push("stored hash does not match content".into());
    }
    if !hash_matches_difficulty(&b.hash) {
        reasons.push(format!(
            "hash must start with {}",
            crate::utils::hashing::DIFFICULTY_PREFIX
        ));
    }
    if i == 0 {
        if b.previous_hash != GENESIS_PREVIOUS_HASH {
            reasons.push("genesis previous_hash must be fixed constant".into());
        }
    } else if b.previous_hash != blocks[i - 1].hash {
        reasons.push("previous_hash does not match prior block hash".into());
    }
    (reasons.is_empty(), reasons)
}

fn build_valid_chain(n: usize) -> Vec<Block> {
    let mut out = Vec::with_capacity(n);
    let (n0, h0) = mine_nonce(0, "Genesis", GENESIS_PREVIOUS_HASH, &None, &[]);
    out.push(Block {
        index: 0,
        nonce: n0,
        data: "Genesis".into(),
        previous_hash: GENESIS_PREVIOUS_HASH.into(),
        hash: h0,
        transactions: vec![],
        coinbase: None,
    });
    for i in 1..n {
        let prev_hash = out[i - 1].hash.clone();
        let data = format!("Block {}", i);
        let (ni, hi) = mine_nonce(i as u64, &data, &prev_hash, &None, &[]);
        out.push(Block {
            index: i as u64,
            nonce: ni,
            data,
            previous_hash: prev_hash,
            hash: hi,
            transactions: vec![],
            coinbase: None,
        });
    }
    out
}
