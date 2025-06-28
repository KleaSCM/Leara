/*
 * Leara AI Assistant - Memory Service
 * 
 * This module provides intelligent memory management for the AI assistant,
 * including task tracking, context awareness, and natural language processing
 * for memory queries and storage.
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
 * File: src/system/memory_service.rs
 * Purpose: Intelligent memory management and task tracking
 */

use rusqlite::Connection;
use chrono::{Utc, Duration, DateTime};
use serde_json::json;
use tracing::{info, warn, error};
use crate::models::memory::*;
use crate::db::queries::*;
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;

/// Memory service for intelligent storage and retrieval of information
/// 
/// This service provides high-level memory management capabilities including:
/// - Intelligent task creation and tracking
/// - Context-aware memory storage
/// - Natural language processing for memory queries
/// - Automatic categorization and prioritization
/// - Session context management
pub struct MemoryService {
    pool: Pool<SqliteConnectionManager>,
}

impl MemoryService {
    /// Create a new memory service instance
    /// 
    /// # Arguments
    /// * `pool` - Connection pool for database operations
    /// 
    /// # Returns
    /// * `Self` - New memory service instance
    pub fn new(pool: Pool<SqliteConnectionManager>) -> Self {
        Self { pool }
    }

    /// Get a connection from the pool
    /// 
    /// # Returns
    /// * `Result<PooledConnection<SqliteConnectionManager>>` - Database connection
    fn get_conn(&self) -> Result<PooledConnection<SqliteConnectionManager>, rusqlite::Error> {
        self.pool.get().map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))
    }

    /// Store a memory entry with intelligent categorization
    /// 
    /// This function automatically categorizes and prioritizes memory entries
    /// based on their content and context.
    /// 
    /// # Arguments
    /// * `key` - Unique identifier for the memory entry
    /// * `value` - The actual data to store
    /// * `context` - Optional context information
    /// * `priority` - Optional priority level (1-5, default 3)
    /// 
    /// # Returns
    /// * `Result<(), rusqlite::Error>` - Success or error
    pub fn store_memory(&self, key: &str, value: &str, context: Option<&str>, priority: Option<i32>) -> Result<(), rusqlite::Error> {
        let conn = self.get_conn()?;
        let category = self.categorize_content(value, context);
        let priority = priority.unwrap_or_else(|| self.determine_priority(value, context));
        
        let memory = Memory {
            id: 0, // Will be auto-generated
            key: key.to_string(),
            value: value.to_string(),
            category: category.as_str().to_string(),
            priority,
            metadata: Some(json!({
                "context": context,
                "auto_categorized": true,
                "stored_at": Utc::now().to_rfc3339(),
            })),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            expires_at: None,
            is_active: true,
        };

        insert_enhanced_memory(&conn, &memory)?;
        info!("Stored memory: {} (category: {}, priority: {})", key, category.as_str(), priority);
        Ok(())
    }

    /// Create a task from natural language input
    /// 
    /// This function parses natural language input to create structured tasks
    /// with appropriate priorities and due dates.
    /// 
    /// # Arguments
    /// * `input` - Natural language task description
    /// * `context` - Optional context information
    /// 
    /// # Returns
    /// * `Result<Task, rusqlite::Error>` - Created task or error
    pub fn create_task_from_input(&self, input: &str, context: Option<&str>) -> Result<Task, rusqlite::Error> {
        let conn = self.get_conn()?;
        let (title, description, priority, due_date) = self.parse_task_input(input);
        
        let task = Task {
            id: 0, // Will be auto-generated
            title,
            description,
            status: "pending".to_string(),
            priority,
            due_date,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            completed_at: None,
            context: context.map(|s| s.to_string()),
            tags: self.extract_tags(input),
        };

        insert_task(&conn, &task)?;
        info!("Created task: {} (priority: {}, due: {:?})", task.title, priority, due_date);
        Ok(task)
    }

    /// Retrieve relevant memories based on natural language query
    /// 
    /// This function uses semantic matching to find relevant memories
    /// based on the user's query.
    /// 
    /// # Arguments
    /// * `query` - Natural language query
    /// * `limit` - Maximum number of results to return
    /// 
    /// # Returns
    /// * `Result<Vec<Memory>, rusqlite::Error>` - Relevant memories or error
    pub fn find_relevant_memories(&self, query: &str, limit: Option<i32>) -> Result<Vec<Memory>, rusqlite::Error> {
        let conn = self.get_conn()?;
        let keywords = self.extract_keywords(query);
        let mut relevant_memories = Vec::new();

        // Search by category first
        for keyword in &keywords {
            let category = self.categorize_content(keyword, None);
            let memory_query = MemoryQuery {
                key: None,
                category: Some(category.as_str().to_string()),
                priority: None,
                limit: Some(10),
                offset: Some(0),
                include_expired: Some(false),
            };

            if let Ok(response) = get_enhanced_memories(&conn, &memory_query) {
                relevant_memories.extend(response.memories);
            }
        }

        // Search by key similarity
        for keyword in &keywords {
            let memory_query = MemoryQuery {
                key: Some(keyword.clone()),
                category: None,
                priority: None,
                limit: Some(5),
                offset: Some(0),
                include_expired: Some(false),
            };

            if let Ok(response) = get_enhanced_memories(&conn, &memory_query) {
                relevant_memories.extend(response.memories);
            }
        }

        // Remove duplicates and sort by relevance
        relevant_memories.sort_by(|a, b| {
            let a_score = self.calculate_relevance_score(a, query);
            let b_score = self.calculate_relevance_score(b, query);
            b_score.partial_cmp(&a_score).unwrap_or(std::cmp::Ordering::Equal)
        });

        // Apply limit
        let limit = limit.unwrap_or(10);
        relevant_memories.truncate(limit as usize);

        Ok(relevant_memories)
    }

    /// Get pending tasks for the user
    /// 
    /// # Arguments
    /// * `include_overdue` - Whether to include overdue tasks
    /// 
    /// # Returns
    /// * `Result<Vec<Task>, rusqlite::Error>` - Pending tasks or error
    pub fn get_pending_tasks(&self, include_overdue: bool) -> Result<Vec<Task>, rusqlite::Error> {
        let conn = self.get_conn()?;
        let query = TaskQuery {
            status: Some("pending".to_string()),
            priority: None,
            limit: Some(50),
            offset: Some(0),
            include_completed: Some(false),
        };

        let response = get_tasks(&conn, &query)?;
        Ok(response.tasks)
    }

    /// Store session context for conversation continuity
    /// 
    /// # Arguments
    /// * `session_id` - Unique session identifier
    /// * `context_key` - Context key
    /// * `context_value` - Context value
    /// 
    /// # Returns
    /// * `Result<(), rusqlite::Error>` - Success or error
    pub fn store_session_context(&self, session_id: &str, context_key: &str, context_value: &str) -> Result<(), rusqlite::Error> {
        let conn = self.get_conn()?;
        let context = SessionContext {
            id: 0, // Will be auto-generated
            session_id: session_id.to_string(),
            context_key: context_key.to_string(),
            context_value: context_value.to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        insert_session_context(&conn, &context)?;
        info!("Stored session context: {} -> {}", context_key, context_value);
        Ok(())
    }

    /// Retrieve session context for conversation continuity
    /// 
    /// # Arguments
    /// * `session_id` - Unique session identifier
    /// 
    /// # Returns
    /// * `Result<Vec<SessionContext>, rusqlite::Error>` - Session contexts or error
    pub fn get_session_context(&self, session_id: &str) -> Result<Vec<SessionContext>, rusqlite::Error> {
        let conn = self.get_conn()?;
        let query = SessionContextQuery {
            session_id: Some(session_id.to_string()),
            context_key: None,
            limit: Some(50),
            offset: Some(0),
        };

        let response = get_session_contexts(&conn, &query)?;
        Ok(response.contexts)
    }

    /// Categorize content based on keywords and context
    /// 
    /// # Arguments
    /// * `content` - Content to categorize
    /// * `context` - Optional context information
    /// 
    /// # Returns
    /// * `MemoryCategory` - Determined category
    fn categorize_content(&self, content: &str, context: Option<&str>) -> MemoryCategory {
        let content_lower = content.to_lowercase();
        let _context_lower = context.map(|s| s.to_lowercase()).unwrap_or_default();

        // Check for task-related keywords
        if content_lower.contains("remind") || content_lower.contains("todo") || 
           content_lower.contains("task") || content_lower.contains("due") ||
           content_lower.contains("deadline") || content_lower.contains("schedule") {
            return MemoryCategory::Task;
        }

        // Check for reminder keywords
        if content_lower.contains("reminder") || content_lower.contains("remember") ||
           content_lower.contains("don't forget") || content_lower.contains("make sure") {
            return MemoryCategory::Reminder;
        }

        // Check for project-related keywords
        if content_lower.contains("project") || content_lower.contains("repository") ||
           content_lower.contains("code") || content_lower.contains("development") ||
           content_lower.contains("file") || content_lower.contains("system.rs") {
            return MemoryCategory::Project;
        }

        // Check for conversation context
        if content_lower.contains("conversation") || content_lower.contains("chat") ||
           content_lower.contains("discussion") || content_lower.contains("talk") {
            return MemoryCategory::Conversation;
        }

        // Check for system-related keywords
        if content_lower.contains("system") || content_lower.contains("computer") ||
           content_lower.contains("terminal") || content_lower.contains("command") {
            return MemoryCategory::System;
        }

        // Check for preference-related keywords
        if content_lower.contains("preference") || content_lower.contains("setting") ||
           content_lower.contains("config") || content_lower.contains("option") {
            return MemoryCategory::Preference;
        }

        // Default to general if no specific category is detected
        MemoryCategory::General
    }

    /// Determine priority based on content and context
    /// 
    /// # Arguments
    /// * `content` - Content to analyze
    /// * `context` - Optional context information
    /// 
    /// # Returns
    /// * `i32` - Priority level (1-5)
    fn determine_priority(&self, content: &str, context: Option<&str>) -> i32 {
        let content_lower = content.to_lowercase();
        let _context_lower = context.map(|s| s.to_lowercase()).unwrap_or_default();

        // High priority indicators
        if content_lower.contains("urgent") || content_lower.contains("important") ||
           content_lower.contains("critical") || content_lower.contains("asap") ||
           content_lower.contains("emergency") {
            return 5;
        }

        // Medium-high priority indicators
        if content_lower.contains("soon") || content_lower.contains("today") ||
           content_lower.contains("this week") || content_lower.contains("deadline") {
            return 4;
        }

        // Medium priority indicators
        if content_lower.contains("later") || content_lower.contains("next week") ||
           content_lower.contains("when you can") {
            return 3;
        }

        // Low priority indicators
        if content_lower.contains("sometime") || content_lower.contains("eventually") ||
           content_lower.contains("no rush") {
            return 2;
        }

        // Default priority
        3
    }

    /// Parse natural language task input
    /// 
    /// # Arguments
    /// * `input` - Natural language task description
    /// 
    /// # Returns
    /// * `(String, Option<String>, i32, Option<DateTime<Utc>>)` - Parsed task components
    fn parse_task_input(&self, input: &str) -> (String, Option<String>, i32, Option<DateTime<Utc>>) {
        let input_lower = input.to_lowercase();
        let mut title = input.to_string();
        let mut description = None;
        let mut priority = 3;
        let mut due_date = None;

        // Extract priority
        if input_lower.contains("urgent") || input_lower.contains("critical") {
            priority = 5;
        } else if input_lower.contains("important") || input_lower.contains("high priority") {
            priority = 4;
        } else if input_lower.contains("low priority") {
            priority = 2;
        }

        // Extract due date
        if input_lower.contains("today") {
            due_date = Some(Utc::now().date_naive().and_hms_opt(18, 0, 0).unwrap().and_utc());
        } else if input_lower.contains("tomorrow") {
            due_date = Some((Utc::now() + Duration::days(1)).date_naive().and_hms_opt(18, 0, 0).unwrap().and_utc());
        } else if input_lower.contains("this week") {
            due_date = Some(Utc::now() + Duration::days(7));
        } else if input_lower.contains("next week") {
            due_date = Some(Utc::now() + Duration::days(14));
        }

        // Clean up title
        title = title.replace("urgent", "").replace("important", "").replace("critical", "");
        title = title.replace("today", "").replace("tomorrow", "").replace("this week", "").replace("next week", "");
        title = title.trim().to_string();

        (title, description, priority, due_date)
    }

    /// Extract keywords from text
    /// 
    /// # Arguments
    /// * `text` - Text to extract keywords from
    /// 
    /// # Returns
    /// * `Vec<String>` - Extracted keywords
    fn extract_keywords(&self, text: &str) -> Vec<String> {
        let stop_words = vec![
            "the", "a", "an", "and", "or", "but", "in", "on", "at", "to", "for", "of", "with", "by",
            "is", "are", "was", "were", "be", "been", "being", "have", "has", "had", "do", "does", "did",
            "will", "would", "could", "should", "may", "might", "can", "this", "that", "these", "those",
            "i", "you", "he", "she", "it", "we", "they", "me", "him", "her", "us", "them",
            "my", "your", "his", "her", "its", "our", "their", "mine", "yours", "hers", "ours", "theirs",
        ];

        text.to_lowercase()
            .split_whitespace()
            .filter(|word| !stop_words.contains(word) && word.len() > 2)
            .map(|word| word.to_string())
            .collect()
    }

    /// Calculate relevance score for memory matching
    /// 
    /// # Arguments
    /// * `memory` - Memory entry to score
    /// * `query` - Query to match against
    /// 
    /// # Returns
    /// * `f64` - Relevance score (higher is more relevant)
    fn calculate_relevance_score(&self, memory: &Memory, query: &str) -> f64 {
        let query_keywords = self.extract_keywords(query);
        let memory_keywords = self.extract_keywords(&memory.value);
        let key_keywords = self.extract_keywords(&memory.key);

        let mut score = 0.0;

        // Exact matches get high scores
        for qk in &query_keywords {
            if memory_keywords.contains(qk) {
                score += 10.0;
            }
            if key_keywords.contains(qk) {
                score += 15.0; // Key matches are more important
            }
        }

        // Category relevance
        let category = MemoryCategory::from_str(&memory.category);
        let query_category = self.categorize_content(query, None);
        if category == query_category {
            score += 5.0;
        }

        // Priority bonus
        score += memory.priority as f64;

        // Recency bonus (newer memories get slight preference)
        let age = Utc::now() - memory.updated_at;
        let days_old = age.num_days() as f64;
        score += (30.0 - days_old).max(0.0) * 0.1;

        score
    }

    /// Extract tags from text
    /// 
    /// # Arguments
    /// * `text` - Text to extract tags from
    /// 
    /// # Returns
    /// * `Option<String>` - Comma-separated tags
    fn extract_tags(&self, text: &str) -> Option<String> {
        let text_lower = text.to_lowercase();
        let mut tags = Vec::new();

        // Extract project-related tags
        if text_lower.contains("rust") || text_lower.contains("cargo") {
            tags.push("rust");
        }
        if text_lower.contains("system.rs") || text_lower.contains("system") {
            tags.push("system");
        }
        if text_lower.contains("leara") {
            tags.push("leara");
        }
        if text_lower.contains("project") {
            tags.push("project");
        }

        if tags.is_empty() {
            None
        } else {
            Some(tags.join(","))
        }
    }

    /// Get a summary of recent memories and tasks
    /// 
    /// # Returns
    /// * `Result<String, rusqlite::Error>` - Summary text or error
    pub fn get_memory_summary(&self) -> Result<String, rusqlite::Error> {
        let conn = self.get_conn()?;
        let mut summary = String::new();

        // Get recent high-priority memories
        let memory_query = MemoryQuery {
            key: None,
            category: None,
            priority: Some(4),
            limit: Some(5),
            offset: Some(0),
            include_expired: Some(false),
        };

        if let Ok(response) = get_enhanced_memories(&conn, &memory_query) {
            if !response.memories.is_empty() {
                summary.push_str("Recent important memories:\n");
                for memory in &response.memories {
                    summary.push_str(&format!("- {}: {}\n", memory.key, memory.value));
                }
                summary.push('\n');
            }
        }

        // Get pending tasks
        if let Ok(tasks) = self.get_pending_tasks(true) {
            if !tasks.is_empty() {
                summary.push_str("Pending tasks:\n");
                for task in &tasks {
                    let due_info = task.due_date
                        .map(|d| format!(" (due: {})", d.format("%Y-%m-%d")))
                        .unwrap_or_default();
                    summary.push_str(&format!("- {} [Priority: {}]{}\n", 
                        task.title, task.priority, due_info));
                }
            }
        }

        if summary.is_empty() {
            summary.push_str("No recent memories or pending tasks found.");
        }

        Ok(summary)
    }
} 