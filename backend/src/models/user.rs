use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub balance: u64,
    /// Clave pública Ed25519 (hex, 32 bytes).
    #[serde(rename = "public_key")]
    pub public_key_hex: String,
    /// Derivada de la clave pública para la demo.
    pub address: String,
}
