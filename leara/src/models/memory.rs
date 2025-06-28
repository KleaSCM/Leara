/*
 * Leara AI Assistant - Memory Models
 * 
 * This module defines data structures for memory management including
 * persistent storage of conversations, user preferences, and context.
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
 * File: src/models/memory.rs
 * Purpose: Memory-related data models and structures
 */

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Enhanced memory entry with better organization and categorization
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Memory {
    pub id: i64,
    pub key: String,
    pub value: String,
    pub category: String,
    pub priority: i32,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
}

/// Request structure for storing memory entries
#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryRequest {
    pub key: String,
    pub value: String,
    pub category: Option<String>,
    pub priority: Option<i32>,
    pub metadata: Option<serde_json::Value>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Query structure for retrieving memory entries
#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryQuery {
    pub key: Option<String>,
    pub category: Option<String>,
    pub priority: Option<i32>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub include_expired: Option<bool>,
}

/// Response structure for memory operations
#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryResponse {
    pub memories: Vec<Memory>,
    pub total: i64,
}

/// Task structure for tracking user tasks and reminders
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: i32,
    pub due_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub context: Option<String>,
    pub tags: Option<String>,
}

/// Request structure for creating or updating tasks
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskRequest {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<i32>,
    pub due_date: Option<DateTime<Utc>>,
    pub context: Option<String>,
    pub tags: Option<String>,
}

/// Query structure for retrieving tasks
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskQuery {
    pub status: Option<String>,
    pub priority: Option<i32>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub include_completed: Option<bool>,
}

/// Response structure for task operations
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResponse {
    pub tasks: Vec<Task>,
    pub total: i64,
}

/// Session context structure for maintaining conversation context
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionContext {
    pub id: i64,
    pub session_id: String,
    pub context_key: String,
    pub context_value: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request structure for storing session context
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionContextRequest {
    pub session_id: String,
    pub context_key: String,
    pub context_value: String,
}

/// Query structure for retrieving session context
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionContextQuery {
    pub session_id: Option<String>,
    pub context_key: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// Response structure for session context operations
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionContextResponse {
    pub contexts: Vec<SessionContext>,
    pub total: i64,
}

/// Memory entry for backward compatibility
#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub id: i64,
    pub key: String,
    pub value: String,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Memory categories for better organization
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum MemoryCategory {
    General,
    Conversation,
    Task,
    Reminder,
    Preference,
    Context,
    System,
    Project,
    Custom(String),
}

impl MemoryCategory {
    pub fn as_str(&self) -> &str {
        match self {
            MemoryCategory::General => "general",
            MemoryCategory::Conversation => "conversation",
            MemoryCategory::Task => "task",
            MemoryCategory::Reminder => "reminder",
            MemoryCategory::Preference => "preference",
            MemoryCategory::Context => "context",
            MemoryCategory::System => "system",
            MemoryCategory::Project => "project",
            MemoryCategory::Custom(s) => s,
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "general" => MemoryCategory::General,
            "conversation" => MemoryCategory::Conversation,
            "task" => MemoryCategory::Task,
            "reminder" => MemoryCategory::Reminder,
            "preference" => MemoryCategory::Preference,
            "context" => MemoryCategory::Context,
            "system" => MemoryCategory::System,
            "project" => MemoryCategory::Project,
            _ => MemoryCategory::Custom(s.to_string()),
        }
    }
}

/// Task status enumeration
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled,
    Custom(String),
}

impl TaskStatus {
    pub fn as_str(&self) -> &str {
        match self {
            TaskStatus::Pending => "pending",
            TaskStatus::InProgress => "in_progress",
            TaskStatus::Completed => "completed",
            TaskStatus::Cancelled => "cancelled",
            TaskStatus::Custom(s) => s,
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "pending" => TaskStatus::Pending,
            "in_progress" | "inprogress" => TaskStatus::InProgress,
            "completed" => TaskStatus::Completed,
            "cancelled" | "canceled" => TaskStatus::Cancelled,
            _ => TaskStatus::Custom(s.to_string()),
        }
    }
} 