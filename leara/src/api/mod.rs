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
/// * `Router` - Configured Axum router with all API endpoints
pub fn create_router() -> Router {
    Router::new()
        // Health check endpoint
        .route("/health", get(health_check))
        
        // Chat endpoints
        .route("/chat", post(handle_chat))
        .route("/chat/memory", post(handle_memory_query))
        .route("/chat/summary", post(get_conversation_summary))
        
        // System information endpoint
        .route("/system", get(get_system_info))
        
        // Memory management endpoints
        .route("/memory", get(get_memory))
        .route("/memory", post(store_memory))
        .route("/memory/search", post(search_memories))
        .route("/memory/summary", get(get_memory_summary))
        
        // Task management endpoints
        .route("/tasks", get(get_tasks))
        .route("/tasks", post(create_task))
        .route("/tasks/:id/status", put(update_task_status))
        
        // Session context endpoints
        .route("/context", post(store_session_context))
        .route("/context/:session_id", get(get_session_context))
} 