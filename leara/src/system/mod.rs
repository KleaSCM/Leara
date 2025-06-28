use crate::models::{SystemInfo, MemoryInfo, DiskInfo, ProcessInfo};

pub fn get_system_info() -> SystemInfo {
    // For now, return basic system info without sysinfo
    // TODO: Implement proper system monitoring
    let memory_info = MemoryInfo {
        total: 0,
        used: 0,
        available: 0,
        usage_percentage: 0.0,
    };

    let disk_info = DiskInfo {
        total: 0,
        used: 0,
        available: 0,
        usage_percentage: 0.0,
    };

    SystemInfo {
        os_name: std::env::consts::OS.to_string(),
        os_version: "Unknown".to_string(),
        hostname: hostname::get()
            .map(|h| h.to_string_lossy().to_string())
            .unwrap_or_else(|_| "Unknown".to_string()),
        cpu_usage: 0.0,
        memory_usage: memory_info,
        disk_usage: disk_info,
        uptime: 0,
    }
}

pub fn get_processes() -> Vec<ProcessInfo> {
    // TODO: Implement process monitoring
    vec![]
} 