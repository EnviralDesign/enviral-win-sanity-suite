//! Command execution utilities
//!
//! Provides helpers for running system commands without visible console windows.

use crate::state::CommandOutput;
use std::process::Stdio;
use std::time::Instant;
use tokio::process::Command;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

/// Windows flag to create process without a visible window
#[cfg(target_os = "windows")]
pub const CREATE_NO_WINDOW: u32 = 0x08000000;

/// Create an async Command configured to run without a visible window on Windows
pub fn hidden_command(program: &str) -> Command {
    let mut cmd = Command::new(program);
    
    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);
    
    cmd
}

/// Create a sync Command configured to run without a visible window on Windows
pub fn hidden_command_sync(program: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new(program);
    
    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);
    
    cmd
}

/// Run a command and capture output, without showing a console window
pub async fn run_command(program: &str, args: &[&str]) -> CommandOutput {
    let command_str = format!("{} {}", program, args.join(" "));
    let start = Instant::now();

    let result = hidden_command(program)
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

/// Run a PowerShell command and capture output, without showing a console window
pub async fn run_powershell(command: &str) -> CommandOutput {
    let start = Instant::now();

    let result = hidden_command("powershell")
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

