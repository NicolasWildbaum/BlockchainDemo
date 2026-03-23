use serde::{Deserialize, Serialize};

/// Transacción firmada entre usuarios. El payload firmado incluye emisor, receptor, monto, tiempo e id.
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
    /// En mempool: típicamente `valid_pending`. Solo informativo para la UI.
    pub status: String,
}

/// Emisión especial del protocolo (recompensa al minero). **No** es una transferencia firmada por un usuario.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CoinbaseTx {
    pub label: String,
    #[serde(rename = "sender")]
    pub from: String,
    #[serde(rename = "recipient")]
    pub to: String,
    pub amount: u64,
    /// Si es true, esta entrada es emisión del protocolo (no firma de usuario).
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
