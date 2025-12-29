//! Process inspection utilities
//!
//! List processes by handle count, memory, CPU usage.

use sysinfo::{ProcessesToUpdate, System};
use std::process::{Command, Stdio};

/// Process info for display
#[derive(Debug, Clone, Default)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub memory_mb: f64,
    pub cpu_percent: f32,
    pub handle_count: u32,
    pub status: String,
}

/// Get top processes by memory usage
pub fn get_top_by_memory(limit: usize) -> Vec<ProcessInfo> {
    let mut sys = System::new_all();
    sys.refresh_processes(ProcessesToUpdate::All, true);

    let mut processes: Vec<ProcessInfo> = sys
        .processes()
        .iter()
        .map(|(pid, proc)| ProcessInfo {
            pid: pid.as_u32(),
            name: proc.name().to_string_lossy().to_string(),
            memory_mb: proc.memory() as f64 / 1024.0 / 1024.0,
            cpu_percent: proc.cpu_usage(),
            handle_count: 0, // Not available from sysinfo
            status: format!("{:?}", proc.status()),
        })
        .collect();

    processes.sort_by(|a, b| b.memory_mb.partial_cmp(&a.memory_mb).unwrap_or(std::cmp::Ordering::Equal));
    processes.truncate(limit);
    processes
}

/// Get top processes by CPU usage
pub fn get_top_by_cpu(limit: usize) -> Vec<ProcessInfo> {
    let mut sys = System::new_all();
    // Need to refresh twice for CPU usage to be accurate
    sys.refresh_processes(ProcessesToUpdate::All, true);
    std::thread::sleep(std::time::Duration::from_millis(200));
    sys.refresh_processes(ProcessesToUpdate::All, true);

    let mut processes: Vec<ProcessInfo> = sys
        .processes()
        .iter()
        .map(|(pid, proc)| ProcessInfo {
            pid: pid.as_u32(),
            name: proc.name().to_string_lossy().to_string(),
            memory_mb: proc.memory() as f64 / 1024.0 / 1024.0,
            cpu_percent: proc.cpu_usage(),
            handle_count: 0,
            status: format!("{:?}", proc.status()),
        })
        .collect();

    processes.sort_by(|a, b| b.cpu_percent.partial_cmp(&a.cpu_percent).unwrap_or(std::cmp::Ordering::Equal));
    processes.truncate(limit);
    processes
}

/// Get top processes by handle count (Windows only - uses PowerShell)
/// This is useful for detecting handle leaks
pub fn get_top_by_handles(limit: usize) -> Vec<ProcessInfo> {
    // Use PowerShell to get handle counts - this is Windows-specific
    let ps_script = r#"
Get-Process | Sort-Object HandleCount -Descending | Select-Object -First 50 | 
ForEach-Object { 
    "$($_.Id),$($_.ProcessName),$($_.HandleCount),$([math]::Round($_.WorkingSet64/1MB,1))" 
}
"#;

    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", ps_script])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let mut processes: Vec<ProcessInfo> = stdout
                .lines()
                .filter_map(|line| {
                    let parts: Vec<&str> = line.split(',').collect();
                    if parts.len() >= 4 {
                        Some(ProcessInfo {
                            pid: parts[0].parse().unwrap_or(0),
                            name: parts[1].to_string(),
                            handle_count: parts[2].parse().unwrap_or(0),
                            memory_mb: parts[3].parse().unwrap_or(0.0),
                            cpu_percent: 0.0, // Not available from this query
                            status: "Running".to_string(),
                        })
                    } else {
                        None
                    }
                })
                .collect();

            processes.truncate(limit);
            processes
        }
        Err(_) => Vec::new(),
    }
}

/// Kill a process by PID (requires elevated privileges for some processes)
pub fn kill_process(pid: u32) -> Result<(), String> {
    let sys = System::new_all();
    let pid = sysinfo::Pid::from_u32(pid);
    
    if let Some(process) = sys.process(pid) {
        if process.kill() {
            Ok(())
        } else {
            Err("Failed to kill process - may require Administrator".to_string())
        }
    } else {
        Err("Process not found".to_string())
    }
}

/// Get system overview (total memory, CPU count, etc.)
pub fn get_system_info() -> SystemOverview {
    let sys = System::new_all();
    
    SystemOverview {
        total_memory_gb: sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0,
        used_memory_gb: sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0,
        cpu_count: sys.cpus().len(),
        process_count: sys.processes().len(),
    }
}

/// System overview stats
#[derive(Debug, Clone, Default)]
pub struct SystemOverview {
    pub total_memory_gb: f64,
    pub used_memory_gb: f64,
    pub cpu_count: usize,
    pub process_count: usize,
}

