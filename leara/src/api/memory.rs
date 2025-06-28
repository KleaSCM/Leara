use axum::{Json, http::StatusCode, extract::Query};
use crate::models::{MemoryRequest, MemoryQuery, MemoryResponse, Memory};
use chrono::Utc;
use uuid::Uuid;
use tracing::info;

pub async fn store_memory(
    Json(payload): Json<MemoryRequest>,
) -> Result<(StatusCode, Json<Memory>), (StatusCode, Json<serde_json::Value>)> {
    info!("Storing memory with key: {}", payload.key);

    let memory = Memory {
        id: Uuid::new_v4(),
        key: payload.key,
        value: payload.value,
        category: payload.category.unwrap_or_else(|| "general".to_string()),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        expires_at: payload.expires_at,
    };

    // TODO: Actually store in database
    info!("Memory stored successfully");
    Ok((StatusCode::CREATED, Json(memory)))
}

pub async fn get_memory(
    Query(_query): Query<MemoryQuery>,
) -> Result<(StatusCode, Json<MemoryResponse>), (StatusCode, Json<serde_json::Value>)> {
    info!("Retrieving memory");

    // TODO: Actually query database
    let memories = vec![
        Memory {
            id: Uuid::new_v4(),
            key: "example_key".to_string(),
            value: "example_value".to_string(),
            category: "general".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            expires_at: None,
        }
    ];

    let response = MemoryResponse {
        memories,
        total: 1,
    };

    Ok((StatusCode::OK, Json(response)))
} 