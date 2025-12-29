//! Network page component
//!
//! Network diagnostics and quick-fix actions.

use dioxus::prelude::*;

use crate::state::NetworkPageState;
use crate::system::network;

/// Network page with quick fixes and adapter info
#[component]
pub fn NetworkPage(is_admin: bool) -> Element {
    // Get persistent state from context
    let mut state: Signal<NetworkPageState> = use_context();

    // Flush DNS handler
    let on_flush_dns = move |_| {
        state.write().is_running = true;
        state.write().running_action = "Flush DNS".to_string();
        state.write().status_message = "Flushing DNS cache...".to_string();
        state.write().status_type = String::new();

        spawn(async move {
            let output = network::flush_dns().await;
            let success = output.succeeded();
            
            let mut s = state.write();
            s.command_outputs = vec![output];
            s.is_running = false;
            s.running_action = String::new();

            if success {
                s.status_message = "DNS cache flushed successfully".to_string();
                s.status_type = "success".to_string();
            } else {
                s.status_message = "Failed to flush DNS cache".to_string();
                s.status_type = "error".to_string();
            }
        });
    };

    // Renew IP handler
    let on_renew_ip = move |_| {
        state.write().is_running = true;
        state.write().running_action = "Renew IP".to_string();
        state.write().status_message = "Releasing and renewing IP...".to_string();
        state.write().status_type = String::new();

        spawn(async move {
            let outputs = network::renew_ip().await;
            let all_success = outputs.iter().all(|o| o.succeeded());
            
            let mut s = state.write();
            s.command_outputs = outputs;
            s.is_running = false;
            s.running_action = String::new();

            if all_success {
                s.status_message = "IP renewed successfully".to_string();
                s.status_type = "success".to_string();
            } else {
                s.status_message = "IP renewal completed with issues".to_string();
                s.status_type = "warning".to_string();
            }
        });
    };

    // Winsock Reset handler
    let on_winsock_reset = move |_| {
        state.write().is_running = true;
        state.write().running_action = "Winsock Reset".to_string();
        state.write().status_message = "Resetting Winsock...".to_string();
        state.write().status_type = String::new();

        spawn(async move {
            let output = network::winsock_reset().await;
            let success = output.succeeded();
            
            let mut s = state.write();
            s.command_outputs = vec![output];
            s.is_running = false;
            s.running_action = String::new();

            if success {
                s.status_message = "Winsock reset successfully. Reboot may be required.".to_string();
                s.status_type = "success".to_string();
            } else {
                s.status_message = "Winsock reset failed. Run as Administrator.".to_string();
                s.status_type = "error".to_string();
            }
        });
    };

    // Restart HTTP Service handler (clears orphaned HTTP sockets)
    let on_restart_http = move |_| {
        state.write().is_running = true;
        state.write().running_action = "Restart HTTP".to_string();
        state.write().status_message = "Restarting HTTP service (http.sys)...".to_string();
        state.write().status_type = String::new();

        spawn(async move {
            let outputs = network::restart_http_service().await;
            let all_success = outputs.iter().all(|o| o.succeeded());
            
            let mut s = state.write();
            s.command_outputs = outputs;
            s.is_running = false;
            s.running_action = String::new();

            if all_success {
                s.status_message = "HTTP service restarted. Orphaned HTTP sockets should be cleared.".to_string();
                s.status_type = "success".to_string();
            } else {
                s.status_message = "HTTP service restart failed. Run as Administrator.".to_string();
                s.status_type = "error".to_string();
            }
        });
    };


    // Ping handler
    let on_ping = move |_| {
        state.write().is_running = true;
        state.write().running_action = "Ping".to_string();
        state.write().status_message = "Pinging 8.8.8.8...".to_string();
        state.write().status_type = String::new();

        spawn(async move {
            let output = network::ping_host("8.8.8.8").await;
            let success = output.succeeded();
            
            let mut s = state.write();
            s.command_outputs = vec![output];
            s.is_running = false;
            s.running_action = String::new();

            if success {
                s.status_message = "Ping completed successfully".to_string();
                s.status_type = "success".to_string();
            } else {
                s.status_message = "Ping failed - network may be down".to_string();
                s.status_type = "error".to_string();
            }
        });
    };

    // HTTP HEAD handler
    let on_http_head = move |_| {
        state.write().is_running = true;
        state.write().running_action = "HTTP HEAD".to_string();
        state.write().status_message = "Fetching headers from microsoft.com...".to_string();
        state.write().status_type = String::new();

        spawn(async move {
            let output = network::http_head("https://www.microsoft.com").await;
            let success = output.succeeded();
            
            let mut s = state.write();
            s.command_outputs = vec![output];
            s.is_running = false;
            s.running_action = String::new();

            if success {
                s.status_message = "HTTP HEAD completed successfully".to_string();
                s.status_type = "success".to_string();
            } else {
                s.status_message = "HTTP request failed".to_string();
                s.status_type = "error".to_string();
            }
        });
    };

    // Clear output handler
    let mut on_clear = move |_| {
        let mut s = state.write();
        s.command_outputs = Vec::new();
        s.status_message = String::new();
        s.status_type = String::new();
    };

    // Read current state
    let current_state = state();
    let outputs_for_panel = current_state.command_outputs.clone();

    rsx! {
        // Page header
        header { class: "page-header",
            h1 { class: "page-title", "🌐 Network Tools" }
            p { class: "page-subtitle", "Quick fixes and diagnostics for network issues" }
        }

        // Split layout: scrollable controls top, fixed output bottom
        div { class: "page-split-layout",
            // Scrollable controls section
            div { class: "page-controls",
                // Quick actions section
                div { class: "section",
                    h3 { class: "section-title", "Quick Actions" }
                    div { class: "quick-actions",
                        // Flush DNS
                        button {
                            class: "quick-action-btn",
                            disabled: current_state.is_running,
                            onclick: on_flush_dns,
                            span { class: "quick-action-icon", "🗑️" }
                            span { class: "quick-action-label",
                                if current_state.running_action == "Flush DNS" { "Running..." } else { "Flush DNS" }
                            }
                        }

                        // Renew IP
                        button {
                            class: "quick-action-btn",
                            disabled: current_state.is_running,
                            onclick: on_renew_ip,
                            span { class: "quick-action-icon", "🔄" }
                            span { class: "quick-action-label",
                                if current_state.running_action == "Renew IP" { "Running..." } else { "Renew IP" }
                            }
                        }

                        // Winsock Reset (requires admin)
                        button {
                            class: "quick-action-btn",
                            disabled: current_state.is_running || !is_admin,
                            onclick: on_winsock_reset,
                            title: if !is_admin { "Requires Administrator" } else { "" },
                            span { class: "quick-action-icon", "⚡" }
                            span { class: "quick-action-label",
                                if current_state.running_action == "Winsock Reset" { "Running..." } else { "Winsock Reset" }
                            }
                        }

                        // Restart HTTP Service (requires admin) - clears orphaned HTTP sockets
                        button {
                            class: if is_admin { "quick-action-btn quick-action-warning" } else { "quick-action-btn" },
                            disabled: current_state.is_running || !is_admin,
                            onclick: on_restart_http,
                            title: if !is_admin { 
                                "Requires Administrator" 
                            } else { 
                                "⚠️ WARNING: Restarts http.sys - will interrupt IIS, WinRM, etc!" 
                            },
                            span { class: "quick-action-icon", "🔁" }
                            span { class: "quick-action-label",
                                if current_state.running_action == "Restart HTTP" { "Running..." } else { "Restart HTTP" }
                            }
                        }

                        // Ping
                        button {
                            class: "quick-action-btn",
                            disabled: current_state.is_running,
                            onclick: on_ping,
                            span { class: "quick-action-icon", "📡" }
                            span { class: "quick-action-label",
                                if current_state.running_action == "Ping" { "Running..." } else { "Ping 8.8.8.8" }
                            }
                        }

                        // HTTP HEAD
                        button {
                            class: "quick-action-btn",
                            disabled: current_state.is_running,
                            onclick: on_http_head,
                            span { class: "quick-action-icon", "🌍" }
                            span { class: "quick-action-label",
                                if current_state.running_action == "HTTP HEAD" { "Running..." } else { "HTTP HEAD" }
                            }
                        }
                    }

                    if !is_admin {
                        div { class: "status-bar warning",
                            "⚠️ Some actions require Administrator privileges. Run as Admin for full functionality."
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

            // Fixed output panel at bottom
            crate::ui::components::OutputPanel {
                outputs: outputs_for_panel,
                on_clear: move |_| on_clear(()),
            }
        }
    }
}
