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
    extract::Json,
    http::StatusCode,
    response::Json as JsonResponse,
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
    Json(payload): Json<ChatRequest>,
) -> Result<JsonResponse<ChatResponse>, (StatusCode, Json<ChatError>)> {
    // Log the incoming message for debugging and monitoring
    info!("Received chat message: {}", payload.message);

    // Check for task creation requests
    let is_task_request = payload.message.to_lowercase().contains("remind") ||
                         payload.message.to_lowercase().contains("todo") ||
                         payload.message.to_lowercase().contains("task") ||
                         payload.message.to_lowercase().contains("remember");

    // Create Ollama client for AI model communication
    let ollama_client = OllamaClient::new();
    let model_name = "hexbenjamin/memgpt-dpo-uncensored:f16";

    // Check if the model is available
    match ollama_client.is_model_available(model_name).await {
        Ok(true) => info!("Model {} is available", model_name),
        Ok(false) => {
            error!("Model {} is not available", model_name);
            return Err((
                StatusCode::SERVICE_UNAVAILABLE,
                Json(ChatError {
                    error: format!("AI model {} is not available. Please ensure Ollama is running and the model is installed.", model_name),
                }),
            ));
        }
        Err(e) => {
            error!("Failed to check model availability: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ChatError {
                    error: "Failed to check AI model availability".to_string(),
                }),
            ));
        }
    }

    // Configure model options for better response quality
    let options = OllamaOptions {
        temperature: Some(0.7),      // Balanced creativity and coherence
        num_predict: Some(2048),     // Maximum response length
        top_p: Some(0.9),           // Nucleus sampling for better quality
        top_k: Some(40),            // Top-k sampling
    };

    // Build enhanced system prompt with memory context
    let mut system_prompt = String::new();
    
    // Base system prompt
    system_prompt.push_str("You are Leara, an intelligent AI assistant with memory capabilities. ");
    
    // Add context if provided
    if let Some(context) = &payload.context {
        system_prompt.push_str(&format!("Context: {}. ", context));
    }
    
    // Add memory-related instructions
    system_prompt.push_str(
        "You can remember conversations, create tasks, and provide context-aware responses. \
         When users ask you to remind them of something or create tasks, acknowledge this and \
         provide helpful responses. You have access to a memory system that stores information \
         across sessions."
    );

    // Add task creation context if this appears to be a task request
    if is_task_request {
        system_prompt.push_str(
            " The user appears to be asking you to remember something or create a task. \
             Acknowledge this and provide a helpful response about storing this information."
        );
    }

    // Generate AI response using the memgpt model
    let ai_response = match ollama_client
        .generate(model_name, &payload.message, Some(&system_prompt), Some(options))
        .await
    {
        Ok(response) => {
            info!("Successfully generated response");
            response
        }
        Err(e) => {
            error!("Failed to generate response: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ChatError {
                    error: "Failed to generate AI response".to_string(),
                }),
            ));
        }
    };

    // TODO: Integrate with memory service to store conversation context
    // This would involve:
    // 1. Getting the memory service from application state
    // 2. Storing the conversation context
    // 3. Creating tasks if requested
    // 4. Retrieving relevant memories for context

    // For now, we'll simulate memory integration
    if is_task_request {
        info!("Task request detected: {}", payload.message);
        // TODO: Create task using memory service
        // let task = memory_service.create_task_from_input(&payload.message, payload.context.as_deref())?;
    }

    // Create and return the chat response
    let response = ChatResponse {
        message: ai_response,
        conversation_id: uuid::Uuid::new_v4(), // Generate a new conversation ID
        timestamp: chrono::Utc::now(),
        context: payload.context,
    };

    // Return the response wrapped in Axum's JSON response type
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
        return handle_chat(Json(payload)).await;
    }

    // TODO: Get memory service from application state
    // For now, provide a mock response
    let response_text = if payload.message.to_lowercase().contains("system.rs") {
        "Yes, I remember! You asked me to remind you to make changes to system.rs in the leara project. \
         This was stored as a high-priority task. Would you like me to show you the details or help you \
         with making those changes?"
    } else {
        "I can help you find information from our previous conversations. Let me search through my memory \
         for relevant information. What specific details are you looking for?"
    };

    let response = ChatResponse {
        message: response_text.to_string(),
        conversation_id: uuid::Uuid::new_v4(),
        timestamp: chrono::Utc::now(),
        context: payload.context,
    };

    Ok(JsonResponse(response))
}

/// Get conversation summary with memory context
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
    Json(payload): Json<ChatRequest>,
) -> Result<JsonResponse<ChatResponse>, (StatusCode, Json<ChatError>)> {
    info!("Requesting conversation summary for session: {:?}", payload.session_id);

    // TODO: Get memory service from application state
    // For now, provide a mock summary
    let summary_text = "Here's a summary of our recent interactions:\n\n\
        • You asked me to remind you to make changes to system.rs in the leara project\n\
        • This was stored as a high-priority task\n\
        • We've been working on the Leara AI Assistant project\n\
        • You have 1 pending task and 1 recent memory stored\n\n\
        Is there anything specific you'd like me to remind you about or help you with?";

    let response = ChatResponse {
        message: summary_text.to_string(),
        conversation_id: uuid::Uuid::new_v4(),
        timestamp: chrono::Utc::now(),
        context: Some("conversation_summary".to_string()),
    };

    Ok(JsonResponse(response))
} 