use std::sync::Arc;

use axum::routing::{get, post, put};
use axum::Router;
use tokio::sync::Mutex;

use crate::state::ChainState;

pub mod blockchain;

pub type SharedState = Arc<Mutex<ChainState>>;

pub fn api_router(state: SharedState) -> Router {
    Router::new()
        .route("/blocks", get(blockchain::get_blocks))
        .route("/users", get(blockchain::get_users))
        .route("/mempool", get(blockchain::get_mempool))
        .route("/transactions/pending", get(blockchain::get_mempool))
        .route("/transactions", post(blockchain::post_transaction))
        .route("/blocks/:index", put(blockchain::put_block))
        .route("/blocks/:index/mine", post(blockchain::mine_block))
        .route("/validate", get(blockchain::validate))
        .route("/reset", post(blockchain::reset))
        .with_state(state)
}
