/*
 * Leara AI Assistant - Memory API Handler
 * 
 * This module handles memory-related API endpoints for storing and retrieving
 * conversation history, user preferences, tasks, and context.
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
 * File: src/api/memory.rs
 * Purpose: Memory API endpoint handlers
 */

// Import Axum web framework components for HTTP handling
use axum::{
    extract::{Json, Query, Path, State},
    http::StatusCode,
};
// Import Serde for JSON serialization/deserialization
use serde::{Deserialize, Serialize};
// Import our local models and services
use crate::models::memory::*;
use crate::system::MemoryService;
use crate::db::queries::*;
use crate::models::AppState;
// Import tracing for structured logging
use tracing::{info, error};

/// Request structure for storing memory entries
/// Contains the key-value pair and optional metadata for persistent storage
#[derive(Debug, Deserialize)]
pub struct MemoryRequest {
    /// Unique identifier for the memory entry (e.g., "user_preferences", "conversation_context")
    pub key: String,
    /// The actual data to be stored (can be JSON, text, or any serializable content)
    pub value: String,
    /// Optional metadata for additional context (e.g., timestamps, categories, tags)
    pub metadata: Option<serde_json::Value>,
    /// Optional context information
    pub context: Option<String>,
    /// Optional priority level (1-5)
    pub priority: Option<i32>,
    /// Optional category for the memory entry
    pub category: Option<String>,
    /// Optional expiration date for the memory entry
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Request structure for searching memories
#[derive(Debug, Deserialize)]
pub struct MemorySearchRequest {
    /// Search query string
    pub query: String,
}

/// Response structure for memory operations
/// Provides standardized success/failure feedback to clients
#[derive(Debug, Serialize)]
pub struct MemoryOperationResponse {
    /// Whether the operation completed successfully
    pub success: bool,
    /// Human-readable message describing the operation result
    pub message: String,
}

/// Error response structure for memory API failures
/// Provides detailed error information when memory operations fail
#[derive(Debug, Serialize)]
pub struct MemoryError {
    /// Human-readable error message explaining what went wrong
    pub error: String,
}

/// Retrieve all memory entries from the database
/// 
/// This function fetches all stored memory entries and returns them as a list.
/// Supports filtering and pagination.
/// 
/// # Returns
/// * `Ok(Json<MemoryResponse>)` - Successfully retrieved memory entries
/// * `Err((StatusCode, Json<MemoryError>))` - Error response with appropriate HTTP status
pub async fn get_memory(
    State(state): State<AppState>,
    Query(query): Query<MemoryQuery>,
) -> Result<Json<MemoryResponse>, (StatusCode, Json<MemoryError>)> {
    let db = state.db.get().unwrap();
    match crate::db::queries::get_enhanced_memories(&db, &query) {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(MemoryError { error: e.to_string() }))),
    }
}

/// Store a new memory entry in the database
/// 
/// This function persists user data, conversation context, or system preferences
/// to the database for later retrieval. Supports key-value storage with optional metadata.
/// 
/// # Arguments
/// * `payload` - The deserialized memory request containing key, value, and optional metadata
/// 
/// # Returns
/// * `Ok(Json<MemoryOperationResponse>)` - Successfully stored memory entry
/// * `Err((StatusCode, Json<MemoryError>))` - Error response with appropriate HTTP status
pub async fn store_memory(
    State(state): State<AppState>,
    Json(payload): Json<MemoryRequest>,
) -> Result<Json<MemoryOperationResponse>, (StatusCode, Json<MemoryError>)> {
    let db = state.db.get().unwrap();
    let memory = Memory {
        id: 0,
        key: payload.key.clone(),
        value: payload.value.clone(),
        category: payload.category.unwrap_or_else(|| "general".to_string()),
        priority: payload.priority.unwrap_or(3),
        metadata: payload.metadata.clone(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        expires_at: payload.expires_at,
        is_active: true,
    };
    match crate::db::queries::insert_enhanced_memory(&db, &memory) {
        Ok(_) => Ok(Json(MemoryOperationResponse {
            success: true,
            message: format!("Stored memory for key: {}", payload.key),
        })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(MemoryError { error: e.to_string() }))),
    }
}

/// Create a task from natural language input
/// 
/// This endpoint allows users to create tasks using natural language,
/// which will be automatically parsed and categorized.
/// 
/// # Arguments
/// * `payload` - Task creation request with natural language description
/// 
/// # Returns
/// * `Ok(Json<Task>)` - Successfully created task
/// * `Err((StatusCode, Json<MemoryError>))` - Error response
pub async fn create_task(
    State(state): State<AppState>,
    Json(payload): Json<TaskRequest>,
) -> Result<Json<Task>, (StatusCode, Json<MemoryError>)> {
    let db = state.db.get().unwrap();
    let task = Task {
        id: 0,
        title: payload.title.clone(),
        description: payload.description.clone(),
        status: "pending".to_string(),
        priority: payload.priority.unwrap_or(3),
        due_date: payload.due_date,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        completed_at: None,
        context: payload.context.clone(),
        tags: payload.tags.clone(),
    };
    match crate::db::queries::insert_task(&db, &task) {
        Ok(_) => Ok(Json(task)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(MemoryError { error: e.to_string() }))),
    }
}

/// Get all tasks with optional filtering
/// 
/// # Arguments
/// * `query` - Query parameters for filtering tasks
/// 
/// # Returns
/// * `Ok(Json<TaskResponse>)` - Tasks and total count
/// * `Err((StatusCode, Json<MemoryError>))` - Error response
pub async fn get_tasks(
    State(state): State<AppState>,
    Query(query): Query<TaskQuery>,
) -> Result<Json<TaskResponse>, (StatusCode, Json<MemoryError>)> {
    let db = state.db.get().unwrap();
    match crate::db::queries::get_tasks(&db, &query) {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(MemoryError { error: e.to_string() }))),
    }
}

/// Update task status
/// 
/// # Arguments
/// * `task_id` - ID of the task to update
/// * `payload` - Status update request
/// 
/// # Returns
/// * `Ok(Json<MemoryOperationResponse>)` - Success response
/// * `Err((StatusCode, Json<MemoryError>))` - Error response
pub async fn update_task_status(
    State(state): State<AppState>,
    Path(task_id): Path<i64>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<MemoryOperationResponse>, (StatusCode, Json<MemoryError>)> {
    let db = state.db.get().unwrap();
    let status = payload.get("status").and_then(|v| v.as_str()).unwrap_or("pending");
    match crate::db::queries::update_task_status(&db, task_id, status) {
        Ok(_) => Ok(Json(MemoryOperationResponse {
            success: true,
            message: format!("Updated status for task {}", task_id),
        })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(MemoryError { error: e.to_string() }))),
    }
}

/// Search memories using natural language query
/// 
/// This endpoint allows users to search for memories using natural language,
/// which will be processed to find relevant stored information.
/// 
/// # Arguments
/// * `payload` - Search request with natural language query
/// 
/// # Returns
/// * `Ok(Json<MemoryResponse>)` - Relevant memories
/// * `Err((StatusCode, Json<MemoryError>))` - Error response
pub async fn search_memories(
    State(state): State<AppState>,
    Json(payload): Json<MemorySearchRequest>,
) -> Result<Json<MemoryResponse>, (StatusCode, Json<MemoryError>)> {
    let db = state.db.get().unwrap();
    match crate::db::queries::search_memories(&db, &payload.query) {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(MemoryError { error: e.to_string() }))),
    }
}

/// Get a summary of all stored memories
/// 
/// This endpoint provides a high-level overview of all stored memories,
/// useful for understanding the current state of the memory system.
/// 
/// # Returns
/// * `Ok(Json<serde_json::Value>)` - Memory summary
/// * `Err((StatusCode, Json<MemoryError>))` - Error response
pub async fn get_memory_summary(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<MemoryError>)> {
    let db = state.db.get().unwrap();
    match crate::db::queries::get_memory_summary(&db) {
        Ok(summary) => Ok(Json(summary)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(MemoryError { error: e.to_string() }))),
    }
}

/// Store session context information
/// 
/// This endpoint allows storing context information for a specific session,
/// which can be used to maintain conversation state and user preferences.
/// 
/// # Arguments
/// * `payload` - Session context request with session ID and context data
/// 
/// # Returns
/// * `Ok(Json<MemoryOperationResponse>)` - Successfully stored context
/// * `Err((StatusCode, Json<MemoryError>))` - Error response
pub async fn store_session_context(
    State(state): State<AppState>,
    Json(payload): Json<SessionContextRequest>,
) -> Result<Json<MemoryOperationResponse>, (StatusCode, Json<MemoryError>)> {
    let db = state.db.get().unwrap();
    match crate::db::queries::store_session_context(&db, &payload) {
        Ok(_) => Ok(Json(MemoryOperationResponse {
            success: true,
            message: "Session context stored".to_string(),
        })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(MemoryError { error: e.to_string() }))),
    }
}

/// Retrieve session context information
/// 
/// This endpoint retrieves all context information for a specific session,
/// allowing the system to restore conversation state and user preferences.
/// 
/// # Arguments
/// * `session_id` - The session identifier to retrieve context for
/// 
/// # Returns
/// * `Ok(Json<SessionContextResponse>)` - Session context data
/// * `Err((StatusCode, Json<MemoryError>))` - Error response
pub async fn get_session_context(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
) -> Result<Json<SessionContextResponse>, (StatusCode, Json<MemoryError>)> {
    let db = state.db.get().unwrap();
    match crate::db::queries::get_session_context(&db, &session_id) {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(MemoryError { error: e.to_string() }))),
    }
} 