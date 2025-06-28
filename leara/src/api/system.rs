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
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
// Import Serde for JSON serialization/deserialization
use serde::{Deserialize, Serialize};
// Import our local system models
use crate::models::system::SystemInfo;
use crate::models::AppState;
// Import tracing for structured logging
use tracing::info;
use std::process::Command;
use std::collections::HashSet;
use tokio::process::Command as TokioCommand;
use std::fs;
use std::path::Path;
use std::env;
use serde_json::json;
use std::os::unix::fs::PermissionsExt;

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

/// Request structure for executing system commands
#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteCommandRequest {
    /// The command to execute
    pub command: String,
    /// Optional arguments for the command
    pub args: Option<Vec<String>>,
    /// Working directory for the command (optional)
    pub working_dir: Option<String>,
    /// Whether to require user confirmation for this command
    pub require_confirmation: Option<bool>,
}

/// Response structure for command execution
#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteCommandResponse {
    /// Whether the command executed successfully
    pub success: bool,
    /// The command that was executed
    pub command: String,
    /// Standard output from the command
    pub stdout: String,
    /// Standard error from the command
    pub stderr: String,
    /// Exit code from the command
    pub exit_code: i32,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
    /// Timestamp when the command was executed
    pub timestamp: String,
}

/// Error response for command execution
#[derive(Debug, Serialize, Deserialize)]
pub struct CommandError {
    /// Human-readable error message
    pub error: String,
    /// Whether the command was blocked for safety reasons
    pub blocked: bool,
}

/// Command history entry
#[derive(Debug, Serialize, Deserialize)]
pub struct CommandHistory {
    pub id: i64,
    pub command: String,
    pub args: Option<String>,
    pub working_dir: Option<String>,
    pub success: bool,
    pub exit_code: i32,
    pub execution_time_ms: u64,
    pub timestamp: String,
    pub user_confirmed: bool,
}

/// Query structure for command history
#[derive(Debug, Serialize, Deserialize)]
pub struct CommandHistoryQuery {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub success_only: Option<bool>,
}

/// Response structure for command history
#[derive(Debug, Serialize, Deserialize)]
pub struct CommandHistoryResponse {
    pub commands: Vec<CommandHistory>,
    pub total: i64,
}

// Whitelist of safe commands that don't require confirmation
const SAFE_COMMANDS: &[&str] = &[
    "ls", "pwd", "whoami", "date", "uptime", "free", "df", "ps", "top", "htop",
    "cat", "head", "tail", "grep", "find", "locate", "which", "whereis",
    "echo", "printf", "wc", "sort", "uniq", "cut", "paste", "join",
    "mkdir", "rmdir", "touch", "cp", "mv", "rm", "chmod", "chown",
    "git", "npm", "cargo", "python", "node", "bash", "sh", "zsh", "fish",
    "code", "vim", "nano", "emacs", "firefox", "chrome", "chromium",
    "xdg-open", "open", "start", "xdg-mime", "file", "type", "stat",
    "du", "tree", "ncdu", "rsync", "scp", "ssh", "curl", "wget",
    "ping", "traceroute", "netstat", "ss", "ip", "ifconfig", "route",
    "systemctl", "service", "journalctl", "log", "dmesg", "lspci", "lsusb",
];

// Dangerous commands that should be blocked
const DANGEROUS_COMMANDS: &[&str] = &[
    "rm -rf /", "rm -rf /*", "dd if=/dev/zero", "mkfs", "fdisk", "parted",
    "chmod 777", "chown root", "sudo rm", "sudo dd", "sudo mkfs",
    "format", "del", "erase", "wipe", "shred", "srm",
];

/// Execute a system command with safety measures
/// 
/// This endpoint allows Leara to execute system commands with proper
/// safety measures including command whitelisting and user confirmation.
/// 
/// # Arguments
/// * `payload` - Command execution request with safety parameters
/// 
/// # Returns
/// * `Ok(Json<ExecuteCommandResponse>)` - Command execution result
/// * `Err((StatusCode, Json<CommandError>))` - Error response
pub async fn execute_command(
    State(state): State<AppState>,
    Json(payload): Json<ExecuteCommandRequest>,
) -> Result<Json<ExecuteCommandResponse>, (StatusCode, Json<CommandError>)> {
    let start_time = std::time::Instant::now();
    
    // Check if command is in dangerous commands list
    for dangerous in DANGEROUS_COMMANDS {
        if payload.command.contains(dangerous) {
            return Err((StatusCode::FORBIDDEN, Json(CommandError {
                error: format!("Command blocked for safety: {}", payload.command),
                blocked: true,
            })));
        }
    }
    
    // Check if command requires confirmation (not in safe list)
    let is_safe = SAFE_COMMANDS.contains(&payload.command.as_str());
    let require_confirmation = payload.require_confirmation.unwrap_or(!is_safe);
    
    if require_confirmation {
        // For now, we'll allow safe commands but require confirmation for others
        // In a real implementation, you'd want to implement a confirmation system
        if !is_safe {
            return Err((StatusCode::BAD_REQUEST, Json(CommandError {
                error: format!("Command requires user confirmation: {}", payload.command),
                blocked: false,
            })));
        }
    }
    
    // Execute the command
    let mut cmd = TokioCommand::new(&payload.command);
    
    if let Some(ref args) = payload.args {
        cmd.args(args);
    }
    
    if let Some(ref working_dir) = payload.working_dir {
        cmd.current_dir(working_dir);
    }
    
    let start_execution = std::time::Instant::now();
    let output = cmd.output().await;
    let execution_time = start_execution.elapsed().as_millis() as u64;
    
    match output {
        Ok(output) => {
            let success = output.status.success();
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let exit_code = output.status.code().unwrap_or(-1);
            
            // Store command in history
            let db = state.db.get().unwrap();
            let _ = crate::db::queries::store_command_history(&db, &payload.command, &payload.args, &payload.working_dir, success, exit_code, execution_time, true);
            
            Ok(Json(ExecuteCommandResponse {
                success,
                command: payload.command,
                stdout,
                stderr,
                exit_code,
                execution_time_ms: execution_time,
                timestamp: chrono::Utc::now().to_rfc3339(),
            }))
        }
        Err(e) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(CommandError {
                error: format!("Failed to execute command: {}", e),
                blocked: false,
            })))
        }
    }
}

/// Get command execution history
/// 
/// This endpoint retrieves the history of commands executed by Leara,
/// useful for auditing and debugging purposes.
/// 
/// # Arguments
/// * `query` - Query parameters for filtering history
/// 
/// # Returns
/// * `Ok(Json<CommandHistoryResponse>)` - Command history
/// * `Err((StatusCode, Json<CommandError>))` - Error response
pub async fn get_command_history(
    State(state): State<AppState>,
    Query(query): Query<CommandHistoryQuery>,
) -> Result<Json<CommandHistoryResponse>, (StatusCode, Json<CommandError>)> {
    let db = state.db.get().unwrap();
    match crate::db::queries::get_command_history(&db, &query) {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(CommandError {
            error: e.to_string(),
            blocked: false,
        }))),
    }
}

/// Get list of available applications dynamically
/// 
/// This endpoint scans .desktop files and $PATH for available applications.
/// 
/// # Returns
/// * `Ok(Json<serde_json::Value>)` - List of available applications
/// * `Err((StatusCode, Json<CommandError>))` - Error response
pub async fn get_available_apps() -> Result<Json<serde_json::Value>, (StatusCode, Json<CommandError>)> {
    let mut apps = Vec::new();
    let mut seen = HashSet::new();

    // Fix the temporary value dropped error by binding the .local path to a variable first
    let home_dir = env::var("HOME").unwrap_or_default();
    let local_applications = format!("{}/.local/share/applications", home_dir);
    let desktop_dirs = vec![
        "/usr/share/applications",
        "/usr/local/share/applications",
        &local_applications,
    ];
    for dir in desktop_dirs {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == "desktop").unwrap_or(false) {
                    if let Ok(content) = fs::read_to_string(&path) {
                        let mut name = None;
                        let mut exec = None;
                        let mut comment = None;
                        let mut category = None;
                        for line in content.lines() {
                            if line.starts_with("Name=") {
                                name = Some(line[5..].trim().to_string());
                            } else if line.starts_with("Exec=") {
                                exec = Some(line[5..].split_whitespace().next().unwrap_or("").to_string());
                            } else if line.starts_with("Comment=") {
                                comment = Some(line[8..].trim().to_string());
                            } else if line.starts_with("Categories=") {
                                category = Some(line[11..].split(';').next().unwrap_or("").to_string());
                            }
                        }
                        if let (Some(name), Some(exec)) = (name, exec) {
                            // Avoid duplicates
                            if seen.insert(exec.clone()) {
                                apps.push(json!({
                                    "name": name,
                                    "command": exec,
                                    "description": comment.unwrap_or_default(),
                                    "category": category.unwrap_or_default(),
                                }));
                            }
                        }
                    }
                }
            }
        }
    }

    // Optionally, scan $PATH for executables not already listed
    if let Ok(path_var) = env::var("PATH") {
        for dir in path_var.split(':') {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() && path.metadata().map(|m| m.permissions().mode() & 0o111 != 0).unwrap_or(false) {
                        let cmd = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                        if seen.insert(cmd.clone()) {
                            apps.push(json!({
                                "name": cmd.clone(),
                                "command": cmd,
                                "description": "",
                                "category": "cli"
                            }));
                        }
                    }
                }
            }
        }
    }

    Ok(Json(json!({ "applications": apps })))
}

/// Create router for system-related endpoints
pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/execute", post(execute_command))
        .route("/history", get(get_command_history))
        .route("/apps", get(get_available_apps))
} 