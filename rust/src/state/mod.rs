//! Application state types
//!
//! Data structures for storing scan results, network info, etc.

use serde::{Deserialize, Serialize};

/// Source of a port binding - where it was detected from
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum BindingSource {
    /// Standard Windows TCP stack (netstat2)
    #[default]
    Windows,
    /// Docker container via port forwarding
    Docker,
    /// WSL2 distro (non-Docker)
    Wsl,
    /// Detected via socket probe but source unknown
    UnknownShadow,
}

impl BindingSource {
    /// Get a human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            BindingSource::Windows => "Windows",
            BindingSource::Docker => "Docker",
            BindingSource::Wsl => "WSL",
            BindingSource::UnknownShadow => "Shadow",
        }
    }

    /// Check if this binding can be killed directly
    pub fn can_kill(&self) -> bool {
        matches!(self, BindingSource::Windows)
    }
}

/// Represents a TCP port binding with process information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PortBinding {
    /// Process ID (0 for Docker/WSL bindings without direct PID)
    pub pid: u32,
    /// Process name (or container name for Docker)
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
    /// Source of this binding detection
    #[serde(default)]
    pub source: BindingSource,
    /// Additional context (e.g., Docker container ID, WSL distro name)
    #[serde(default)]
    pub source_detail: String,
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
        match self.source {
            BindingSource::Docker => "Docker",
            BindingSource::Wsl => "WSL",
            BindingSource::UnknownShadow => "Shadow",
            BindingSource::Windows => {
                if self.is_system {
                    "System/Kernel"
                } else if self.is_orphan {
                    "Orphaned"
                } else {
                    "Active"
                }
            }
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
    /// Docker containers using this port
    pub docker_bindings: Vec<DockerPortBinding>,
    /// WSL processes using this port
    pub wsl_bindings: Vec<WslPortBinding>,
    /// True if socket probe detected the port is in use but no visible binding found
    pub shadow_detected: bool,
}

/// Docker container port binding
#[derive(Debug, Clone, Default)]
pub struct DockerPortBinding {
    /// Container ID (short)
    pub container_id: String,
    /// Container name
    pub container_name: String,
    /// Image name
    pub image: String,
    /// Host port
    pub host_port: u16,
    /// Container port
    pub container_port: u16,
    /// Protocol (tcp/udp)
    pub protocol: String,
}

/// WSL process port binding
#[derive(Debug, Clone, Default)]
pub struct WslPortBinding {
    /// WSL distro name
    pub distro: String,
    /// Process name inside WSL
    pub process_name: String,
    /// PID inside WSL
    pub pid: u32,
    /// Port number
    pub port: u16,
    /// Local address
    pub local_addr: String,
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
    Processes,
    Hosts,
    Disk,
}

/// Sort column options for ports table
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PortsSortColumn {
    #[default]
    Source,
    Pid,
    Process,
    Address,
    State,
    Scope,
}

/// Sort column options for services table
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ServicesSortColumn {
    #[default]
    Name,
    DisplayName,
    Status,
    StartType,
}

/// Sort column options for processes table
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProcessesSortColumn {
    #[default]
    Memory,
    Pid,
    Name,
    Handles,
    Cpu,
    Status,
}

/// Sort column options for hosts table
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HostsSortColumn {
    #[default]
    Status,
    Ip,
    Hostname,
}

/// Sort column options for disk temp folders table
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiskSortColumn {
    #[default]
    Folder,
    Size,
    Files,
}

/// Global application state that persists across tab switches
#[derive(Debug, Clone, Default)]
pub struct PortsPageState {
    pub port_input: u16,
    pub scan_result: PortScanResult,
    pub status_message: String,
    pub status_type: String,
    pub is_scanning: bool,
    /// Current sort column
    pub sort_column: Option<PortsSortColumn>,
    /// Sort direction (true = ascending)
    pub sort_ascending: bool,
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
    /// Current sort column
    pub sort_column: Option<ServicesSortColumn>,
    /// Sort direction (true = ascending)
    pub sort_ascending: bool,
}

/// Processes page state
#[derive(Debug, Clone, Default)]
pub struct ProcessesPageState {
    pub status_message: String,
    pub status_type: String,
    pub is_running: bool,
    pub running_action: String,
    /// Process list
    pub processes: Vec<crate::system::processes::ProcessInfo>,
    /// System overview
    pub system_info: crate::system::processes::SystemOverview,
    /// Sort mode: "memory" or "cpu"
    pub sort_by: String,
    /// Selected process PID
    pub selected_pid: Option<u32>,
    /// Current sort column for table header clicks
    pub sort_column: Option<ProcessesSortColumn>,
    /// Sort direction (true = ascending)
    pub sort_ascending: bool,
}

impl ProcessesPageState {
    pub fn new() -> Self {
        Self {
            sort_by: "memory".to_string(),
            ..Default::default()
        }
    }
}

/// Hosts page state
#[derive(Debug, Clone, Default)]
pub struct HostsPageState {
    pub status_message: String,
    pub status_type: String,
    pub is_running: bool,
    /// Raw hosts file content
    pub raw_content: String,
    /// Parsed entries
    pub entries: Vec<crate::system::hosts::HostEntry>,
    /// New entry IP
    pub new_ip: String,
    /// New entry hostname
    pub new_hostname: String,
    /// Selected hostname
    pub selected_hostname: String,
    /// Current sort column
    pub sort_column: Option<HostsSortColumn>,
    /// Sort direction (true = ascending)
    pub sort_ascending: bool,
}

/// Disk page state
#[derive(Debug, Clone, Default)]
pub struct DiskPageState {
    pub status_message: String,
    pub status_type: String,
    pub is_running: bool,
    pub running_action: String,
    /// Disk drives
    pub disks: Vec<crate::system::disk::DiskInfo>,
    /// Temp folder sizes
    pub temp_folders: Vec<crate::system::disk::FolderSize>,
    /// Current sort column
    pub sort_column: Option<DiskSortColumn>,
    /// Sort direction (true = ascending)
    pub sort_ascending: bool,
}

