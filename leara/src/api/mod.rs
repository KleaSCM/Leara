/*
 * Leara AI Assistant - API Module
 * 
 * This module contains all API route handlers and related functionality.
 * Provides endpoints for chat, system info, memory, and health checks.
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
 * File: src/api/mod.rs
 * Purpose: API module organization and exports
 */

use axum::{
    routing::{get, post, put},
    Router,
};

use crate::models::AppState;

pub mod health;
pub mod chat;
pub mod system;
pub mod memory;

use health::health_check;
use chat::{handle_chat, handle_memory_query, get_conversation_summary};
use system::get_system_info;
use memory::{
    get_memory, store_memory, create_task, get_tasks, update_task_status,
    search_memories, get_memory_summary, store_session_context, get_session_context,
};

/// Create the main API router with all endpoints
/// 
/// This function sets up all the API routes for the Leara AI Assistant,
/// including chat, memory, system, and health endpoints.
/// 
/// # Returns
/// * `Router<AppState>` - Configured Axum router with all API endpoints
pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/health", health::create_router())
        .nest("/chat", chat::create_router())
        .nest("/memory", memory::create_router())
        .nest("/system", system::create_router())
} 