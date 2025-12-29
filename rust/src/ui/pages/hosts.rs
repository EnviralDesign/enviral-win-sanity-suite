//! Hosts page component
//!
//! View and edit the Windows hosts file.

use dioxus::prelude::*;

use crate::state::HostsPageState;
use crate::system::hosts;

/// Hosts page
#[component]
pub fn HostsPage(is_admin: bool) -> Element {
    let mut state: Signal<HostsPageState> = use_context();

    // Load hosts file
    let on_load = move |_| {
        state.write().is_running = true;
        state.write().status_message = "Loading hosts file...".to_string();
        state.write().status_type = String::new();

        spawn(async move {
            match hosts::read_hosts() {
                Ok((entries, raw)) => {
                    let mut s = state.write();
                    s.entries = entries;
                    s.raw_content = raw;
                    s.is_running = false;
                    s.status_message = format!("Loaded {} entries", s.entries.len());
                    s.status_type = "success".to_string();
                }
                Err(e) => {
                    let mut s = state.write();
                    s.is_running = false;
                    s.status_message = e;
                    s.status_type = "error".to_string();
                }
            }
        });
    };

    // Add entry handler
    let on_add = move |_| {
        let ip = state().new_ip.clone();
        let hostname = state().new_hostname.clone();

        if ip.is_empty() || hostname.is_empty() {
            state.write().status_message = "Enter both IP and hostname".to_string();
            state.write().status_type = "warning".to_string();
            return;
        }

        state.write().is_running = true;
        state.write().status_message = format!("Adding {} {}...", ip, hostname);

        spawn(async move {
            match hosts::add_host_entry(&ip, &hostname, None) {
                Ok(_) => {
                    // Reload
                    if let Ok((entries, raw)) = hosts::read_hosts() {
                        let mut s = state.write();
                        s.entries = entries;
                        s.raw_content = raw;
                        s.new_ip = String::new();
                        s.new_hostname = String::new();
                        s.status_message = format!("Added {} -> {}", ip, hostname);
                        s.status_type = "success".to_string();
                    }
                }
                Err(e) => {
                    state.write().status_message = e;
                    state.write().status_type = "error".to_string();
                }
            }
            state.write().is_running = false;
        });
    };

    // Toggle entry handler
    let on_toggle = move |_| {
        let hostname = state().selected_hostname.clone();
        if hostname.is_empty() {
            state.write().status_message = "Select an entry first".to_string();
            state.write().status_type = "warning".to_string();
            return;
        }

        state.write().is_running = true;
        state.write().status_message = format!("Toggling {}...", hostname);

        spawn(async move {
            match hosts::toggle_host_entry(&hostname) {
                Ok(enabled) => {
                    // Reload
                    if let Ok((entries, raw)) = hosts::read_hosts() {
                        let mut s = state.write();
                        s.entries = entries;
                        s.raw_content = raw;
                        let status = if enabled { "enabled" } else { "disabled" };
                        s.status_message = format!("{} is now {}", hostname, status);
                        s.status_type = "success".to_string();
                    }
                }
                Err(e) => {
                    state.write().status_message = e;
                    state.write().status_type = "error".to_string();
                }
            }
            state.write().is_running = false;
        });
    };

    // Remove entry handler
    let on_remove = move |_| {
        let hostname = state().selected_hostname.clone();
        if hostname.is_empty() {
            state.write().status_message = "Select an entry first".to_string();
            state.write().status_type = "warning".to_string();
            return;
        }

        state.write().is_running = true;
        state.write().status_message = format!("Removing {}...", hostname);

        spawn(async move {
            match hosts::remove_host_entry(&hostname) {
                Ok(_) => {
                    // Reload
                    if let Ok((entries, raw)) = hosts::read_hosts() {
                        let mut s = state.write();
                        s.entries = entries;
                        s.raw_content = raw;
                        s.selected_hostname = String::new();
                        s.status_message = format!("Removed {}", hostname);
                        s.status_type = "success".to_string();
                    }
                }
                Err(e) => {
                    state.write().status_message = e;
                    state.write().status_type = "error".to_string();
                }
            }
            state.write().is_running = false;
        });
    };

    let current_state = state();

    rsx! {
        header { class: "page-header",
            h1 { class: "page-title", "📝 Hosts" }
            p { class: "page-subtitle", "Edit the Windows hosts file" }
        }

        div { class: "page-split-layout",
            div { class: "page-controls",
                // Load button
                div { class: "section",
                    div { class: "action-bar",
                        button {
                            class: "btn btn-primary",
                            disabled: current_state.is_running,
                            onclick: on_load,
                            "🔄 Load Hosts File"
                        }

                        if !is_admin {
                            span { class: "muted", "⚠️ Editing requires Administrator" }
                        }
                    }
                }

                // Add entry section
                div { class: "section",
                    h3 { class: "section-title", "Add Entry" }
                    div { class: "action-bar",
                        div { class: "action-bar-group",
                            label { "IP:" }
                            input {
                                r#type: "text",
                                class: "input",
                                placeholder: "127.0.0.1",
                                value: "{current_state.new_ip}",
                                oninput: move |e| {
                                    state.write().new_ip = e.value();
                                },
                            }
                        }

                        div { class: "action-bar-group",
                            label { "Hostname:" }
                            input {
                                r#type: "text",
                                class: "input",
                                placeholder: "myapp.local",
                                value: "{current_state.new_hostname}",
                                oninput: move |e| {
                                    state.write().new_hostname = e.value();
                                },
                            }
                        }

                        button {
                            class: "btn btn-primary",
                            disabled: current_state.is_running || !is_admin || current_state.new_ip.is_empty() || current_state.new_hostname.is_empty(),
                            onclick: on_add,
                            "➕ Add"
                        }
                    }
                }

                // Entries list
                if !current_state.entries.is_empty() {
                    div { class: "section",
                        h3 { class: "section-title", "Entries ({current_state.entries.len()})" }
                        div { class: "service-list",
                            table { class: "data-table",
                                thead {
                                    tr {
                                        th { "" }
                                        th { "Status" }
                                        th { "IP" }
                                        th { "Hostname" }
                                        th { "Comment" }
                                    }
                                }
                                tbody {
                                    for entry in current_state.entries.iter() {
                                        {
                                            let is_selected = current_state.selected_hostname == entry.hostname;
                                            let hostname = entry.hostname.clone();
                                            let status_class = if entry.enabled { "success" } else { "muted" };
                                            rsx! {
                                                tr {
                                                    class: if is_selected { "selected" } else { "" },
                                                    onclick: move |_| {
                                                        state.write().selected_hostname = hostname.clone();
                                                    },
                                                    td {
                                                        input {
                                                            r#type: "radio",
                                                            checked: is_selected,
                                                            onchange: move |_| {},
                                                        }
                                                    }
                                                    td { class: status_class,
                                                        if entry.enabled { "✓" } else { "✗" }
                                                    }
                                                    td { class: "mono", "{entry.ip}" }
                                                    td { class: "mono", "{entry.hostname}" }
                                                    td { class: "muted", 
                                                        {entry.comment.as_deref().unwrap_or("")}
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Entry actions
                        if !current_state.selected_hostname.is_empty() {
                            div { class: "action-bar", style: "margin-top: 1rem;",
                                span { class: "mono", "Selected: {current_state.selected_hostname}" }
                                
                                button {
                                    class: "btn btn-secondary",
                                    disabled: current_state.is_running || !is_admin,
                                    onclick: on_toggle,
                                    "🔀 Toggle"
                                }

                                button {
                                    class: "btn btn-danger",
                                    disabled: current_state.is_running || !is_admin,
                                    onclick: on_remove,
                                    "🗑️ Remove"
                                }
                            }
                        }
                    }
                } else if current_state.raw_content.is_empty() {
                    div { class: "section",
                        div { class: "empty-state",
                            div { class: "empty-state-icon", "📝" }
                            p { class: "empty-state-text", "Click 'Load Hosts File' to view entries" }
                        }
                    }
                }

                if !current_state.status_message.is_empty() {
                    div {
                        class: format!("status-bar {}", current_state.status_type),
                        "{current_state.status_message}"
                    }
                }
            }
        }
    }
}
