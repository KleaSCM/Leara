/*
 * Leara AI Assistant - System API Handler
 * 
 * This module provides system information and monitoring endpoints.
 * Returns system stats, resource usage, and hardware information.
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
 * File: src/api/system.rs
 * Purpose: System information API endpoint handlers
 */

// Import Axum web framework components for HTTP handling
use axum::{
    http::StatusCode,
    response::Json,
};
// Import Serde for JSON serialization/deserialization
use serde::Deserialize;
// Import our local system models
use crate::models::system::SystemInfo;
// Import tracing for structured logging
use tracing::info;

/// Retrieve comprehensive system information and hardware statistics
/// 
/// This function gathers detailed information about the host system including:
/// - Operating system details (name, version, kernel)
/// - Hardware specifications (CPU cores, total memory)
/// - System uptime and hostname
/// 
/// The information is useful for:
/// - System monitoring and diagnostics
/// - Performance analysis and capacity planning
/// - Debugging system-related issues
/// - Resource utilization tracking
/// 
/// # Returns
/// * `(StatusCode::OK, Json<SystemInfo>)` - Successfully retrieved system information
/// 
/// # Example Response
/// ```json
/// {
///   "hostname": "nyxaria",
///   "os_name": "Linux",
///   "os_version": "6.15.3-zen1-1-zen",
///   "kernel_version": "6.15.3-zen1-1-zen",
///   "cpu_count": 8,
///   "total_memory": 32106127360,
///   "uptime": 76320
/// }
/// ```
/// 
/// # Usage Examples
/// ```bash
/// # Get system information
/// curl http://localhost:3000/api/system/info
/// 
/// # Get system info with pretty formatting
/// curl http://localhost:3000/api/system/info | jq
/// 
/// # Monitor system resources
/// watch -n 5 'curl -s http://localhost:3000/api/system/info | jq ".total_memory"'
/// ```
/// 
/// # Performance Considerations
/// - The sysinfo library performs system calls to gather information
/// - Response time may vary depending on system load and hardware
/// - Consider caching for high-frequency monitoring scenarios
pub async fn get_system_info() -> (StatusCode, Json<SystemInfo>) {
    // Log the system information request for monitoring and debugging
    info!("Fetching system information");
    
    // Initialize sysinfo with all system components
    // This creates a comprehensive system information gatherer
    let mut sys = sysinfo::System::new_all();
    // Refresh all system information to get current values
    // This ensures we have the most up-to-date system statistics
    sys.refresh_all();

    // Build the system information response
    // Each field is gathered from the appropriate sysinfo method
    let system_info = SystemInfo {
        // Get the system hostname (e.g., "nyxaria", "server-01")
        hostname: sysinfo::System::host_name().unwrap_or_else(|| "unknown".to_string()),
        // Get the operating system name (e.g., "Linux", "Windows", "macOS")
        os_name: sysinfo::System::name().unwrap_or_else(|| "unknown".to_string()),
        // Get the OS version string (e.g., "6.15.3-zen1-1-zen", "10.0.19045")
        os_version: sysinfo::System::os_version().unwrap_or_else(|| "unknown".to_string()),
        // Get the kernel version (e.g., "6.15.3-zen1-1-zen", "NT 10.0")
        kernel_version: sysinfo::System::kernel_version().unwrap_or_else(|| "unknown".to_string()),
        // Get the number of physical CPU cores (not logical cores/hyperthreading)
        cpu_count: sys.physical_core_count().unwrap_or(0),
        // Get total system memory in bytes (RAM)
        total_memory: sys.total_memory(),
        // Get system uptime in seconds since last boot
        uptime: sysinfo::System::uptime(),
    };

    // Return HTTP 200 OK with comprehensive system information
    // This provides a complete snapshot of the current system state
    (StatusCode::OK, Json(system_info))
} 