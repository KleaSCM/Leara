/*
 * Leara AI Assistant - System Module
 * 
 * This module provides system-level functionality including memory management,
 * task tracking, and context awareness for the AI assistant.
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
 * File: src/system/mod.rs
 * Purpose: System-level functionality and memory management
 */

pub mod memory_service;

pub use memory_service::MemoryService;

use crate::models::system::SystemInfo;

/// Get current system information
pub fn get_system_info() -> SystemInfo {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();

    SystemInfo {
        hostname: sysinfo::System::host_name().unwrap_or_else(|| "unknown".to_string()),
        os_name: sysinfo::System::name().unwrap_or_else(|| "unknown".to_string()),
        os_version: sysinfo::System::os_version().unwrap_or_else(|| "unknown".to_string()),
        kernel_version: sysinfo::System::kernel_version().unwrap_or_else(|| "unknown".to_string()),
        cpu_count: sys.physical_core_count().unwrap_or(0),
        total_memory: sys.total_memory(),
        uptime: sysinfo::System::uptime(),
    }
} 