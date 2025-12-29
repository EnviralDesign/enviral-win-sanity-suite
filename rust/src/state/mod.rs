//! Application state types
//!
//! Data structures for storing scan results, network info, etc.

use serde::{Deserialize, Serialize};

/// Represents a TCP port binding with process information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PortBinding {
    /// Process ID
    pub pid: u32,
    /// Process name
    pub process_name: String,
    /// Local IP address
    pub local_ip: String,
    /// Local port number
    pub local_port: u16,
    /// Connection state (LISTEN, ESTABLISHED, etc.)
    pub state: String,
    /// Whether this is a loopback address (127.x.x.x or ::1)
    pub is_loopback: bool,
    /// Whether this binds to all interfaces (0.0.0.0 or ::)
    pub is_all_interfaces: bool,
    /// Whether the process is orphaned (socket exists but process doesn't)
    pub is_orphan: bool,
    /// Whether this is a system/kernel socket (PID 0 or 4)
    pub is_system: bool,
}

impl PortBinding {
    /// Get a human-readable scope description
    pub fn scope_description(&self) -> &'static str {
        if self.is_loopback {
            "Loopback"
        } else if self.is_all_interfaces {
            "All Interfaces"
        } else {
            "Specific IP"
        }
    }

    /// Format the full address string
    pub fn address(&self) -> String {
        format!("{}:{}", self.local_ip, self.local_port)
    }

    /// Get process status description
    pub fn process_status(&self) -> &'static str {
        if self.is_system {
            "System/Kernel"
        } else if self.is_orphan {
            "Orphaned"
        } else {
            "Active"
        }
    }
}

/// Result of a port scan operation
#[derive(Debug, Clone, Default)]
pub struct PortScanResult {
    /// All bindings found for the scanned port
    pub bindings: Vec<PortBinding>,
    /// PIDs that have conflicts (both loopback and all-interfaces bindings)
    pub conflict_pids: Vec<u32>,
    /// PIDs that are orphaned (socket exists but process doesn't)
    pub orphan_pids: Vec<u32>,
}

/// Represents a network adapter with its addresses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkAdapter {
    /// Adapter name
    pub name: String,
    /// IPv4 and IPv6 addresses
    pub addresses: Vec<String>,
    /// Whether the adapter is up
    pub is_up: bool,
}

/// Result of running a system command
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CommandOutput {
    /// The command that was run
    pub command: String,
    /// Standard output
    pub stdout: String,
    /// Standard error
    pub stderr: String,
    /// Exit code (0 = success)
    pub exit_code: i32,
    /// Duration in milliseconds
    pub duration_ms: u64,
}

impl CommandOutput {
    /// Check if the command succeeded
    pub fn succeeded(&self) -> bool {
        self.exit_code == 0
    }

    /// Get combined output for display
    pub fn display_output(&self) -> String {
        let mut output = format!("$ {}\n", self.command);
        if !self.stdout.is_empty() {
            output.push_str(&self.stdout);
            if !self.stdout.ends_with('\n') {
                output.push('\n');
            }
        }
        if !self.stderr.is_empty() {
            output.push_str(&self.stderr);
            if !self.stderr.ends_with('\n') {
                output.push('\n');
            }
        }
        output.push_str(&format!("Exit code: {} ({}ms)\n", self.exit_code, self.duration_ms));
        output
    }
}

/// Current page/tab selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CurrentPage {
    #[default]
    Ports,
    Network,
    Docker,
    Services,
}

/// Global application state that persists across tab switches
#[derive(Debug, Clone, Default)]
pub struct PortsPageState {
    pub port_input: u16,
    pub scan_result: PortScanResult,
    pub status_message: String,
    pub status_type: String,
    pub is_scanning: bool,
}

impl PortsPageState {
    pub fn new() -> Self {
        Self {
            port_input: 3010,
            ..Default::default()
        }
    }
}

/// Network page state
#[derive(Debug, Clone, Default)]
pub struct NetworkPageState {
    pub command_outputs: Vec<CommandOutput>,
    pub status_message: String,
    pub status_type: String,
    pub is_running: bool,
    pub running_action: String,
}

/// Docker page state
#[derive(Debug, Clone, Default)]
pub struct DockerPageState {
    pub command_outputs: Vec<CommandOutput>,
    pub status_message: String,
    pub status_type: String,
    pub is_running: bool,
    pub running_action: String,
    /// Currently selected container for logs/actions
    pub selected_container: String,
    /// Image name input for pull
    pub image_input: String,
    /// Compose file path input
    pub compose_path: String,
    /// Cached list of container names
    pub container_names: Vec<String>,
    /// Number of log lines to tail
    pub log_tail_lines: u32,
}

impl DockerPageState {
    pub fn new() -> Self {
        Self {
            log_tail_lines: 100,
            ..Default::default()
        }
    }
}

/// Services page state
#[derive(Debug, Clone, Default)]
pub struct ServicesPageState {
    pub command_outputs: Vec<CommandOutput>,
    pub status_message: String,
    pub status_type: String,
    pub is_running: bool,
    pub running_action: String,
    /// Filter for service list
    pub filter: String,
    /// Selected service name
    pub selected_service: String,
    /// Cached service list
    pub services: Vec<crate::system::services::ServiceInfo>,
}
