use axum::extract::{Path, State};
use axum::Json;

use crate::app_state::AppState;
use crate::errors::{ApiError, ApiResult};
use crate::models::block::Block;
use crate::models::dto::BlockchainValidateResponse;
use crate::services::blockchain_service::get_block_by_index;
use crate::utils::validation::validate_blockchain_full;

pub async fn list_blocks(State(state): State<AppState>) -> ApiResult<Json<Vec<Block>>> {
    let s = state.inner.read().await;
    Ok(Json(s.blockchain.blocks.clone()))
}

pub async fn get_block(State(state): State<AppState>, Path(index): Path<u64>) -> ApiResult<Json<Block>> {
    let s = state.inner.read().await;
    let b = get_block_by_index(&s.blockchain, index)
        .ok_or_else(|| ApiError::NotFound(format!("block {}", index)))?;
    Ok(Json(b.clone()))
}

pub async fn validate_chain(State(state): State<AppState>) -> Json<BlockchainValidateResponse> {
    let s = state.inner.read().await;
    let (valid, issues) = validate_blockchain_full(&s.blockchain);
    Json(BlockchainValidateResponse { valid, issues })
}

pub async fn reset_demo(State(state): State<AppState>) -> Json<serde_json::Value> {
    let mut s = state.inner.write().await;
    s.reset();
    Json(serde_json::json!({ "ok": true }))
}
