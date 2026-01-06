//! Ports page component
//!
//! Port scanner and process killer functionality.

use dioxus::prelude::*;

use crate::state::{BindingSource, PortBinding, PortScanResult, PortsPageState};
use crate::system::ports;

/// Ports page with port scanning and process killing
#[component]
pub fn PortsPage(is_admin: bool) -> Element {
    // Get persistent state from context
    let mut state: Signal<PortsPageState> = use_context();

    // Scan handler
    let on_scan = move |_| {
        let port = state().port_input;
        state.write().is_scanning = true;
        state.write().status_message = format!("Scanning port {}...", port);
        state.write().status_type = String::new();

        // Run scan in spawn to not block UI (uses enhanced scanner with Docker/WSL detection)
        spawn(async move {
            let result = ports::list_bindings_enhanced(port).await;
            let binding_count = result.bindings.len();
            let has_conflicts = !result.conflict_pids.is_empty();
            let has_orphans = !result.orphan_pids.is_empty();
            let has_docker = !result.docker_bindings.is_empty();
            let has_wsl = !result.wsl_bindings.is_empty();
            let has_shadow = result.shadow_detected;

            let mut s = state.write();
            s.scan_result = result;
            s.is_scanning = false;

            if binding_count == 0 {
                s.status_message = format!("No listeners detected on port {}", port);
                s.status_type = "success".to_string();
            } else if has_shadow {
                s.status_message = format!(
                    "⚠️ Shadow binding detected on port {} - port in use but source unknown!",
                    port
                );
                s.status_type = "warning".to_string();
            } else if has_docker || has_wsl {
                let mut sources = Vec::new();
                if has_docker { sources.push("Docker"); }
                if has_wsl { sources.push("WSL"); }
                s.status_message = format!(
                    "Found {} binding(s) on port {} via {}",
                    binding_count,
                    port,
                    sources.join("/")
                );
                s.status_type = "info".to_string();
            } else if has_orphans {
                s.status_message = format!(
                    "Found {} binding(s) on port {} - {} orphaned socket(s) detected!",
                    binding_count,
                    port,
                    s.scan_result.orphan_pids.len()
                );
                s.status_type = "warning".to_string();
            } else if has_conflicts {
                s.status_message = format!(
                    "Found {} binding(s) on port {} - conflict detected!",
                    binding_count, port
                );
                s.status_type = "warning".to_string();
            } else {
                s.status_message = format!("Found {} binding(s) on port {}", binding_count, port);
                s.status_type = String::new();
            }
        });
    };

    // Kill process handler
    let on_kill = move |pid: u32| {
        let port = state().port_input;
        spawn(async move {
            match ports::kill_process(pid) {
                Ok(_) => {
                    state.write().status_message = format!("Terminated PID {}. Rescanning...", pid);
                    state.write().status_type = "success".to_string();
                    // Rescan after kill (use enhanced scanner)
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    let result = ports::list_bindings_enhanced(port).await;
                    state.write().scan_result = result;
                }
                Err(e) => {
                    state.write().status_message = format!("Failed to kill PID {}: {}", pid, e);
                    state.write().status_type = "error".to_string();
                }
            }
        });
    };

    // Force close orphan handler - diagnoses and optionally restarts http.sys
    let on_force_close = move |binding: PortBinding| {
        let admin = is_admin;
        spawn(async move {
            state.write().status_message = format!(
                "Diagnosing orphaned socket on {}:{}...",
                binding.local_ip, binding.local_port
            );
            state.write().status_type = String::new();

            // First, get diagnostics
            let result = ports::force_close_socket(&binding).await;
            
            match result {
                Ok(msg) => {
                    // Found http.sys involvement - if admin, offer to restart
                    if admin {
                        state.write().status_message = format!(
                            "{}\n\nAttempting to restart HTTP service...",
                            msg
                        );
                        
                        // Actually restart the http service
                        let restart_result = crate::system::network::restart_http_service().await;
                        let restart_ok = restart_result.iter().all(|o| o.succeeded());
                        
                        if restart_ok {
                            state.write().status_message = "HTTP service restarted successfully. Rescanning port...".to_string();
                            state.write().status_type = "success".to_string();
                        } else {
                            state.write().status_message = "HTTP service restart failed. Check output on Network tab.".to_string();
                            state.write().status_type = "warning".to_string();
                        }
                    } else {
                        state.write().status_message = format!(
                            "{}\n\n💡 Run as Administrator to auto-restart HTTP service, or use Network tab → Restart HTTP.",
                            msg
                        );
                        state.write().status_type = "warning".to_string();
                    }
                    
                    // Rescan after a brief delay (use enhanced scanner)
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    let scan = ports::list_bindings_enhanced(state().port_input).await;
                    state.write().scan_result = scan;
                }
                Err(e) => {
                    if admin {
                        // Even without http.sys detection, try restarting as it might help
                        state.write().status_message = format!(
                            "{}\n\nTrying HTTP service restart anyway...",
                            e
                        );
                        
                        let restart_result = crate::system::network::restart_http_service().await;
                        let restart_ok = restart_result.iter().all(|o| o.succeeded());
                        
                        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                        let scan = ports::list_bindings_enhanced(state().port_input).await;
                        let no_orphans = scan.bindings.iter().all(|b| !b.is_orphan);
                        state.write().scan_result = scan;
                        
                        if restart_ok && no_orphans {
                            state.write().status_message = "HTTP service restart cleared the orphaned socket!".to_string();
                            state.write().status_type = "success".to_string();
                        } else {
                            state.write().status_message = e;
                            state.write().status_type = "error".to_string();
                        }
                    } else {
                        state.write().status_message = format!(
                            "{}\n\n💡 Try running as Administrator for more options.",
                            e
                        );
                        state.write().status_type = "error".to_string();
                    }
                }
            }
        });
    };

    // Suggest free port handler
    let on_suggest = move |_| {
        if let Some(free_port) = ports::suggest_free_port(3000, 3100) {
            state.write().port_input = free_port;
            state.write().status_message = format!("Suggested free port: {}", free_port);
            state.write().status_type = "success".to_string();
        } else {
            state.write().status_message = "No free port found in range 3000-3100".to_string();
            state.write().status_type = "warning".to_string();
        }
    };

    // Copy report handler
    let on_copy = move |_| {
        let bindings = &state().scan_result.bindings;
        if bindings.is_empty() {
            state.write().status_message = "No data to copy".to_string();
            state.write().status_type = "warning".to_string();
            return;
        }

        let mut report = String::from("PID\tProcess\tLocal Address\tState\tScope\tStatus\n");
        for b in bindings {
            report.push_str(&format!(
                "{}\t{}\t{}\t{}\t{}\t{}\n",
                b.pid,
                b.process_name,
                b.address(),
                b.state,
                b.scope_description(),
                b.process_status()
            ));
        }

        if let Ok(mut clipboard) = arboard::Clipboard::new() {
            if clipboard.set_text(&report).is_ok() {
                state.write().status_message = "Report copied to clipboard".to_string();
                state.write().status_type = "success".to_string();
            } else {
                state.write().status_message = "Failed to copy to clipboard".to_string();
                state.write().status_type = "error".to_string();
            }
        }
    };

    // Clear handler
    let on_clear = move |_| {
        let mut s = state.write();
        s.scan_result = PortScanResult::default();
        s.status_message = String::new();
        s.status_type = String::new();
    };

    // Read current state
    let current_state = state();

    rsx! {
        // Page header
        header { class: "page-header",
            h1 { class: "page-title", "🔌 Port Scanner" }
            p { class: "page-subtitle", "Scan TCP ports and manage processes" }
        }

        // Page content
        div { class: "page-content",
            // Action bar
            div { class: "action-bar",
                div { class: "action-bar-group",
                    div { class: "input-group",
                        label { "Port:" }
                        input {
                            r#type: "number",
                            class: "input input-number",
                            value: "{current_state.port_input}",
                            min: 1,
                            max: 65535,
                            oninput: move |e| {
                                if let Ok(v) = e.value().parse::<u16>() {
                                    state.write().port_input = v;
                                }
                            },
                        }
                    }
                    button {
                        class: "btn btn-primary",
                        onclick: on_scan,
                        disabled: current_state.is_scanning,
                        if current_state.is_scanning {
                            span { class: "spinner" }
                        } else {
                            "Scan"
                        }
                    }
                }

                div { class: "action-bar-divider" }

                div { class: "action-bar-group",
                    button {
                        class: "btn btn-secondary",
                        onclick: on_suggest,
                        "Suggest Free Port"
                    }
                    button {
                        class: "btn btn-secondary",
                        onclick: on_copy,
                        disabled: current_state.scan_result.bindings.is_empty(),
                        "Copy Report"
                    }
                    button {
                        class: "btn btn-ghost",
                        onclick: on_clear,
                        "Clear"
                    }
                }
            }

            // Orphan warning
            if !current_state.scan_result.orphan_pids.is_empty() {
                div { class: "status-bar warning",
                    "⚠️ Orphaned sockets detected! These are sockets where the process has exited but the connection remains. ",
                    "This can happen with http.sys or crashed applications. Try 'Force Close' or restart your machine."
                }
            }

            // Docker/WSL info banner
            if !current_state.scan_result.docker_bindings.is_empty() {
                div { class: "status-bar info",
                    "🐳 Docker container detected. To free this port, stop the container: ",
                    code { "docker stop <container_name>" }
                }
            }

            // WSL info banner
            if !current_state.scan_result.wsl_bindings.is_empty() && current_state.scan_result.docker_bindings.is_empty() {
                div { class: "status-bar info",
                    "🐧 WSL process detected. To free this port, run inside WSL: ",
                    code { "kill <pid>" },
                    " or terminate the WSL distro."
                }
            }

            // Shadow binding warning
            if current_state.scan_result.shadow_detected {
                div { class: "status-bar warning",
                    "👻 Shadow binding detected! The port is in use at the kernel level but no visible process found. ",
                    "This can happen with Docker/WSL not running, or Hyper-V networking. Try restarting Docker Desktop or WSL."
                }
            }

            // Results table
            if current_state.scan_result.bindings.is_empty() {
                div { class: "empty-state",
                    div { class: "empty-state-icon", "📋" }
                    p { class: "empty-state-text", "Enter a port number and click Scan to find listeners" }
                }
            } else {
                table { class: "data-table",
                    thead {
                        tr {
                            th { "Source" }
                            th { "PID" }
                            th { "Process" }
                            th { "Local Address" }
                            th { "State" }
                            th { "Scope" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        for binding in current_state.scan_result.bindings.iter().cloned() {
                            PortRow {
                                binding: binding.clone(),
                                is_conflict: current_state.scan_result.conflict_pids.contains(&binding.pid),
                                on_kill: move |pid| on_kill(pid),
                                on_force_close: move |b| on_force_close(b),
                            }
                        }
                    }
                }
            }

            // Status bar
            if !current_state.status_message.is_empty() {
                div {
                    class: format!("status-bar {}", current_state.status_type),
                    "{current_state.status_message}"
                }
            }
        }
    }
}

/// Individual port binding row
#[component]
fn PortRow(
    binding: PortBinding,
    is_conflict: bool,
    on_kill: EventHandler<u32>,
    on_force_close: EventHandler<PortBinding>,
) -> Element {
    // Determine row styling based on source and status
    let row_class = match binding.source {
        BindingSource::Docker => "docker",
        BindingSource::Wsl => "wsl",
        BindingSource::UnknownShadow => "shadow",
        BindingSource::Windows => {
            if binding.is_orphan {
                "orphan"
            } else if is_conflict {
                "conflict"
            } else {
                ""
            }
        }
    };

    // Source badge styling
    let source_class = match binding.source {
        BindingSource::Docker => "badge badge-docker",
        BindingSource::Wsl => "badge badge-wsl",
        BindingSource::UnknownShadow => "badge badge-warning",
        BindingSource::Windows => "badge badge-windows",
    };

    let binding_for_close = binding.clone();
    let pid_display = if binding.pid == 0 {
        "-".to_string()
    } else {
        binding.pid.to_string()
    };

    rsx! {
        tr { class: row_class,
            td {
                span { class: source_class, "{binding.source.description()}" }
            }
            td { class: "mono", "{pid_display}" }
            td { "{binding.process_name}" }
            td { class: "mono", "{binding.address()}" }
            td { class: "muted", "{binding.state}" }
            td { class: row_class, "{binding.scope_description()}" }
            td {
                // Action buttons based on source
                match binding.source {
                    BindingSource::Docker => rsx! {
                        span { class: "muted hint",
                            title: "Stop the Docker container to free this port",
                            "🐳 docker stop"
                        }
                    },
                    BindingSource::Wsl => rsx! {
                        span { class: "muted hint",
                            title: format!("Kill process {} in WSL distro: {}", binding.pid, binding.source_detail),
                            "🐧 wsl kill {binding.pid}"
                        }
                    },
                    BindingSource::UnknownShadow => rsx! {
                        span { class: "muted hint",
                            title: "Shadow binding - try restarting Docker/WSL",
                            "👻 Unknown"
                        }
                    },
                    BindingSource::Windows => rsx! {
                        if binding.is_orphan {
                            button {
                                class: "btn btn-warning btn-sm",
                                onclick: move |_| on_force_close.call(binding_for_close.clone()),
                                "Force Close"
                            }
                        } else if binding.is_system {
                            span { class: "muted", "System" }
                        } else {
                            button {
                                class: "btn btn-danger btn-sm",
                                onclick: move |_| on_kill.call(binding.pid),
                                "Kill"
                            }
                        }
                    }
                }
            }
        }
    }
}
