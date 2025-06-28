/*
 * Leara AI Assistant - Database Migrations
 * 
 * This module handles database schema migrations and table creation.
 * Manages the evolution of the database structure over time.
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
 * File: src/db/migrations.rs
 * Purpose: Database schema migrations and table management
 */

use rusqlite::{Connection, Result};
use tracing::info;

/// Run all database migrations
pub fn run_migrations(conn: &Connection) -> Result<()> {
    info!("Running database migrations...");
    
    // Create conversations table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS conversations (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            created_at DATETIME NOT NULL,
            updated_at DATETIME NOT NULL
        )",
        [],
    )?;

    // Create messages table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS messages (
            id TEXT PRIMARY KEY,
            conversation_id TEXT NOT NULL,
            content TEXT NOT NULL,
            role TEXT NOT NULL,
            timestamp DATETIME NOT NULL,
            FOREIGN KEY (conversation_id) REFERENCES conversations (id)
        )",
        [],
    )?;

    // Create memory table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS memory (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            key TEXT NOT NULL UNIQUE,
            value TEXT NOT NULL,
            metadata TEXT,
            created_at DATETIME NOT NULL,
            updated_at DATETIME NOT NULL
        )",
        [],
    )?;

    // Create indexes
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_messages_conversation_id ON messages (conversation_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_memory_key ON memory (key)",
        [],
    )?;

    info!("Database migrations completed successfully");
    Ok(())
} 