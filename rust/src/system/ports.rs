//! Port scanning utilities
//!
//! Enumerate TCP socket bindings and process information.

use crate::state::{PortBinding, PortScanResult};
use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};
use std::collections::HashSet;
use sysinfo::System;

/// List all TCP bindings for a specific port
pub fn list_bindings(port: u16) -> PortScanResult {
    let mut bindings = Vec::new();
    let mut sys = System::new();
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    // Get all TCP sockets
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP;

    let sockets = match get_sockets_info(af_flags, proto_flags) {
        Ok(s) => s,
        Err(e) => {
            tracing::error!("Failed to get socket info: {}", e);
            return PortScanResult::default();
        }
    };

    for socket in sockets {
        if let ProtocolSocketInfo::Tcp(tcp_info) = socket.protocol_socket_info {
            // Filter by port
            if tcp_info.local_port != port {
                continue;
            }

            let local_ip = tcp_info.local_addr.to_string();
            let pid = socket.associated_pids.first().copied().unwrap_or(0);

            // Check if this is a system/kernel socket (PID 0 or 4)
            let is_system = pid == 0 || pid == 4;

            // Try to get process info and detect orphans
            let (process_name, is_orphan) = if is_system {
                // System process - use special names
                let name = if pid == 0 {
                    "[System Idle]".to_string()
                } else {
                    "[System]".to_string()
                };
                (name, false)
            } else if pid > 0 {
                match sys.process(sysinfo::Pid::from_u32(pid)) {
                    Some(p) => (p.name().to_string_lossy().to_string(), false),
                    None => {
                        // Process doesn't exist - this is an orphaned socket!
                        ("<orphaned>".to_string(), true)
                    }
                }
            } else {
                ("<unknown>".to_string(), false)
            };

            // Determine address type
            let is_loopback = local_ip.starts_with("127.") || local_ip == "::1";
            let is_all_interfaces = local_ip == "0.0.0.0" || local_ip == "::";

            bindings.push(PortBinding {
                pid,
                process_name,
                local_ip,
                local_port: tcp_info.local_port,
                state: format!("{:?}", tcp_info.state),
                is_loopback,
                is_all_interfaces,
                is_orphan,
                is_system,
            });
        }
    }

    // Detect conflicts: PIDs that have both loopback and all-interfaces bindings
    let loopback_pids: HashSet<u32> = bindings
        .iter()
        .filter(|b| b.is_loopback)
        .map(|b| b.pid)
        .collect();
    let all_interface_pids: HashSet<u32> = bindings
        .iter()
        .filter(|b| b.is_all_interfaces)
        .map(|b| b.pid)
        .collect();
    let conflict_pids: Vec<u32> = loopback_pids
        .intersection(&all_interface_pids)
        .copied()
        .collect();

    // Collect orphaned PIDs
    let orphan_pids: Vec<u32> = bindings
        .iter()
        .filter(|b| b.is_orphan)
        .map(|b| b.pid)
        .collect();

    PortScanResult {
        bindings,
        conflict_pids,
        orphan_pids,
    }
}

/// Kill a process by PID
pub fn kill_process(pid: u32) -> Result<(), String> {
    if pid == 0 {
        return Err("Cannot kill PID 0".to_string());
    }

    let mut sys = System::new();
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    if let Some(process) = sys.process(sysinfo::Pid::from_u32(pid)) {
        if process.kill() {
            tracing::info!("Killed process {}", pid);
            Ok(())
        } else {
            Err(format!("Failed to kill process {}", pid))
        }
    } else {
        Err(format!("Process {} not found", pid))
    }
}

/// Suggest a free port in the given range
pub fn suggest_free_port(start: u16, end: u16) -> Option<u16> {
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP;

    let sockets = match get_sockets_info(af_flags, proto_flags) {
        Ok(s) => s,
        Err(_) => return None,
    };

    // Collect all ports currently in use
    let used_ports: HashSet<u16> = sockets
        .iter()
        .filter_map(|s| {
            if let ProtocolSocketInfo::Tcp(tcp) = &s.protocol_socket_info {
                Some(tcp.local_port)
            } else {
                None
            }
        })
        .collect();

    // Find first free port in range
    (start..=end).find(|port| !used_ports.contains(port))
}

/// Attempt to force close an orphaned socket
/// 
/// This tries multiple approaches:
/// 1. Check if it's an http.sys registration and show diagnostics
/// 2. Attempt to use netsh to show related state
/// 
/// Note: Truly orphaned sockets often require a system restart to clear.
pub async fn force_close_socket(binding: &PortBinding) -> Result<String, String> {
    use std::process::Stdio;
    use tokio::process::Command;

    // First, check http.sys URL reservations for this port
    let urlacl_result = Command::new("netsh")
        .args(["http", "show", "urlacl"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await;

    let mut diagnostics = Vec::new();

    if let Ok(output) = urlacl_result {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let port_str = format!(":{}", binding.local_port);
        
        // Check if any URL reservation mentions this port
        let matching_lines: Vec<&str> = stdout
            .lines()
            .filter(|line| line.contains(&port_str))
            .collect();

        if !matching_lines.is_empty() {
            diagnostics.push(format!(
                "Found http.sys URL reservations for port {}:\n{}",
                binding.local_port,
                matching_lines.join("\n")
            ));
        }
    }

    // Check http.sys service state
    let servicestate_result = Command::new("netsh")
        .args(["http", "show", "servicestate", "view=requestq"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await;

    if let Ok(output) = servicestate_result {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let port_str = format!(":{}", binding.local_port);
        
        let matching_lines: Vec<&str> = stdout
            .lines()
            .filter(|line| line.contains(&port_str))
            .collect();

        if !matching_lines.is_empty() {
            diagnostics.push(format!(
                "Found http.sys active listeners on port {}:\n{}",
                binding.local_port,
                matching_lines.join("\n")
            ));
        }
    }

    // If we found http.sys involvement, report it
    if !diagnostics.is_empty() {
        return Ok(format!(
            "This appears to be an http.sys managed socket. Diagnostics:\n\n{}\n\n\
            To clear, try: 1) Stop IIS/HTTP services, 2) Run 'net stop http' as admin, 3) Restart",
            diagnostics.join("\n\n")
        ));
    }

    // Try to identify if there's a TIME_WAIT or similar state
    // Unfortunately Windows doesn't have a direct way to force close TCP connections
    // The best we can do is provide guidance
    
    Err(format!(
        "Cannot programmatically close orphaned socket (PID {} no longer exists).\n\n\
        This socket is likely in a kernel cleanup state. Options:\n\
        1. Wait 2-4 minutes for TCP TIME_WAIT to expire\n\
        2. Run 'net stop http && net start http' as Admin if http.sys related\n\
        3. Restart the machine to fully clear all socket states",
        binding.pid
    ))
}
