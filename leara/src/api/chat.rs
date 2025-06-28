/*
 * Leara AI Assistant - Chat API Handler
 * 
 * This module handles chat-related API endpoints for the AI assistant.
 * Processes user messages and manages conversation state with memory integration.
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
 * File: src/api/chat.rs
 * Purpose: Chat API endpoint handlers with memory integration
 */

// Import Axum web framework components for HTTP handling
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::Json as JsonResponse,
    Router,
    routing::post,
};
// Import Serde for JSON serialization/deserialization
use serde::{Deserialize, Serialize};
// Import our local chat models
use crate::models::chat::ChatResponse;
// Import tracing for structured logging
use tracing::{info, error};
// Import our Ollama client for AI model integration
use crate::utils::ollama::{OllamaClient, OllamaOptions};
// Import uuid for generating conversation IDs
use uuid;
// Import our AppState
use crate::models::AppState;
use chrono::Utc;

/// Request structure for incoming chat messages
/// Contains the user's message and optional context information
#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    /// The main message content from the user
    pub message: String,
    /// Optional context or metadata about the conversation
    pub context: Option<String>,
    /// Optional session ID for conversation continuity
    pub session_id: Option<String>,
}

/// Error response structure for chat API failures
/// Provides standardized error information to clients
#[derive(Debug, Serialize)]
pub struct ChatError {
    /// Human-readable error message
    pub error: String,
}

/// Handle incoming chat messages from clients
/// 
/// This function processes user messages and generates appropriate responses
/// using the hexbenjamin/memgpt-dpo-uncensored:f16 model via Ollama.
/// It also integrates with the memory system to remember conversations,
/// create tasks, and provide context-aware responses.
/// 
/// # Arguments
/// * `payload` - The deserialized chat request containing user message and context
/// 
/// # Returns
/// * `Ok(JsonResponse<ChatResponse>)` - Successfully processed chat response
/// * `Err((StatusCode, Json<ChatError>))` - Error response with appropriate HTTP status
/// 
/// # Example
/// ```rust
/// // Client sends: {"message": "remind me to make changes to system.rs", "context": "leara project"}
/// // Server responds with AI-generated response and creates a task
/// ```
pub async fn handle_chat(
    State(state): State<AppState>,
    Json(payload): Json<ChatRequest>,
) -> Result<JsonResponse<ChatResponse>, (StatusCode, Json<ChatError>)> {
    // Example: Search for tasks and memories containing keywords from the message
    let db = state.db.get().unwrap();
    let mut found_tasks = Vec::new();
    let mut found_memories = Vec::new();

    // Search tasks
    if let Ok(task_response) = crate::db::queries::get_tasks(&db, &crate::models::memory::TaskQuery {
        status: None,
        priority: None,
        limit: Some(5),
        offset: Some(0),
        include_completed: Some(false),
    }) {
        for task in task_response.tasks.iter() {
            if payload.message.to_lowercase().contains(&task.title.to_lowercase()) {
                found_tasks.push(task.title.clone());
            }
        }
    }

    // Search memories
    if let Ok(memory_response) = crate::db::queries::get_enhanced_memories(&db, &crate::models::memory::MemoryQuery {
        key: None,
        category: None,
        priority: None,
        limit: Some(5),
        offset: Some(0),
        include_expired: Some(false),
    }) {
        for memory in memory_response.memories.iter() {
            if payload.message.to_lowercase().contains(&memory.key.to_lowercase()) {
                found_memories.push(memory.key.clone());
            }
        }
    }

    let mut response_text = String::new();
    if !found_tasks.is_empty() {
        response_text.push_str(&format!("I found these tasks related to your message: {}\n", found_tasks.join(", ")));
    }
    if !found_memories.is_empty() {
        response_text.push_str(&format!("I found these memories related to your message: {}\n", found_memories.join(", ")));
    }
    if response_text.is_empty() {
        response_text = "I searched your tasks and memories but didn't find anything directly related. Please provide more details or create a new task or memory if needed.".to_string();
    }

    let response = ChatResponse {
        message: response_text,
        conversation_id: uuid::Uuid::new_v4(),
        timestamp: chrono::Utc::now(),
        context: payload.context,
    };

    Ok(JsonResponse(response))
}

/// Handle memory-related chat queries
/// 
/// This function specifically handles queries about stored memories,
/// tasks, and context information.
/// 
/// # Arguments
/// * `payload` - The deserialized chat request
/// 
/// # Returns
/// * `Ok(JsonResponse<ChatResponse>)` - Response with memory information
/// * `Err((StatusCode, Json<ChatError>))` - Error response
pub async fn handle_memory_query(
    State(state): State<AppState>,
    Json(payload): Json<ChatRequest>,
) -> Result<JsonResponse<ChatResponse>, (StatusCode, Json<ChatError>)> {
    info!("Received memory query: {}", payload.message);

    // Check if this is a memory retrieval request
    let is_memory_query = payload.message.to_lowercase().contains("what was") ||
                         payload.message.to_lowercase().contains("remind me") ||
                         payload.message.to_lowercase().contains("do you remember") ||
                         payload.message.to_lowercase().contains("last time") ||
                         payload.message.to_lowercase().contains("what did we talk about");

    if !is_memory_query {
        // Redirect to regular chat handler
        return handle_chat(State(state), Json(payload)).await;
    }

    // Search for relevant memories and tasks based on the query
    let db = state.db.get().unwrap();
    let mut found_memories = Vec::new();
    let mut found_tasks = Vec::new();

    // Search memories
    if let Ok(memory_response) = crate::db::queries::get_enhanced_memories(&db, &crate::models::memory::MemoryQuery {
        key: None,
        category: None,
        priority: None,
        limit: Some(10),
        offset: Some(0),
        include_expired: Some(false),
    }) {
        for memory in memory_response.memories.iter() {
            if payload.message.to_lowercase().contains(&memory.key.to_lowercase()) ||
               memory.value.to_lowercase().contains(&payload.message.to_lowercase()) {
                found_memories.push(format!("{}: {}", memory.key, memory.value));
            }
        }
    }

    // Search tasks
    if let Ok(task_response) = crate::db::queries::get_tasks(&db, &crate::models::memory::TaskQuery {
        status: None,
        priority: None,
        limit: Some(10),
        offset: Some(0),
        include_completed: Some(false),
    }) {
        for task in task_response.tasks.iter() {
            if payload.message.to_lowercase().contains(&task.title.to_lowercase()) ||
               task.description.as_ref().map(|d| d.to_lowercase().contains(&payload.message.to_lowercase())).unwrap_or(false) {
                found_tasks.push(format!("{}: {}", task.title, task.description.as_deref().unwrap_or("No description")));
            }
        }
    }

    let mut response_text = String::new();
    if !found_memories.is_empty() {
        response_text.push_str("I found these memories:\n");
        for memory in found_memories.iter().take(3) {
            response_text.push_str(&format!("• {}\n", memory));
        }
    }
    if !found_tasks.is_empty() {
        response_text.push_str("I found these tasks:\n");
        for task in found_tasks.iter().take(3) {
            response_text.push_str(&format!("• {}\n", task));
        }
    }
    if response_text.is_empty() {
        response_text = "I searched through your memories and tasks but didn't find anything related to your query. You can create new memories or tasks if needed.".to_string();
    }

    let response = ChatResponse {
        message: response_text,
        conversation_id: uuid::Uuid::new_v4(),
        timestamp: chrono::Utc::now(),
        context: payload.context,
    };

    Ok(JsonResponse(response))
}

/// Get conversation summary with memory context (dynamic)
/// 
/// This endpoint provides a summary of recent conversations and stored memories
/// to help users understand what information has been stored.
/// 
/// # Arguments
/// * `payload` - Request with optional session ID
/// 
/// # Returns
/// * `Ok(JsonResponse<ChatResponse>)` - Summary response
/// * `Err((StatusCode, Json<ChatError>))` - Error response
pub async fn get_conversation_summary(
    State(state): State<AppState>,
    Json(payload): Json<ChatRequest>,
) -> Result<JsonResponse<ChatResponse>, (StatusCode, Json<ChatError>)> {
    info!("Requesting conversation summary for session: {:?}", payload.session_id);
    let db = state.db.get().unwrap();

    // Get recent tasks
    let mut summary_lines = Vec::new();
    if let Ok(task_response) = crate::db::queries::get_tasks(&db, &crate::models::memory::TaskQuery {
        status: None,
        priority: None,
        limit: Some(3),
        offset: Some(0),
        include_completed: Some(false),
    }) {
        for task in task_response.tasks.iter() {
            summary_lines.push(format!("• Task: {} (priority {})", task.title, task.priority));
        }
    }
    // Get recent memories
    if let Ok(memory_response) = crate::db::queries::get_enhanced_memories(&db, &crate::models::memory::MemoryQuery {
        key: None,
        category: None,
        priority: None,
        limit: Some(3),
        offset: Some(0),
        include_expired: Some(false),
    }) {
        for memory in memory_response.memories.iter() {
            summary_lines.push(format!("• Memory: {} (category {})", memory.key, memory.category));
        }
    }
    let summary_text = if summary_lines.is_empty() {
        "No recent tasks or memories found.".to_string()
    } else {
        format!("Here's a summary of your recent tasks and memories:\n\n{}", summary_lines.join("\n"))
    };

    let response = ChatResponse {
        message: summary_text,
        conversation_id: uuid::Uuid::new_v4(),
        timestamp: chrono::Utc::now(),
        context: Some("conversation_summary".to_string()),
    };

    Ok(JsonResponse(response))
}

/// Create router for chat-related endpoints
pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", post(handle_chat))
        .route("/memory", post(handle_memory_query))
        .route("/summary", post(get_conversation_summary))
} 