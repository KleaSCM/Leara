/*
 * Leara AI Assistant - Chat API Handler
 * 
 * This module handles chat-related API endpoints for the AI assistant.
 * Processes user messages and manages conversation state.
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
 * Purpose: Chat API endpoint handlers
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
/// // Client sends: {"message": "Hello", "context": "greeting"}
/// // Server responds with AI-generated response from memgpt model
/// ```
pub async fn handle_chat(
    Json(payload): Json<ChatRequest>,
) -> Result<JsonResponse<ChatResponse>, (StatusCode, Json<ChatError>)> {
    // Log the incoming message for debugging and monitoring
    info!("Received chat message: {}", payload.message);

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

    // Create system prompt for context-aware responses
    let system_prompt = match &payload.context {
        Some(context) => format!(
            "You are Leara, an AI assistant. Context: {}. \
            and systems admin.",
            context
        ),
        None => "You are Leara, an intelligent AI assistant.".to_string(),
    };

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
                    error: "nope.".to_string(),
                }),
            ));
        }
    };

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