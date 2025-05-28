use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize)]
struct Info {
    version: String,
}

#[derive(Deserialize)]
struct Page {
    title: String,
    body: String
}

// return version
async fn info() -> Json<Info> {
    Json(Info{ version: VERSION.to_string() })
}

// temporary test draft fn
async fn draft(Json(Page { title, body }): Json<Page>) -> StatusCode {
    log::info!("Received draft: title = {}, body = {}", title, body);
    StatusCode::OK
}


#[tokio::main]
async fn main() {
    env_logger::Builder::from_default_env()
    .filter_level(log::LevelFilter::Info)
    .init();
    log::info!("magnolia starting on 0.0.0.0:3000");

    let app = Router::new().route("/info", get(info)).route("/draft", post(draft));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

