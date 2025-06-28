        // openhermes:latest                          
        // hexbenjamin/memgpt-dpo-uncensored:f16      
        // wizard-vicuna-uncensored:13b               
        // qwen2-math:7b                             
        // HammerAI/mythomax-l2:latest                
        // bakllava:latest                            
        // qwen2.5-coder:7b                           
        // qwen2.5-coder:14b                          
        // HammerAI/llama-3-lexi-uncensored:latest

use axum::{Json, http::StatusCode};
use crate::models::{ChatRequest, ChatResponse};
use chrono::Utc;
use uuid::Uuid;
use tracing::info;

pub async fn handle_chat(
    Json(payload): Json<ChatRequest>,
) -> Result<(StatusCode, Json<ChatResponse>), (StatusCode, Json<serde_json::Value>)> {
    info!("Received chat request: {}", payload.message);

    // For now, return a simple echo response
    // TODO: Integrate with actual AI model
    let response = ChatResponse {

        message: format!("I received your message: '{}'. This is a placeholder response from the Rust backend.", payload.message),
        conversation_id: payload.conversation_id.unwrap_or_else(Uuid::new_v4),
        timestamp: Utc::now(),
    };

    info!("Sending chat response");
    Ok((StatusCode::OK, Json(response)))
} 