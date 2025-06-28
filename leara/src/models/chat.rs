use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: Uuid,
    pub content: String,
    pub sender: MessageSender,
    pub timestamp: DateTime<Utc>,
    pub conversation_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MessageSender {
    User,
    Assistant,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub conversation_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    pub message: String,
    pub conversation_id: Uuid,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Conversation {
    pub id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub message_count: i32,
} 