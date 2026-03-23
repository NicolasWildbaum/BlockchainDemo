//! Ed25519 stuff for txs — private key signs, public key is enough to verify

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use sha2::{Digest, Sha256};

const PAYLOAD_PREFIX: &str = "TX_V1";

/// bytes we actually sign: from, to, amount, time, id — tweak any field and the sig won't match another tx
pub fn transfer_signing_payload(from: &str, to: &str, amount: u64, timestamp_ms: i64, id: &str) -> Vec<u8> {
    format!(
        "{}|{}|{}|{}|{}|{}",
        PAYLOAD_PREFIX, from, to, amount, timestamp_ms, id
    )
    .into_bytes()
}

pub fn sign_bytes(message: &[u8], key: &SigningKey) -> [u8; Signature::BYTE_SIZE] {
    key.sign(message).to_bytes()
}

pub fn signature_hex_from_signing(message: &[u8], key: &SigningKey) -> String {
    hex::encode(sign_bytes(message, key))
}

pub fn parse_verifying_key_hex(public_key_hex: &str) -> Result<VerifyingKey, String> {
    let bytes = hex::decode(public_key_hex.trim()).map_err(|_| "clave pública inválida (hex)".to_string())?;
    let arr: [u8; 32] = bytes
        .as_slice()
        .try_into()
        .map_err(|_| "clave pública Ed25519 debe ser 32 bytes en hex".to_string())?;
    VerifyingKey::from_bytes(&arr).map_err(|_| "clave pública Ed25519 inválida".to_string())
}

pub fn parse_signature_hex(signature_hex: &str) -> Result<Signature, String> {
    let bytes = hex::decode(signature_hex.trim()).map_err(|_| "firma inválida (hex)".to_string())?;
    let arr: [u8; 64] = bytes
        .as_slice()
        .try_into()
        .map_err(|_| "firma Ed25519 debe ser 64 bytes en hex".to_string())?;
    Ok(Signature::from_bytes(&arr))
}

/// does the sig line up with that exact payload? if not, bail
pub fn verify_transfer_signature(
    from: &str,
    to: &str,
    amount: u64,
    timestamp_ms: i64,
    id: &str,
    public_key_hex: &str,
    signature_hex: &str,
) -> Result<(), String> {
    let vk = parse_verifying_key_hex(public_key_hex)?;
    let sig = parse_signature_hex(signature_hex)?;
    let msg = transfer_signing_payload(from, to, amount, timestamp_ms, id);
    vk.verify(&msg, &sig)
        .map_err(|_| "firma digital inválida (no coincide con el payload ni la clave pública)".to_string())
}

/// short address hashed from the pub key — demo-only, not a real chain format
pub fn demo_address_from_verifying_key(vk: &VerifyingKey) -> String {
    let mut hasher = Sha256::new();
    hasher.update(vk.as_bytes());
    let digest: [u8; 32] = hasher.finalize().into();
    format!("addr_{}", hex::encode(&digest[..8]))
}
