/*
 * Leara AI Assistant - System Models
 * 
 * This module defines data structures for system information and monitoring
 * including hardware stats, resource usage, and system metadata.
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
 * File: src/models/system.rs
 * Purpose: System-related data models and structures
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub hostname: String,
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub cpu_count: usize,
    pub total_memory: u64,
    pub uptime: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub disk_usage: u64,
    pub network_rx: u64,
    pub network_tx: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub usage_percentage: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiskInfo {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub usage_percentage: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub status: String,
} 