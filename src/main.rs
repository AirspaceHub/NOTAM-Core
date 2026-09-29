mod state;
mod database;

use state::AppState;
use database::db;
use std::{
    sync::Arc,
    env,
};

use axum::{
    routing::get,
    http::StatusCode,
    Router,
};

#[tokio::main]
async fn main() {
    let db_url: String = env::var("TOASTY_CONNECTION_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());

    let db = db::connect(&db_url).await.expect("Failed to connect to database");

    let state = AppState {
        db: Arc::new(db),
    };

    let app = Router::new()
    .route("/", get(root));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("Failed to bind to address");
    axum::serve(listener, app).await.expect("Failed to serve application");
}

async fn root () -> (StatusCode, String) {
    (StatusCode::OK, "Hello, World!".to_string())
}