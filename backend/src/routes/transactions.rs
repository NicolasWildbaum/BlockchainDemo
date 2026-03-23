use axum::extract::State;
use axum::Json;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::errors::{ApiError, ApiResult};
use crate::models::dto::{CreateTransactionRequest, PendingTransactionResponse};
use crate::models::transaction::PendingTransaction;
use crate::utils::time::now_utc;
use crate::utils::validation::validate_pending_transaction;

pub async fn list_pending(State(state): State<AppState>) -> ApiResult<Json<Vec<PendingTransactionResponse>>> {
    let s = state.inner.read().await;
    let list: Vec<PendingTransactionResponse> = s.mempool.iter().map(PendingTransactionResponse::from).collect();
    Ok(Json(list))
}

pub async fn create_transaction(
    State(state): State<AppState>,
    Json(body): Json<CreateTransactionRequest>,
) -> ApiResult<Json<PendingTransactionResponse>> {
    let mut s = state.inner.write().await;
    let tx = PendingTransaction {
        id: Uuid::new_v4(),
        from_account_id: body.from_account_id,
        to_account_id: body.to_account_id,
        amount: body.amount,
        created_at: now_utc(),
    };
    validate_pending_transaction(&tx, &s.accounts, &s.mempool).map_err(ApiError::BadRequest)?;
    s.mempool.push(tx);
    let last = s.mempool.last().expect("just pushed");
    Ok(Json(PendingTransactionResponse::from(last)))
}
