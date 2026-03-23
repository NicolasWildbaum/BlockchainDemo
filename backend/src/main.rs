mod app_state;
mod errors;
mod models;
mod routes;
mod services;
mod utils;

use std::net::SocketAddr;

use axum::Router;
use tower_http::cors::{Any, CorsLayer};

use crate::app_state::{AppState, DemoConfig};
use crate::routes::api_router;

#[tokio::main]
async fn main() {
    let config = DemoConfig::default();
    let state = AppState::new(config);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .nest("/api", api_router())
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind port 3000");
    println!("blockchain_demo API listening on http://{}", addr);
    axum::serve(listener, app).await.expect("server error");
}
