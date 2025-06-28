/*
 * Leara AI Assistant - Models Module
 * 
 * This module contains all data models and structures used throughout
 * the application for type safety and data validation.
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
 * File: src/models/mod.rs
 * Purpose: Data models and structures
 */

pub mod chat;
pub mod memory;
pub mod system;

pub use chat::*;
pub use memory::*;
pub use system::*;

use std::sync::{Arc, Mutex};
use rusqlite::Connection;
use crate::system::MemoryService;
use r2d2::{Pool};
use r2d2_sqlite::SqliteConnectionManager;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<SqliteConnectionManager>,
    pub memory_service: Arc<MemoryService>,
} 