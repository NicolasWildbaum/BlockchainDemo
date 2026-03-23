use axum::routing::{get, post};
use axum::Router;

use crate::app_state::AppState;

mod accounts;
mod blockchain;
mod health;
mod mining;
mod transactions;

pub fn api_router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health))
        .route("/accounts", get(accounts::list_accounts))
        .route("/accounts/:id", get(accounts::get_account))
        .route("/transactions/pending", get(transactions::list_pending))
        .route("/transactions", post(transactions::create_transaction))
        .route("/blocks", get(blockchain::list_blocks))
        .route("/blocks/:index", get(blockchain::get_block))
        .route("/mine", post(mining::mine))
        .route("/blockchain/validate", get(blockchain::validate_chain))
        .route("/reset-demo", post(blockchain::reset_demo))
}
