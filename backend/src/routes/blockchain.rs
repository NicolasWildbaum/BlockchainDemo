use std::fmt;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize};

use crate::models::{Block, TransferTx, User};

use super::SharedState;

#[derive(Deserialize)]
pub struct UpdateBlockBody {
    pub data: Option<String>,
    pub nonce: Option<u64>,
}

#[derive(Deserialize, Default)]
pub struct MineQuery {
    #[serde(default)]
    pub miner_id: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateTxBody {
    #[serde(alias = "from")]
    pub sender: String,
    #[serde(alias = "to")]
    pub recipient: String,
    #[serde(deserialize_with = "deserialize_tx_amount")]
    pub amount: u64,
    /// demo knob: flip true to mangle the sig and watch backend/UI say nope
    #[serde(default)]
    pub demo_invalid_signature: bool,
}

fn deserialize_tx_amount<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    struct V;

    impl Visitor<'_> for V {
        type Value = u64;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("entero positivo (monto)")
        }

        fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<u64, E> {
            Ok(v)
        }

        fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<u64, E> {
            u64::try_from(v).map_err(E::custom)
        }

        fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<u64, E> {
            if !v.is_finite() || v < 0.0 {
                return Err(E::custom("monto inválido"));
            }
            Ok(v as u64)
        }

        fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<u64, E> {
            v.trim()
                .parse::<u64>()
                .map_err(|_| E::custom("monto inválido"))
        }

        fn visit_none<E: serde::de::Error>(self) -> Result<u64, E> {
            Err(E::custom("monto requerido"))
        }
    }

    deserializer.deserialize_any(V)
}

#[derive(Serialize)]
pub struct ApiError {
    pub error: String,
}

pub async fn get_blocks(State(state): State<SharedState>) -> Json<Vec<Block>> {
    let s = state.lock().await;
    Json(s.blocks.clone())
}

pub async fn get_users(State(state): State<SharedState>) -> Json<Vec<User>> {
    let s = state.lock().await;
    Json(s.users_snapshot())
}

pub async fn get_mempool(State(state): State<SharedState>) -> Json<Vec<TransferTx>> {
    let s = state.lock().await;
    Json(s.mempool.clone())
}

pub async fn post_transaction(
    State(state): State<SharedState>,
    Json(body): Json<CreateTxBody>,
) -> Result<Json<TransferTx>, (StatusCode, Json<ApiError>)> {
    let mut s = state.lock().await;
    s.add_transaction(
        body.sender,
        body.recipient,
        body.amount,
        body.demo_invalid_signature,
    )
        .map(Json)
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(ApiError { error: e })))
}

pub async fn put_block(
    State(state): State<SharedState>,
    Path(index): Path<usize>,
    Json(body): Json<UpdateBlockBody>,
) -> Result<Json<Block>, (StatusCode, Json<ApiError>)> {
    let mut s = state.lock().await;
    s.update_block(index, body.data, body.nonce).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(ApiError { error: e }),
        )
    })?;
    let b = s.blocks.get(index).cloned().ok_or((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ApiError {
            error: "missing block after update".into(),
        }),
    ))?;
    Ok(Json(b))
}

pub async fn mine_block(
    State(state): State<SharedState>,
    Path(index): Path<usize>,
    Query(q): Query<MineQuery>,
) -> Result<Json<Vec<Block>>, (StatusCode, Json<ApiError>)> {
    let miner = q.miner_id.as_deref().unwrap_or("nico");
    let mut s = state.lock().await;
    s.mine_at(index, miner).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(ApiError { error: e }),
        )
    })?;
    Ok(Json(s.blocks.clone()))
}

pub async fn validate(State(state): State<SharedState>) -> Json<crate::state::ValidationReport> {
    let s = state.lock().await;
    Json(s.validate())
}

pub async fn reset(State(state): State<SharedState>) -> Json<Vec<Block>> {
    let mut s = state.lock().await;
    s.reset();
    Json(s.blocks.clone())
}
