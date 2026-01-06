//! Port scanning utilities
//!
//! Enumerate TCP socket bindings and process information.
//! Supports multi-layer detection: Windows TCP stack, Docker, and WSL.

use crate::state::{BindingSource, DockerPortBinding, PortBinding, PortScanResult, WslPortBinding};
use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};
use std::collections::HashSet;
use std::net::{TcpListener, SocketAddr};
use sysinfo::System;

/// Test if a port is actually in use at the kernel level via socket probe
/// Returns true if the port is in use (bind fails), false if free
pub fn probe_port_in_use(port: u16) -> bool {
    // Try both IPv4 and IPv6
    let addrs = [
        SocketAddr::from(([0, 0, 0, 0], port)),
        SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], port)),
    ];
    
    for addr in &addrs {
        match TcpListener::bind(addr) {
            Ok(_listener) => {
                // Successfully bound - port is free (listener drops and releases)
                return false;
            }
            Err(e) => {
                // Error 10048 (WSAEADDRINUSE) means port is in use
                if let Some(code) = e.raw_os_error() {
                    if code == 10048 {
                        return true;
                    }
                }
                // Other errors (permission denied, etc.) - continue trying
            }
        }
    }
    
    false
}

/// Check if Docker Desktop is running
pub async fn is_docker_running() -> bool {
    use std::process::Stdio;
    use tokio::process::Command;
    
    match Command::new("docker")
        .args(["info"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await
    {
        Ok(status) => status.success(),
        Err(_) => false,
    }
}

/// Get Docker container port mappings for a specific port
pub async fn get_docker_port_bindings(port: u16) -> Vec<DockerPortBinding> {
    use std::process::Stdio;
    use tokio::process::Command;
    
    let output = match Command::new("docker")
        .args(["ps", "--format", "{{.ID}}|{{.Names}}|{{.Image}}|{{.Ports}}"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .await
    {
        Ok(o) if o.status.success() => o,
        _ => return Vec::new(),
    };
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut bindings = Vec::new();
    
    for line in stdout.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 4 {
            continue;
        }
        
        let container_id = parts[0].to_string();
        let container_name = parts[1].to_string();
        let image = parts[2].to_string();
        let ports_str = parts[3];
        
        // Parse port mappings like "0.0.0.0:8080->80/tcp, :::8080->80/tcp"
        for port_mapping in ports_str.split(", ") {
            if let Some(parsed) = parse_docker_port_mapping(port_mapping, port) {
                bindings.push(DockerPortBinding {
                    container_id: container_id.clone(),
                    container_name: container_name.clone(),
                    image: image.clone(),
                    host_port: parsed.0,
                    container_port: parsed.1,
                    protocol: parsed.2,
                });
            }
        }
    }
    
    bindings
}

/// Parse a Docker port mapping string like "0.0.0.0:8080->80/tcp"
/// Returns (host_port, container_port, protocol) if it matches the target port
fn parse_docker_port_mapping(mapping: &str, target_port: u16) -> Option<(u16, u16, String)> {
    // Format: "0.0.0.0:8080->80/tcp" or ":::8080->80/tcp"
    let arrow_pos = mapping.find("->")?;
    let host_part = &mapping[..arrow_pos];
    let container_part = &mapping[arrow_pos + 2..];
    
    // Extract host port (after the last colon)
    let host_port: u16 = host_part.rsplit(':').next()?.parse().ok()?;
    
    if host_port != target_port {
        return None;
    }
    
    // Extract container port and protocol
    let (container_port_str, protocol) = if let Some(slash_pos) = container_part.find('/') {
        (&container_part[..slash_pos], container_part[slash_pos + 1..].to_string())
    } else {
        (container_part, "tcp".to_string())
    };
    
    let container_port: u16 = container_port_str.parse().ok()?;
    
    Some((host_port, container_port, protocol))
}

/// Get list of running WSL distros
pub async fn get_running_wsl_distros() -> Vec<String> {
    use std::process::Stdio;
    use tokio::process::Command;
    
    let output = match Command::new("wsl")
        .args(["--list", "--running", "--quiet"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .await
    {
        Ok(o) if o.status.success() => o,
        _ => return Vec::new(),
    };
    
    // WSL output is UTF-16LE on Windows
    let stdout = String::from_utf16_lossy(
        &output.stdout
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect::<Vec<u16>>()
    );
    
    stdout
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Get WSL port bindings for a specific port across all running distros
pub async fn get_wsl_port_bindings(port: u16) -> Vec<WslPortBinding> {
    use std::process::Stdio;
    use tokio::process::Command;
    
    let distros = get_running_wsl_distros().await;
    let mut bindings = Vec::new();
    
    for distro in distros {
        // Run ss -tlnp inside WSL to get listening ports
        // Format: State  Recv-Q Send-Q  Local Address:Port   Peer Address:Port  Process
        let output = match Command::new("wsl")
            .args(["-d", &distro, "--", "ss", "-tlnp"])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .output()
            .await
        {
            Ok(o) if o.status.success() => o,
            _ => continue,
        };
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        for line in stdout.lines().skip(1) { // Skip header
            if let Some(binding) = parse_ss_line(line, port, &distro) {
                bindings.push(binding);
            }
        }
    }
    
    bindings
}

/// Parse a line from `ss -tlnp` output
fn parse_ss_line(line: &str, target_port: u16, distro: &str) -> Option<WslPortBinding> {
    // Example: LISTEN  0  4096  0.0.0.0:8080  0.0.0.0:*  users:(("node",pid=1234,fd=21))
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 5 {
        return None;
    }
    
    // Local address is typically the 4th column
    let local_addr = parts[3];
    
    // Extract port from address (last part after :)
    let port_str = local_addr.rsplit(':').next()?;
    let port: u16 = port_str.parse().ok()?;
    
    if port != target_port {
        return None;
    }
    
    // Try to extract process info from the last column
    let (process_name, pid) = if let Some(last) = parts.last() {
        parse_ss_process_info(last)
    } else {
        ("unknown".to_string(), 0)
    };
    
    Some(WslPortBinding {
        distro: distro.to_string(),
        process_name,
        pid,
        port,
        local_addr: local_addr.to_string(),
    })
}

/// Parse process info from ss output like users:(("node",pid=1234,fd=21))
fn parse_ss_process_info(info: &str) -> (String, u32) {
    // Try to extract process name and PID
    if let Some(start) = info.find("((\"") {
        let rest = &info[start + 3..];
        if let Some(end) = rest.find('"') {
            let process_name = rest[..end].to_string();
            
            // Try to find pid=XXXX
            if let Some(pid_start) = rest.find("pid=") {
                let pid_rest = &rest[pid_start + 4..];
                if let Some(pid_end) = pid_rest.find(|c: char| !c.is_ascii_digit()) {
                    if let Ok(pid) = pid_rest[..pid_end].parse() {
                        return (process_name, pid);
                    }
                } else if let Ok(pid) = pid_rest.trim_end_matches(|c: char| !c.is_ascii_digit()).parse() {
                    return (process_name, pid);
                }
            }
            
            return (process_name, 0);
        }
    }
    
    ("unknown".to_string(), 0)
}

/// Enhanced port scan that checks Windows TCP stack, Docker, and WSL
pub async fn list_bindings_enhanced(port: u16) -> PortScanResult {
    // Start with traditional Windows scan
    let mut result = list_bindings(port);
    
    // Socket probe to detect shadow bindings
    let port_in_use = probe_port_in_use(port);
    let has_visible_bindings = !result.bindings.is_empty();
    
    // Check Docker (silently fails if not running)
    let docker_bindings = if is_docker_running().await {
        get_docker_port_bindings(port).await
    } else {
        Vec::new()
    };
    
    // Add Docker bindings to main bindings list
    for db in &docker_bindings {
        result.bindings.push(PortBinding {
            pid: 0,
            process_name: format!("{} ({})", db.container_name, db.image),
            local_ip: "0.0.0.0".to_string(),
            local_port: db.host_port,
            state: "LISTEN".to_string(),
            is_loopback: false,
            is_all_interfaces: true,
            is_orphan: false,
            is_system: false,
            source: BindingSource::Docker,
            source_detail: db.container_id.clone(),
        });
    }
    
    // Check WSL distros
    let wsl_bindings = get_wsl_port_bindings(port).await;
    
    // Add WSL bindings to main bindings list (but avoid duplicates with Docker)
    // Docker containers often show up in both - dedupe by checking if Docker already has it
    let docker_has_port = !docker_bindings.is_empty();
    
    for wb in &wsl_bindings {
        // Skip if this looks like a Docker process (common patterns)
        let is_docker_process = wb.process_name.contains("docker")
            || wb.process_name.contains("containerd")
            || wb.process_name.contains("com.docker");
        
        if docker_has_port && is_docker_process {
            continue;
        }
        
        result.bindings.push(PortBinding {
            pid: wb.pid,
            process_name: format!("{} [WSL:{}]", wb.process_name, wb.distro),
            local_ip: wb.local_addr.rsplit(':').nth(1).unwrap_or("0.0.0.0").to_string(),
            local_port: wb.port,
            state: "LISTEN".to_string(),
            is_loopback: wb.local_addr.starts_with("127.") || wb.local_addr.starts_with("[::1]"),
            is_all_interfaces: wb.local_addr.starts_with("0.0.0.0") || wb.local_addr.starts_with("[::]") || wb.local_addr.starts_with("*"),
            is_orphan: false,
            is_system: false,
            source: BindingSource::Wsl,
            source_detail: wb.distro.clone(),
        });
    }
    
    // Store raw bindings for reference
    result.docker_bindings = docker_bindings;
    result.wsl_bindings = wsl_bindings;
    
    // Detect shadow binding: port is in use but no visible source
    result.shadow_detected = port_in_use && !has_visible_bindings && result.docker_bindings.is_empty() && result.wsl_bindings.is_empty();
    
    // If we detected a shadow binding, add a placeholder entry
    if result.shadow_detected {
        result.bindings.push(PortBinding {
            pid: 0,
            process_name: "<shadow binding>".to_string(),
            local_ip: "?".to_string(),
            local_port: port,
            state: "UNKNOWN".to_string(),
            is_loopback: false,
            is_all_interfaces: true,
            is_orphan: false,
            is_system: false,
            source: BindingSource::UnknownShadow,
            source_detail: "Port in use but source not detected".to_string(),
        });
    }
    
    result
}

/// List all TCP bindings for a specific port (Windows TCP stack only)
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
                source: BindingSource::Windows,
                source_detail: String::new(),
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
        docker_bindings: Vec::new(),
        wsl_bindings: Vec::new(),
        shadow_detected: false,
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

/// Suggest a free port in the given range (uses socket probe for accuracy)
pub fn suggest_free_port(start: u16, end: u16) -> Option<u16> {
    // Use socket probe for accurate detection (catches Docker/WSL too)
    (start..=end).find(|&port| !probe_port_in_use(port))
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
