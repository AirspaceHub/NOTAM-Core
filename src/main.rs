use axum::{
    routing::get,
    http::StatusCode,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(root));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("Failed to bind to address");
    axum::serve(listener, app).await.expect("Failed to serve application");
}

async fn root () -> (StatusCode, String) {
    (StatusCode::OK, "Hello, World!".to_string())
}