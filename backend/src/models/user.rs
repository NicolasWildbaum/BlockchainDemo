use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub balance: u64,
    /// Ed25519 public key as hex (32 bytes)
    #[serde(rename = "public_key")]
    pub public_key_hex: String,
    /// derived from pub key so the demo has a short thing to show
    pub address: String,
}
