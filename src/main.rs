use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
//use serde::{Deserialize, Serialize};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");


async fn health() -> &'static str {
    "OK"
}

async fn version() -> &'static str {
    VERSION
}

#[tokio::main]
async fn main() {
    env_logger::init();
    log::info!("magnolia starting on 0.0.0.0:3000");

    let app: Router = Router::new().route("/version", get(version)).route("/health", axum::routing::get(health));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
