use axum::extract::State;
use axum::Json;

use crate::app_state::AppState;
use crate::errors::ApiResult;
use crate::models::dto::{MineRequest, MineResponse};
use crate::services::mining_service::mine_next_block;

pub async fn mine(State(state): State<AppState>, Json(body): Json<MineRequest>) -> ApiResult<Json<MineResponse>> {
    let mut s = state.inner.write().await;
    let difficulty = body
        .difficulty
        .unwrap_or(s.config.default_difficulty)
        .clamp(1, 8);
    let max_tx = body
        .max_transactions
        .unwrap_or(s.config.max_tx_per_block)
        .clamp(1, 64);
    let (block, included_transaction_ids) = mine_next_block(&mut s, body.miner_account_id, difficulty, max_tx)?;
    Ok(Json(MineResponse {
        block,
        included_transaction_ids,
    }))
}
