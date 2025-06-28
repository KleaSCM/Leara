use axum::{
    routing::{get, post},
    Router,
    http::Method,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tokio::net::TcpListener;

mod api;
mod db;
mod system;
mod models;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    info!("Starting Leara AI Assistant Backend...");

    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize database
    let db_path = std::env::var("DATABASE_PATH").unwrap_or_else(|_| "data/leara.db".to_string());
    db::init_database(&db_path).await?;
    info!("Database initialized at: {}", db_path);

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    // Create router with API routes
    let app = Router::new()
        .route("/health", get(api::health::health_check))
        .route("/api/chat", post(api::chat::handle_chat))
        .route("/api/system/info", get(api::system::get_system_info))
        .route("/api/memory", get(api::memory::get_memory))
        .route("/api/memory", post(api::memory::store_memory))
        .layer(cors);

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    info!("Server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
