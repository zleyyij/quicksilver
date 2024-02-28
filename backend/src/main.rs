mod db;
mod handlers;
mod util;

use anyhow::Result;
use axum::{
    http::Method,
    routing::get,
    Router,
};
use db::init_db;
use handlers::*;
use log::{info, Level, LevelFilter, Metadata, Record};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tower_http::cors::{self, CorsLayer};

#[derive(Clone)]
pub struct AppState {
    db_connection_pool: SqlitePool,
}

/// https://docs.rs/log/latest/log/#implementing-a-logger
struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            // TODO: use buffered output because the current system is not efficient at all
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, sqlx::Type)]
pub enum PasteCategory {
    Markdown,
    Plaintext,
    Html,
    File,
    Url,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, sqlx::FromRow)]
pub struct Paste {
    id: String,
    category: PasteCategory,
    contents: Vec<u8>,
    /// ISO8601/RFC3339 string
    date: String,
    /// how long until the paste expires in seconds
    duration: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    // initialize logging
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
        .unwrap();
    // spawn a global app state instance
    let state = AppState {
        db_connection_pool: init_db(db::DATABASE_URL).await?,
    };
    // assemble our application
    let app = Router::new()
        .route("/pastes", get(get_paste).post(post_paste))
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
                .allow_headers(cors::Any)
                .allow_origin(cors::Any),
        )
        .with_state(state);
    let port = "3000";
    info!("Application started, listening on port: {}", port);
    // serve our application
    let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
