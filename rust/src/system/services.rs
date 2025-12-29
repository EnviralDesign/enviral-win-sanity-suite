//! Windows services utilities
//!
//! List, start, and stop Windows services.

use crate::state::CommandOutput;
use std::process::Stdio;
use std::time::Instant;

/// Run a PowerShell command and capture output
async fn run_powershell(command: &str) -> CommandOutput {
    let start = Instant::now();

    let result = tokio::process::Command::new("powershell")
        .args(["-NoProfile", "-Command", command])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await;

    let duration_ms = start.elapsed().as_millis() as u64;

    match result {
        Ok(output) => CommandOutput {
            command: format!("powershell: {}", command),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code().unwrap_or(-1),
            duration_ms,
        },
        Err(e) => CommandOutput {
            command: format!("powershell: {}", command),
            stdout: String::new(),
            stderr: format!("Failed to execute: {}", e),
            exit_code: -1,
            duration_ms,
        },
    }
}

/// Service info from Get-Service
#[derive(Debug, Clone, Default)]
pub struct ServiceInfo {
    pub name: String,
    pub display_name: String,
    pub status: String,
    pub start_type: String,
}

/// List all Windows services
pub async fn list_services() -> (Vec<ServiceInfo>, CommandOutput) {
    let cmd = r#"Get-Service | Select-Object Name, DisplayName, Status, StartType | ConvertTo-Csv -NoTypeInformation"#;
    let output = run_powershell(cmd).await;
    
    let mut services = Vec::new();
    
    if output.exit_code == 0 {
        // Parse CSV output (skip header line)
        for line in output.stdout.lines().skip(1) {
            // CSV format: "Name","DisplayName","Status","StartType"
            let parts: Vec<&str> = line.split(',')
                .map(|s| s.trim_matches('"').trim())
                .collect();
            
            if parts.len() >= 4 {
                services.push(ServiceInfo {
                    name: parts[0].to_string(),
                    display_name: parts[1].to_string(),
                    status: parts[2].to_string(),
                    start_type: parts[3].to_string(),
                });
            }
        }
    }
    
    (services, output)
}

/// List services filtered by name pattern
pub async fn list_services_filtered(filter: &str) -> (Vec<ServiceInfo>, CommandOutput) {
    let cmd = format!(
        r#"Get-Service | Where-Object {{ $_.Name -like '*{}*' -or $_.DisplayName -like '*{}*' }} | Select-Object Name, DisplayName, Status, StartType | ConvertTo-Csv -NoTypeInformation"#,
        filter, filter
    );
    let output = run_powershell(&cmd).await;
    
    let mut services = Vec::new();
    
    if output.exit_code == 0 {
        for line in output.stdout.lines().skip(1) {
            let parts: Vec<&str> = line.split(',')
                .map(|s| s.trim_matches('"').trim())
                .collect();
            
            if parts.len() >= 4 {
                services.push(ServiceInfo {
                    name: parts[0].to_string(),
                    display_name: parts[1].to_string(),
                    status: parts[2].to_string(),
                    start_type: parts[3].to_string(),
                });
            }
        }
    }
    
    (services, output)
}

/// Start a Windows service (requires admin)
pub async fn start_service(name: &str) -> CommandOutput {
    let cmd = format!("Start-Service -Name '{}'", name);
    run_powershell(&cmd).await
}

/// Stop a Windows service (requires admin)
pub async fn stop_service(name: &str) -> CommandOutput {
    let cmd = format!("Stop-Service -Name '{}' -Force", name);
    run_powershell(&cmd).await
}

/// Restart a Windows service (requires admin)
pub async fn restart_service(name: &str) -> CommandOutput {
    let cmd = format!("Restart-Service -Name '{}' -Force", name);
    run_powershell(&cmd).await
}

/// Get detailed info about a specific service
pub async fn get_service_details(name: &str) -> CommandOutput {
    let cmd = format!(
        r#"Get-Service -Name '{}' | Format-List Name, DisplayName, Status, StartType, DependentServices, ServicesDependedOn"#,
        name
    );
    run_powershell(&cmd).await
}
