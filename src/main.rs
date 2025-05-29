use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize)]
struct Info { version: String }

#[derive(Deserialize)]
struct Markdown { md: String }

#[derive(Serialize)]
struct HtmlOut { html: String }

// return version
async fn info() -> Json<Info> {
    Json(Info{ version: VERSION.to_string() })
}

// convert markdown to HTML
async fn convertmd(Json(Markdown { md}): Json<Markdown>
) -> Json<HtmlOut> {
    let html = markdown::to_html(&md);
    log::info!("Received markdown conversion request, converted result: {}", html);
    Json(HtmlOut { html })
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_default_env()
    .format_timestamp_secs()
    .filter_level(log::LevelFilter::Info)
    .init();
    log::info!("magnolia starting on 0.0.0.0:3000");

    // init database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL")
        .expect("DATABASE_URL not set"))
        .await
        .expect("failed to connect to Postgres");

    let app = Router::new()
    .route("/info", get(info))
    .route("/render", post(convertmd))
    .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}