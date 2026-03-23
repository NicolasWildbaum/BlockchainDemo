mod models;
mod routes;
mod state;
mod utils;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};

use crate::routes::{api_router, SharedState};
use crate::state::ChainState;

#[tokio::main]
async fn main() {
    let state: SharedState = Arc::new(Mutex::new(ChainState::new()));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .nest("/api", api_router(state))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind port 3000");
    println!("minimal blockchain API on http://{}", addr);
    axum::serve(listener, app).await.expect("server stopped");
}
