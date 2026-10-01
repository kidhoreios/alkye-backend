mod db;
mod error;
mod handlers;
mod models;

use std::sync::{Arc, Mutex};

use axum::{Router, routing::get};
use tower_http::cors::CorsLayer;

pub type AppState = Arc<Mutex<rusqlite::Connection>>;

#[tokio::main]
async fn main() {
    let db_path = std::env::var("DATABASE_PATH").unwrap_or_else(|_| "articles.db".into());
    let conn = db::open(&db_path).expect("failed to open SQLite database");
    let state: AppState = Arc::new(Mutex::new(conn));

    let app = Router::new()
        .route("/articles", get(handlers::list_articles))
        .route("/articles/{id}", get(handlers::get_article))
        .fallback(handlers::not_found)
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Hosting platforms (Render, Railway, Fly) inject PORT; BIND_ADDR still wins if set.
    let addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| {
        let port = std::env::var("PORT").unwrap_or_else(|_| "3000".into());
        format!("0.0.0.0:{port}")
    });
    let listener = tokio::net::TcpListener::bind(&addr).await.expect("failed to bind");
    println!("API listening on http://{addr}");
    axum::serve(listener, app).await.expect("server error");
}
