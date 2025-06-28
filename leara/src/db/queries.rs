/*
 * Leara AI Assistant - Database Queries
 * 
 * This module contains database query functions for common operations
 * including CRUD operations for conversations, messages, and memory.
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
 * File: src/db/queries.rs
 * Purpose: Database query functions and operations
 */

// Import rusqlite for SQLite database operations
use rusqlite::{Connection, Result, params};
// Import our local models for type safety
use crate::models::{chat::*, memory::*};
// Import chrono for timestamp handling
use chrono::Utc;
// Import uuid for unique identifier handling
use uuid;

/// Insert a new conversation into the database
/// 
/// This function creates a new conversation record in the conversations table.
/// The conversation ID is converted to a string for SQLite storage, and timestamps
/// are formatted as RFC3339 strings for consistent datetime handling.
/// 
/// # Arguments
/// * `conn` - Active database connection
/// * `conversation` - Conversation struct containing the data to insert
/// 
/// # Returns
/// * `Ok(())` - Successfully inserted conversation
/// * `Err(rusqlite::Error)` - Database error (connection, constraint violation, etc.)
/// 
/// # Example
/// ```rust
/// let conversation = Conversation {
///     id: Uuid::new_v4(),
///     title: "Rust async/await help".to_string(),
///     created_at: Utc::now(),
///     updated_at: Utc::now(),
///     message_count: 0,
/// };
/// insert_conversation(&conn, &conversation)?;
/// ```
/// 
/// # Database Schema
/// ```sql
/// INSERT INTO conversations (id, title, created_at, updated_at) 
/// VALUES (?, ?, ?, ?)
/// ```
pub fn insert_conversation(conn: &Connection, conversation: &Conversation) -> Result<()> {
    // Execute the INSERT statement with parameterized values
    // Using parameterized queries prevents SQL injection and improves performance
    conn.execute(
        "INSERT INTO conversations (id, title, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
        params![
            conversation.id.to_string(),  // Convert UUID to string for SQLite storage
            conversation.title,           // Store conversation title
            conversation.created_at.to_rfc3339(),  // Format timestamp as RFC3339 string
            conversation.updated_at.to_rfc3339()   // Format timestamp as RFC3339 string
        ],
    )?;
    Ok(())
}

/// Retrieve all conversations from the database
/// 
/// This function fetches all conversation records, ordered by most recently updated.
/// Each conversation is reconstructed from the database with proper type conversions.
/// 
/// # Arguments
/// * `conn` - Active database connection
/// 
/// # Returns
/// * `Ok(Vec<Conversation>)` - List of all conversations, ordered by updated_at DESC
/// * `Err(rusqlite::Error)` - Database error (connection, query, parsing, etc.)
/// 
/// # Example Response
/// ```rust
/// let conversations = get_conversations(&conn)?;
/// for conv in conversations {
///     println!("Conversation: {} ({} messages)", conv.title, conv.message_count);
/// }
/// ```
/// 
/// # Database Schema
/// ```sql
/// SELECT id, title, created_at, updated_at 
/// FROM conversations 
/// ORDER BY updated_at DESC
/// ```
/// 
/// # Performance Considerations
/// - For large datasets, consider implementing pagination
/// - The message_count is currently hardcoded to 0 (TODO: implement actual count)
/// - Consider adding indexes on updated_at for better performance
pub fn get_conversations(conn: &Connection) -> Result<Vec<Conversation>> {
    // Prepare the SELECT statement for better performance
    // This allows the database to optimize the query execution plan
    let mut stmt = conn.prepare(
        "SELECT id, title, created_at, updated_at FROM conversations ORDER BY updated_at DESC"
    )?;
    
    // Execute the query and map results to Conversation structs
    // Each row is processed individually to handle type conversions safely
    let conversations = stmt.query_map([], |row| {
        // Extract string values from the database row
        let created_at_str: String = row.get(2)?;  // created_at column
        let updated_at_str: String = row.get(3)?;  // updated_at column
        let id_str: String = row.get(0)?;          // id column
        
        // Construct the Conversation struct with proper type conversions
        Ok(Conversation {
            // Parse UUID from string, fallback to new UUID if parsing fails
            id: uuid::Uuid::parse_str(&id_str).unwrap_or_else(|_| uuid::Uuid::new_v4()),
            title: row.get(1)?,  // title column
            // Parse RFC3339 timestamp, fallback to current time if parsing fails
            created_at: chrono::DateTime::parse_from_rfc3339(&created_at_str)
                .unwrap_or_else(|_| Utc::now().into())
                .with_timezone(&Utc),
            // Parse RFC3339 timestamp, fallback to current time if parsing fails
            updated_at: chrono::DateTime::parse_from_rfc3339(&updated_at_str)
                .unwrap_or_else(|_| Utc::now().into())
                .with_timezone(&Utc),
            message_count: 0, // TODO: Calculate actual message count
            // Future implementation should:
            // SELECT COUNT(*) FROM messages WHERE conversation_id = ?
        })
    })?
    .collect::<Result<Vec<_>>>()?;  // Collect all results into a Vec
    
    Ok(conversations)
}

/// Insert or update a memory entry in the database
/// 
/// This function uses INSERT OR REPLACE to handle both new entries and updates.
/// If a memory entry with the same key already exists, it will be updated.
/// Metadata is serialized to JSON string for storage.
/// 
/// # Arguments
/// * `conn` - Active database connection
/// * `entry` - MemoryEntry struct containing the data to store
/// 
/// # Returns
/// * `Ok(())` - Successfully stored memory entry
/// * `Err(rusqlite::Error)` - Database error (connection, constraint violation, etc.)
/// 
/// # Example
/// ```rust
/// let entry = MemoryEntry {
///     id: 0,  // Will be auto-generated
///     key: "user_preferences".to_string(),
///     value: r#"{"theme": "dark", "language": "en"}"#.to_string(),
///     metadata: Some(json!({"category": "settings"})),
///     created_at: Utc::now(),
///     updated_at: Utc::now(),
/// };
/// insert_memory(&conn, &entry)?;
/// ```
/// 
/// # Database Schema
/// ```sql
/// INSERT OR REPLACE INTO memory (key, value, metadata, created_at, updated_at) 
/// VALUES (?, ?, ?, ?, ?)
/// ```
pub fn insert_memory(conn: &Connection, entry: &MemoryEntry) -> Result<()> {
    // Execute the INSERT OR REPLACE statement
    // This handles both new entries and updates to existing entries
    conn.execute(
        "INSERT OR REPLACE INTO memory (key, value, metadata, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            entry.key,  // Memory key (unique identifier)
            entry.value,  // Memory value (actual data)
            // Serialize metadata to JSON string, or empty string if None
            entry.metadata.as_ref().map(|m| serde_json::to_string(m).unwrap_or_default()),
            entry.created_at.to_rfc3339(),  // Format timestamp as RFC3339 string
            entry.updated_at.to_rfc3339()   // Format timestamp as RFC3339 string
        ],
    )?;
    Ok(())
}

/// Retrieve a memory entry by its key
/// 
/// This function looks up a specific memory entry using its unique key.
/// Returns None if no entry with the given key exists.
/// 
/// # Arguments
/// * `conn` - Active database connection
/// * `key` - Unique key identifying the memory entry
/// 
/// # Returns
/// * `Ok(Some(MemoryEntry))` - Found memory entry
/// * `Ok(None)` - No entry found with the given key
/// * `Err(rusqlite::Error)` - Database error (connection, query, parsing, etc.)
/// 
/// # Example
/// ```rust
/// match get_memory_by_key(&conn, "user_preferences")? {
///     Some(entry) => println!("Found: {}", entry.value),
///     None => println!("No preferences found"),
/// }
/// ```
/// 
/// # Database Schema
/// ```sql
/// SELECT id, key, value, metadata, created_at, updated_at 
/// FROM memory 
/// WHERE key = ?
/// ```
/// 
/// # Error Handling
/// - Gracefully handles missing entries by returning None
/// - Robust timestamp parsing with fallback to current time
/// - JSON metadata parsing with error recovery
pub fn get_memory_by_key(conn: &Connection, key: &str) -> Result<Option<MemoryEntry>> {
    // Prepare the SELECT statement for better performance
    let mut stmt = conn.prepare(
        "SELECT id, key, value, metadata, created_at, updated_at FROM memory WHERE key = ?"
    )?;
    
    // Execute the query with the provided key
    let mut rows = stmt.query(params![key])?;
    
    // Check if a row was found
    if let Some(row) = rows.next()? {
        // Extract timestamp strings from the database row
        let created_at_str: String = row.get(4)?;  // created_at column
        let updated_at_str: String = row.get(5)?;  // updated_at column
        
        // Construct the MemoryEntry struct with proper type conversions
        let entry = MemoryEntry {
            id: row.get(0)?,  // Auto-generated ID
            key: row.get(1)?,  // Memory key
            value: row.get(2)?,  // Memory value
            // Parse JSON metadata, or None if parsing fails
            metadata: row.get::<_, Option<String>>(3)?
                .and_then(|s| serde_json::from_str(&s).ok()),
            // Parse RFC3339 timestamp, fallback to current time if parsing fails
            created_at: chrono::DateTime::parse_from_rfc3339(&created_at_str)
                .unwrap_or_else(|_| Utc::now().into())
                .with_timezone(&Utc),
            // Parse RFC3339 timestamp, fallback to current time if parsing fails
            updated_at: chrono::DateTime::parse_from_rfc3339(&updated_at_str)
                .unwrap_or_else(|_| Utc::now().into())
                .with_timezone(&Utc),
        };
        Ok(Some(entry))
    } else {
        // No entry found with the given key
        Ok(None)
    }
} 