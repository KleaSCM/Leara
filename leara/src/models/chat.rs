/*
 * Leara AI Assistant - Chat Models
 * 
 * This module defines data structures for chat functionality including
 * messages, conversations, and API request/response models.
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
 * File: src/models/chat.rs
 * Purpose: Chat-related data models and structures
 */

// Import Serde for JSON serialization/deserialization
use serde::{Deserialize, Serialize};
// Import chrono for timestamp handling
use chrono::{DateTime, Utc};
// Import UUID for generating unique identifiers
use uuid::Uuid;

/// Individual chat message within a conversation
/// Represents a single message exchange between user and assistant
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Unique identifier for the message (UUID v4)
    pub id: Uuid,
    /// The actual message content (text, markdown, or structured data)
    pub content: String,
    /// Who sent the message (user or assistant)
    pub sender: MessageSender,
    /// When the message was created (ISO 8601 timestamp)
    pub timestamp: DateTime<Utc>,
    /// Optional reference to the conversation this message belongs to
    /// None for standalone messages, Some(UUID) for conversation messages
    pub conversation_id: Option<Uuid>,
}

/// Enumeration of possible message senders
/// Distinguishes between user input and assistant responses
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MessageSender {
    /// Message sent by the user (human input)
    User,
    /// Message sent by the AI assistant (system response)
    Assistant,
}

/// Request structure for incoming chat messages
/// Contains the user's message and optional conversation context
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    /// The user's message content to be processed by the AI
    pub message: String,
    /// Optional conversation ID for continuing existing conversations
    /// If provided, the response will be part of the same conversation thread
    pub conversation_id: Option<Uuid>,
}

/// Response structure for chat API responses
/// Contains the AI assistant's reply and conversation metadata
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    /// The AI assistant's generated response message
    pub message: String,
    /// Unique identifier for the conversation this response belongs to
    /// New conversations get a new UUID, existing ones use the provided ID
    pub conversation_id: Uuid,
    /// When the response was generated (ISO 8601 timestamp)
    pub timestamp: DateTime<Utc>,
    /// Optional context information that influenced the response
    /// Can include conversation history, user preferences, or system context
    pub context: Option<String>,
}

/// Complete conversation thread containing multiple messages
/// Represents a full conversation session between user and assistant
#[derive(Debug, Serialize, Deserialize)]
pub struct Conversation {
    /// Unique identifier for the conversation (UUID v4)
    pub id: Uuid,
    /// Human-readable title for the conversation (auto-generated or user-defined)
    /// Examples: "Rust async/await help", "System troubleshooting", "Code review"
    pub title: String,
    /// When the conversation was first created (ISO 8601 timestamp)
    pub created_at: DateTime<Utc>,
    /// When the conversation was last updated (ISO 8601 timestamp)
    /// Updated whenever a new message is added to the conversation
    pub updated_at: DateTime<Utc>,
    /// Total number of messages in this conversation
    /// Includes both user and assistant messages
    pub message_count: i32,
} 