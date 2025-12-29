//! Network utilities
//!
//! Network diagnostics and quick-fix commands.

use crate::state::{CommandOutput, NetworkAdapter};
use std::process::Stdio;
use std::time::Instant;
use sysinfo::Networks;

/// Run a command and capture output
async fn run_command(program: &str, args: &[&str]) -> CommandOutput {
    let command_str = format!("{} {}", program, args.join(" "));
    let start = Instant::now();

    let result = tokio::process::Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await;

    let duration_ms = start.elapsed().as_millis() as u64;

    match result {
        Ok(output) => CommandOutput {
            command: command_str,
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code().unwrap_or(-1),
            duration_ms,
        },
        Err(e) => CommandOutput {
            command: command_str,
            stdout: String::new(),
            stderr: format!("Failed to execute: {}", e),
            exit_code: -1,
            duration_ms,
        },
    }
}

/// Flush DNS cache
pub async fn flush_dns() -> CommandOutput {
    run_command("ipconfig", &["/flushdns"]).await
}

/// Release and renew IP address
pub async fn renew_ip() -> Vec<CommandOutput> {
    let release = run_command("ipconfig", &["/release"]).await;
    let renew = run_command("ipconfig", &["/renew"]).await;
    vec![release, renew]
}

/// Reset Winsock (requires admin)
pub async fn winsock_reset() -> CommandOutput {
    run_command("netsh", &["winsock", "reset"]).await
}

/// Restart HTTP.sys service (requires admin)
/// This clears orphaned HTTP sockets that may be holding ports
pub async fn restart_http_service() -> Vec<CommandOutput> {
    let stop = run_command("net", &["stop", "http", "/y"]).await;
    // Small delay to ensure service fully stops
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    let start = run_command("net", &["start", "http"]).await;
    vec![stop, start]
}

/// Ping a host
pub async fn ping_host(host: &str) -> CommandOutput {
    run_command("ping", &["-n", "4", host]).await
}

/// HTTP HEAD request
pub async fn http_head(url: &str) -> CommandOutput {
    // Try curl first (modern Windows has it)
    if which::which("curl").is_ok() {
        run_command("curl", &["-I", "--connect-timeout", "10", url]).await
    } else {
        // Fallback to PowerShell
        let script = format!(
            "Invoke-WebRequest -Uri '{}' -Method HEAD -UseBasicParsing | Select-Object -ExpandProperty Headers | ConvertTo-Json",
            url
        );
        run_command("powershell", &["-NoProfile", "-Command", &script]).await
    }
}

/// Get network adapters with their addresses
/// Reserved for future network adapter display feature
#[allow(dead_code)]
pub fn get_network_adapters() -> Vec<NetworkAdapter> {
    let networks = Networks::new_with_refreshed_list();
    
    networks
        .iter()
        .map(|(name, _data)| {
            // sysinfo doesn't give us IP addresses directly in newer versions,
            // so we'll use a simpler approach for now
            NetworkAdapter {
                name: name.to_string(),
                addresses: vec![], // We'll populate this via ipconfig parsing if needed
                is_up: true, // Assume up if listed
            }
        })
        .collect()
}

/// Get detailed network adapter info using ipconfig
/// Reserved for future network adapter display feature
#[allow(dead_code)]
pub async fn get_adapter_details() -> CommandOutput {
    run_command("ipconfig", &["/all"]).await
}
