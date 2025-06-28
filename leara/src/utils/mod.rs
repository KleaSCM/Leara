/*
 * Leara AI Assistant - Utils Module
 * 
 * This module contains utility functions and helper modules used throughout
 * the application for common operations and external integrations.
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
 * File: src/utils/mod.rs
 * Purpose: Utility functions and helper modules
 */

pub mod ollama;

/// Get current timestamp in ISO format
pub fn get_timestamp() -> String {
    chrono::Utc::now().to_rfc3339()
}

/// Generate a unique identifier
pub fn generate_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Format bytes into human readable format
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 4] = ["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    format!("{:.1} {}", size, UNITS[unit_index])
}

/// Formats a decimal value as a percentage string
/// 
/// Converts a decimal value (0.0 to 1.0 or any float) into a percentage
/// string with one decimal place. Useful for displaying progress,
/// statistics, or any ratio as a percentage.
/// 
/// # Arguments
/// * `value` - The decimal value to convert to percentage
/// 
/// # Returns
/// A formatted percentage string (e.g., "75.5%", "0.1%")
/// 
/// # Example
/// ```
/// assert_eq!(format_percentage(0.755), "75.5%");
/// assert_eq!(format_percentage(0.001), "0.1%");
/// assert_eq!(format_percentage(1.0), "100.0%");
/// ```
pub fn format_percentage(value: f32) -> String {
    format!("{:.1}%", value)
} 