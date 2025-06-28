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
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Memory {
    pub id: Uuid,
    pub key: String,
    pub value: String,
    pub category: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryRequest {
    pub key: String,
    pub value: String,
    pub category: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryQuery {
    pub key: Option<String>,
    pub category: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryResponse {
    pub memories: Vec<Memory>,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub id: i64,
    pub key: String,
    pub value: String,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
} 