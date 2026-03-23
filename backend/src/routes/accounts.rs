use axum::extract::{Path, State};
use axum::Json;

use crate::app_state::AppState;
use crate::errors::{ApiError, ApiResult};
use crate::models::dto::AccountResponse;

pub async fn list_accounts(State(state): State<AppState>) -> ApiResult<Json<Vec<AccountResponse>>> {
    let s = state.inner.read().await;
    let mut list: Vec<AccountResponse> = s.accounts.values().map(AccountResponse::from).collect();
    list.sort_by_key(|a| a.id);
    Ok(Json(list))
}

pub async fn get_account(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> ApiResult<Json<AccountResponse>> {
    let s = state.inner.read().await;
    let acc = s
        .accounts
        .get(&id)
        .ok_or_else(|| ApiError::NotFound(format!("account {}", id)))?;
    Ok(Json(AccountResponse::from(acc)))
}
