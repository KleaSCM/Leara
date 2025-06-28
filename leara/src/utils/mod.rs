/*
 * Leara AI Assistant - Utility Functions
 * 
 * This file contains utility functions for the Leara AI Assistant backend.
 * Provides common functionality like ID generation, timestamp handling,
 * and data formatting utilities.
 * 
 * Copyright (c) 2024 Leara AI Assistant Contributors
 * 
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 
 * Author: KleaSCM
 * Created: 2024-06-28
 * Last Modified: 2024-06-28
 * Version: 0.1.0
 * 
 * File: src/utils/mod.rs
 * Purpose: Utility functions for ID generation, timestamps, and data formatting
 */

use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// Generates a unique identifier using UUID v4
/// 
/// Returns a random UUID string that can be used as a unique identifier
/// for database records, API requests, or any other entity that needs
/// a globally unique identifier.
/// 
/// # Returns
/// A string containing a UUID v4 in the format: `xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx`
/// 
/// # Example
/// ```
/// let id = generate_id();
/// println!("Generated ID: {}", id); // e.g., "550e8400-e29b-41d4-a716-446655440000"
/// ```
pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

/// Gets the current Unix timestamp in seconds
/// 
/// Returns the number of seconds elapsed since January 1, 1970 UTC
/// (Unix epoch). This is useful for timestamping events, calculating
/// durations, or storing time values in databases.
/// 
/// # Returns
/// A u64 representing the current Unix timestamp in seconds
/// 
/// # Example
/// ```
/// let timestamp = get_timestamp();
/// println!("Current timestamp: {}", timestamp); // e.g., 1640995200
/// ```
pub fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Formats a byte count into a human-readable string with appropriate units
/// 
/// Converts raw byte counts into a more readable format by automatically
/// selecting the most appropriate unit (B, KB, MB, GB) and formatting
/// the number with one decimal place.
/// 
/// # Arguments
/// * `bytes` - The number of bytes to format
/// 
/// # Returns
/// A formatted string with the appropriate unit (e.g., "1.5 MB", "2.3 GB")
/// 
/// # Example
/// ```
/// assert_eq!(format_bytes(1024), "1.0 KB");
/// assert_eq!(format_bytes(1536), "1.5 KB");
/// assert_eq!(format_bytes(1073741824), "1.0 GB");
/// ```
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 4] = ["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    // Convert to the largest appropriate unit
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