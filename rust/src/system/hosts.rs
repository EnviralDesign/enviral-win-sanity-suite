//! Hosts file utilities
//!
//! Read and edit the Windows hosts file.

#![allow(dead_code)]

use std::fs;
use std::path::Path;

const HOSTS_PATH: &str = r"C:\Windows\System32\drivers\etc\hosts";

/// A single host entry
#[derive(Debug, Clone)]
pub struct HostEntry {
    pub ip: String,
    pub hostname: String,
    pub comment: Option<String>,
    pub enabled: bool,
    pub line_number: usize,
}

/// Read and parse the hosts file
pub fn read_hosts() -> Result<(Vec<HostEntry>, String), String> {
    let content = fs::read_to_string(HOSTS_PATH)
        .map_err(|e| format!("Failed to read hosts file: {}", e))?;
    
    let mut entries = Vec::new();
    
    for (idx, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        
        // Skip empty lines and comment-only lines
        if trimmed.is_empty() {
            continue;
        }
        
        // Check if line is commented out
        let (enabled, parse_line) = if trimmed.starts_with('#') {
            (false, trimmed.trim_start_matches('#').trim())
        } else {
            (true, trimmed)
        };
        
        // Skip lines that are just comments (not disabled entries)
        if !enabled && !parse_line.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
            continue;
        }
        
        // Parse: IP hostname [# comment]
        let parts: Vec<&str> = parse_line.splitn(2, '#').collect();
        let main_part = parts[0].trim();
        let comment = parts.get(1).map(|s| s.trim().to_string());
        
        let tokens: Vec<&str> = main_part.split_whitespace().collect();
        if tokens.len() >= 2 {
            entries.push(HostEntry {
                ip: tokens[0].to_string(),
                hostname: tokens[1].to_string(),
                comment,
                enabled,
                line_number: idx + 1,
            });
        }
    }
    
    Ok((entries, content))
}

/// Get raw hosts file content
pub fn get_hosts_raw() -> Result<String, String> {
    fs::read_to_string(HOSTS_PATH)
        .map_err(|e| format!("Failed to read hosts file: {}", e))
}

/// Write raw content to hosts file (requires admin)
pub fn write_hosts_raw(content: &str) -> Result<(), String> {
    fs::write(HOSTS_PATH, content)
        .map_err(|e| format!("Failed to write hosts file: {} - Run as Administrator", e))
}

/// Add a new entry to hosts file (requires admin)
pub fn add_host_entry(ip: &str, hostname: &str, comment: Option<&str>) -> Result<(), String> {
    let mut content = get_hosts_raw()?;
    
    // Ensure newline at end
    if !content.ends_with('\n') {
        content.push('\n');
    }
    
    // Add the new entry
    if let Some(c) = comment {
        content.push_str(&format!("{}\t{}\t# {}\n", ip, hostname, c));
    } else {
        content.push_str(&format!("{}\t{}\n", ip, hostname));
    }
    
    write_hosts_raw(&content)
}

/// Remove a host entry by hostname (requires admin)
pub fn remove_host_entry(hostname: &str) -> Result<(), String> {
    let content = get_hosts_raw()?;
    
    let new_content: String = content
        .lines()
        .filter(|line| {
            let trimmed = line.trim().trim_start_matches('#').trim();
            let tokens: Vec<&str> = trimmed.split_whitespace().collect();
            // Keep line if it doesn't match the hostname
            tokens.get(1).map(|h| *h != hostname).unwrap_or(true)
        })
        .collect::<Vec<_>>()
        .join("\n");
    
    write_hosts_raw(&format!("{}\n", new_content))
}

/// Toggle an entry (comment/uncomment) by hostname (requires admin)
pub fn toggle_host_entry(hostname: &str) -> Result<bool, String> {
    let content = get_hosts_raw()?;
    let mut found = false;
    let mut now_enabled = false;
    
    let new_content: String = content
        .lines()
        .map(|line| {
            let trimmed = line.trim();
            let is_commented = trimmed.starts_with('#');
            let parse_line = trimmed.trim_start_matches('#').trim();
            let tokens: Vec<&str> = parse_line.split_whitespace().collect();
            
            if tokens.get(1).map(|h| *h == hostname).unwrap_or(false) {
                found = true;
                if is_commented {
                    // Uncomment
                    now_enabled = true;
                    trimmed.trim_start_matches('#').trim().to_string()
                } else {
                    // Comment out
                    now_enabled = false;
                    format!("# {}", line)
                }
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    
    if !found {
        return Err(format!("Host entry '{}' not found", hostname));
    }
    
    write_hosts_raw(&format!("{}\n", new_content))?;
    Ok(now_enabled)
}

/// Check if hosts file is writable
pub fn is_hosts_writable() -> bool {
    Path::new(HOSTS_PATH).metadata()
        .map(|m| !m.permissions().readonly())
        .unwrap_or(false)
}
