/*
 * Leara AI Assistant - Main Application Entry Point
 * 
 * This file contains the main entry point for the Leara AI Assistant backend.
 * Sets up the web server, database, and API routes.
 * 
 * Copyright (c) 2024 Leara AI Assistant Contributors
 * 
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * Author: KleaSCM
 * Created: 2024-06-28
 * Last Modified: 2024-06-28
 * Version: 0.1.0
 * 
 * File: src/main.rs
 * Purpose: Main application entry point and server setup
 */

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
