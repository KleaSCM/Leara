use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os_name: String,
    pub os_version: String,
    pub hostname: String,
    pub cpu_usage: f32,
    pub memory_usage: MemoryInfo,
    pub disk_usage: DiskInfo,
    pub uptime: u64,
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