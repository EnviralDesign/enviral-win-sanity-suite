//! Services page component
//!
//! Windows services management - list, start, stop, restart.

use dioxus::prelude::*;

use crate::state::{ServicesPageState, ServicesSortColumn};
use crate::system::services;
use crate::ui::components::{OutputPanel, SortableHeader, StaticHeader, SortDirection};

fn update_service_sort(mut state: Signal<ServicesPageState>, column: ServicesSortColumn) {
    let mut s = state.write();
    if s.sort_column == Some(column) {
        s.sort_ascending = !s.sort_ascending;
    } else {
        s.sort_column = Some(column);
        s.sort_ascending = true;
    }
}

/// Services page with Windows service management
#[component]
pub fn ServicesPage(is_admin: bool) -> Element {
    // Get persistent state from context
    let mut state: Signal<ServicesPageState> = use_context();

    // List services handler
    let on_list_services = move |_| {
        let filter = state().filter.clone();
        state.write().is_running = true;
        state.write().running_action = "List".to_string();
        state.write().status_message = "Loading services...".to_string();
        state.write().status_type = String::new();

        spawn(async move {
            let (svc_list, output) = if filter.is_empty() {
                services::list_services().await
            } else {
                services::list_services_filtered(&filter).await
            };
            
            let count = svc_list.len();
            let success = output.exit_code == 0;
            
            let mut s = state.write();
            s.services = svc_list;
            s.command_outputs = vec![output];
            s.is_running = false;
            s.running_action = String::new();

            if success {
                s.status_message = format!("Found {} services", count);
                s.status_type = "success".to_string();
            } else {
                s.status_message = "Failed to list services".to_string();
                s.status_type = "error".to_string();
            }
        });
    };

    // Start service handler
    let on_start_service = move |_| {
        let service_name = state().selected_service.clone();
        if service_name.is_empty() {
            state.write().status_message = "Select a service first".to_string();
            state.write().status_type = "warning".to_string();
            return;
        }

        state.write().is_running = true;
        state.write().running_action = "Start".to_string();
        state.write().status_message = format!("Starting {}...", service_name);
        state.write().status_type = String::new();

        spawn(async move {
            let output = services::start_service(&service_name).await;
            let success = output.succeeded();
            
            let mut s = state.write();
            s.command_outputs = vec![output];
            s.is_running = false;
            s.running_action = String::new();

            if success {
                s.status_message = format!("Service {} started", service_name);
                s.status_type = "success".to_string();
            } else {
                s.status_message = format!("Failed to start {} - Run as Administrator", service_name);
                s.status_type = "error".to_string();
            }
        });
    };

    // Stop service handler
    let on_stop_service = move |_| {
        let service_name = state().selected_service.clone();
        if service_name.is_empty() {
            state.write().status_message = "Select a service first".to_string();
            state.write().status_type = "warning".to_string();
            return;
        }

        state.write().is_running = true;
        state.write().running_action = "Stop".to_string();
        state.write().status_message = format!("Stopping {}...", service_name);
        state.write().status_type = String::new();

        spawn(async move {
            let output = services::stop_service(&service_name).await;
            let success = output.succeeded();
            
            let mut s = state.write();
            s.command_outputs = vec![output];
            s.is_running = false;
            s.running_action = String::new();

            if success {
                s.status_message = format!("Service {} stopped", service_name);
                s.status_type = "success".to_string();
            } else {
                s.status_message = format!("Failed to stop {} - Run as Administrator", service_name);
                s.status_type = "error".to_string();
            }
        });
    };

    // Restart service handler
    let on_restart_service = move |_| {
        let service_name = state().selected_service.clone();
        if service_name.is_empty() {
            state.write().status_message = "Select a service first".to_string();
            state.write().status_type = "warning".to_string();
            return;
        }

        state.write().is_running = true;
        state.write().running_action = "Restart".to_string();
        state.write().status_message = format!("Restarting {}...", service_name);
        state.write().status_type = String::new();

        spawn(async move {
            let output = services::restart_service(&service_name).await;
            let success = output.succeeded();
            
            let mut s = state.write();
            s.command_outputs = vec![output];
            s.is_running = false;
            s.running_action = String::new();

            if success {
                s.status_message = format!("Service {} restarted", service_name);
                s.status_type = "success".to_string();
            } else {
                s.status_message = format!("Failed to restart {} - Run as Administrator", service_name);
                s.status_type = "error".to_string();
            }
        });
    };

    // Get details handler
    let on_get_details = move |_| {
        let service_name = state().selected_service.clone();
        if service_name.is_empty() {
            state.write().status_message = "Select a service first".to_string();
            state.write().status_type = "warning".to_string();
            return;
        }

        state.write().is_running = true;
        state.write().running_action = "Details".to_string();
        state.write().status_message = format!("Getting details for {}...", service_name);
        state.write().status_type = String::new();

        spawn(async move {
            let output = services::get_service_details(&service_name).await;
            let success = output.succeeded();
            
            let mut s = state.write();
            s.command_outputs = vec![output];
            s.is_running = false;
            s.running_action = String::new();

            if success {
                s.status_message = format!("Details for {}", service_name);
                s.status_type = "success".to_string();
            } else {
                s.status_message = format!("Failed to get details for {}", service_name);
                s.status_type = "error".to_string();
            }
        });
    };

    // Clear handler
    let mut on_clear = move |_| {
        let mut s = state.write();
        s.command_outputs = Vec::new();
        s.status_message = String::new();
        s.status_type = String::new();
    };



    // Read current state
    let current_state = state();
    let outputs_for_panel = current_state.command_outputs.clone();

    // Sort services based on current sort state
    let mut sorted_services = current_state.services.clone();
    if let Some(sort_col) = current_state.sort_column {
        let asc = current_state.sort_ascending;
        sorted_services.sort_by(|a, b| {
            let cmp = match sort_col {
                ServicesSortColumn::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                ServicesSortColumn::DisplayName => a.display_name.to_lowercase().cmp(&b.display_name.to_lowercase()),
                ServicesSortColumn::Status => a.status.cmp(&b.status),
                ServicesSortColumn::StartType => a.start_type.cmp(&b.start_type),
            };
            if asc { cmp } else { cmp.reverse() }
        });
    }

    // Determine sort direction for display
    let sort_dir = if current_state.sort_ascending {
        SortDirection::Ascending
    } else {
        SortDirection::Descending
    };

    rsx! {
        // Page header
        header { class: "page-header",
            h1 { class: "page-title", "⚙️ Services" }
            p { class: "page-subtitle", "Windows service management" }
        }

        // Split layout
        div { class: "page-split-layout",
            // Scrollable controls
            div { class: "page-controls",
                // Search/Filter section
                div { class: "section",
                    h3 { class: "section-title", "Search Services" }
                    div { class: "action-bar",
                        div { class: "action-bar-group",
                            label { "Filter:" }
                            input {
                                r#type: "text",
                                class: "input",
                                placeholder: "e.g. docker, sql, iis...",
                                value: "{current_state.filter}",
                                oninput: move |e| {
                                    state.write().filter = e.value();
                                },
                            }
                        }

                        button {
                            class: "btn btn-primary",
                            disabled: current_state.is_running,
                            onclick: on_list_services,
                            if current_state.running_action == "List" { "Loading..." } else { "List Services" }
                        }
                    }
                }

                // Service list section
                if !sorted_services.is_empty() {
                    div { class: "section",
                        h3 { class: "section-title", 
                            "Services ({current_state.services.len()})" 
                        }
                        div { class: "service-list",
                            table { class: "data-table",
                                thead {
                                    tr {
                                        StaticHeader { label: "".to_string() }
                                        SortableHeader {
                                            column: ServicesSortColumn::Name,
                                            label: "Name".to_string(),
                                            current_sort: current_state.sort_column,
                                            direction: sort_dir,
                                            on_sort: move |col| update_service_sort(state, col),
                                        }
                                        SortableHeader {
                                            column: ServicesSortColumn::DisplayName,
                                            label: "Display Name".to_string(),
                                            current_sort: current_state.sort_column,
                                            direction: sort_dir,
                                            on_sort: move |col| update_service_sort(state, col),
                                        }
                                        SortableHeader {
                                            column: ServicesSortColumn::Status,
                                            label: "Status".to_string(),
                                            current_sort: current_state.sort_column,
                                            direction: sort_dir,
                                            on_sort: move |col| update_service_sort(state, col),
                                        }
                                        SortableHeader {
                                            column: ServicesSortColumn::StartType,
                                            label: "Start Type".to_string(),
                                            current_sort: current_state.sort_column,
                                            direction: sort_dir,
                                            on_sort: move |col| update_service_sort(state, col),
                                        }
                                    }
                                }
                                tbody {
                                    for svc in sorted_services.iter() {
                                        {
                                            let is_selected = current_state.selected_service == svc.name;
                                            let svc_name = svc.name.clone();
                                            let status_class = match svc.status.as_str() {
                                                "Running" => "success",
                                                "Stopped" => "muted",
                                                _ => "",
                                            };
                                            rsx! {
                                                tr {
                                                    class: if is_selected { "selected" } else { "" },
                                                    onclick: move |_| {
                                                        state.write().selected_service = svc_name.clone();
                                                    },
                                                    td { 
                                                        input { 
                                                            r#type: "radio",
                                                            checked: is_selected,
                                                            onchange: move |_| {},
                                                        }
                                                    }
                                                    td { class: "mono", "{svc.name}" }
                                                    td { "{svc.display_name}" }
                                                    td { class: status_class, "{svc.status}" }
                                                    td { class: "muted", "{svc.start_type}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Service actions section
                div { class: "section",
                    h3 { class: "section-title", "Service Actions" }
                    
                    if current_state.selected_service.is_empty() {
                        div { class: "status-bar",
                            "Select a service from the list above to perform actions"
                        }
                    } else {
                        div { class: "action-bar",
                            span { class: "mono", "Selected: {current_state.selected_service}" }
                            
                            div { class: "action-bar-divider" }

                            button {
                                class: "btn btn-primary",
                                disabled: current_state.is_running || !is_admin,
                                onclick: on_start_service,
                                title: if !is_admin { "Requires Administrator" } else { "" },
                                if current_state.running_action == "Start" { "Starting..." } else { "▶ Start" }
                            }

                            button {
                                class: "btn btn-danger",
                                disabled: current_state.is_running || !is_admin,
                                onclick: on_stop_service,
                                title: if !is_admin { "Requires Administrator" } else { "" },
                                if current_state.running_action == "Stop" { "Stopping..." } else { "⏹ Stop" }
                            }

                            button {
                                class: "btn btn-secondary",
                                disabled: current_state.is_running || !is_admin,
                                onclick: on_restart_service,
                                title: if !is_admin { "Requires Administrator" } else { "" },
                                if current_state.running_action == "Restart" { "Restarting..." } else { "🔄 Restart" }
                            }

                            button {
                                class: "btn btn-ghost",
                                disabled: current_state.is_running,
                                onclick: on_get_details,
                                if current_state.running_action == "Details" { "Loading..." } else { "ℹ Details" }
                            }
                        }

                        if !is_admin {
                            div { class: "status-bar warning",
                                "⚠️ Start/Stop/Restart require Administrator privileges"
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

            // Fixed output panel at bottom
            OutputPanel {
                outputs: outputs_for_panel,
                on_clear: move |_| on_clear(()),
            }
        }
    }
}

