//! Network utilities
//!
//! Network diagnostics and quick-fix commands.

use crate::state::{CommandOutput, NetworkAdapter};
use crate::system::command::run_command;
use sysinfo::Networks;


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
