/*
 * Leara AI Assistant - Database Module
 * 
 * This module handles database initialization, connection management,
 * and provides database utilities for the application.
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
 * File: src/db/mod.rs
 * Purpose: Database module organization and connection management
 */

pub mod migrations;
pub mod queries;

use rusqlite::{Connection, Result};
use std::path::Path;
use tracing::info;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Database { conn })
    }

    pub fn get_connection(&self) -> &Connection {
        &self.conn
    }

    pub fn get_connection_mut(&mut self) -> &mut Connection {
        &mut self.conn
    }
}

/// Initialize the database with required tables
pub async fn init_database(db_path: &str) -> anyhow::Result<()> {
    // Ensure data directory exists
    if let Some(parent) = Path::new(db_path).parent() {
        std::fs::create_dir_all(parent).map_err(|e| anyhow::anyhow!("Failed to create directory: {}", e))?;
    }

    let conn = Connection::open(db_path)?;
    
    // Run migrations
    migrations::run_migrations(&conn)?;
    
    info!("Database init successfully");
    Ok(())
}

/// Get a database connection
pub fn get_connection(db_path: &str) -> Result<Connection> {
    Connection::open(db_path)
} 