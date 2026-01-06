//! Processes page component
//!
//! Process list with memory/CPU sorting and kill functionality.

use dioxus::prelude::*;

use crate::state::{ProcessesPageState, ProcessesSortColumn};
use crate::system::processes;
use crate::ui::components::{SortableHeader, StaticHeader, SortDirection};

fn update_process_sort(mut state: Signal<ProcessesPageState>, column: ProcessesSortColumn) {
    let mut s = state.write();
    if s.sort_column == Some(column) {
        s.sort_ascending = !s.sort_ascending;
    } else {
        s.sort_column = Some(column);
        s.sort_ascending = true;
        
        // Default sort direction logic override
        match column {
            ProcessesSortColumn::Memory | ProcessesSortColumn::Cpu | 
            ProcessesSortColumn::Handles => s.sort_ascending = false,
            _ => {}
        }
    }
}

/// Processes page
#[component]
pub fn ProcessesPage(is_admin: bool) -> Element {
    let mut state: Signal<ProcessesPageState> = use_context();

    // Refresh processes
    let on_refresh = move |_| {
        let sort_by = state().sort_by.clone();
        state.write().is_running = true;
        state.write().running_action = "Refresh".to_string();
        state.write().status_message = "Loading processes...".to_string();
        state.write().status_type = String::new();

        spawn(async move {
            // Get system info
            let sys_info = processes::get_system_info();
            
            // Get processes based on sort
            let procs = if sort_by == "cpu" {
                processes::get_top_by_cpu(50)
            } else if sort_by == "handles" {
                processes::get_top_by_handles(50)
            } else {
                processes::get_top_by_memory(50)
            };

            let mut s = state.write();
            s.system_info = sys_info;
            s.processes = procs;
            s.is_running = false;
            s.running_action = String::new();
            s.status_message = format!("Loaded {} processes (sorted by {})", s.processes.len(), sort_by);
            s.status_type = "success".to_string();
        });
    };

    // Kill process handler
    let on_kill = move |_| {
        let pid = match state().selected_pid {
            Some(p) => p,
            None => {
                state.write().status_message = "Select a process first".to_string();
                state.write().status_type = "warning".to_string();
                return;
            }
        };

        state.write().is_running = true;
        state.write().running_action = "Kill".to_string();
        state.write().status_message = format!("Killing process {}...", pid);

        spawn(async move {
            match processes::kill_process(pid) {
                Ok(_) => {
                    let mut s = state.write();
                    s.status_message = format!("Process {} terminated", pid);
                    s.status_type = "success".to_string();
                    s.selected_pid = None;
                    // Remove from list
                    s.processes.retain(|p| p.pid != pid);
                }
                Err(e) => {
                    let mut s = state.write();
                    s.status_message = e;
                    s.status_type = "error".to_string();
                }
            }
            state.write().is_running = false;
            state.write().running_action = String::new();
        });
    };



    let current_state = state();
    
    // Sort processes based on current sort state (local sort of the fetched list)
    let mut sorted_processes = current_state.processes.clone();
    if let Some(sort_col) = current_state.sort_column {
        let asc = current_state.sort_ascending;
        sorted_processes.sort_by(|a, b| {
            let cmp = match sort_col {
                ProcessesSortColumn::Pid => a.pid.cmp(&b.pid),
                ProcessesSortColumn::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                ProcessesSortColumn::Memory => a.memory_mb.partial_cmp(&b.memory_mb).unwrap_or(std::cmp::Ordering::Equal),
                ProcessesSortColumn::Handles => a.handle_count.cmp(&b.handle_count),
                ProcessesSortColumn::Cpu => a.cpu_percent.partial_cmp(&b.cpu_percent).unwrap_or(std::cmp::Ordering::Equal),
                ProcessesSortColumn::Status => a.status.cmp(&b.status),
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
        header { class: "page-header",
            h1 { class: "page-title", "📊 Processes" }
            p { class: "page-subtitle", "Top processes by resource usage" }
        }

        div { class: "page-split-layout",
            div { class: "page-controls",
                // System overview
                div { class: "section",
                    h3 { class: "section-title", "System Overview" }
                    div { class: "stats-grid",
                        div { class: "stat-card",
                            span { class: "stat-value", 
                                "{current_state.system_info.used_memory_gb:.1} / {current_state.system_info.total_memory_gb:.1} GB" 
                            }
                            span { class: "stat-label", "Memory" }
                        }
                        div { class: "stat-card",
                            span { class: "stat-value", "{current_state.system_info.cpu_count}" }
                            span { class: "stat-label", "CPU Cores" }
                        }
                        div { class: "stat-card",
                            span { class: "stat-value", "{current_state.system_info.process_count}" }
                            span { class: "stat-label", "Processes" }
                        }
                    }
                }

                // Controls
                div { class: "section",
                    h3 { class: "section-title", "View" }
                    div { class: "action-bar",
                        div { class: "action-bar-group",
                            label { "Fetch Top 50 By:" }
                            select {
                                class: "input",
                                value: "{current_state.sort_by}",
                                onchange: move |e| {
                                    state.write().sort_by = e.value();
                                },
                                option { value: "memory", "Memory" }
                                option { value: "cpu", "CPU" }
                                option { value: "handles", "Handles (leak detection)" }
                            }
                        }

                        button {
                            class: "btn btn-primary",
                            disabled: current_state.is_running,
                            onclick: on_refresh,
                            if current_state.running_action == "Refresh" { "Loading..." } else { "🔄 Refresh" }
                        }

                        if current_state.selected_pid.is_some() {
                            button {
                                class: "btn btn-danger",
                                disabled: current_state.is_running,
                                onclick: on_kill,
                                title: if !is_admin { "May require Administrator for some processes" } else { "" },
                                if current_state.running_action == "Kill" { "Killing..." } else { "☠ Kill Process" }
                            }
                        }
                    }
                }

                // Process list
                if !sorted_processes.is_empty() {
                    div { class: "section",
                        h3 { class: "section-title", "Process List" }
                        div { class: "service-list",
                            table { class: "data-table",
                                thead {
                                    tr {
                                        StaticHeader { label: "".to_string() }
                                        SortableHeader {
                                            column: ProcessesSortColumn::Pid,
                                            label: "PID".to_string(),
                                            current_sort: current_state.sort_column,
                                            direction: sort_dir,
                                            on_sort: move |col| update_process_sort(state, col),
                                        }
                                        SortableHeader {
                                            column: ProcessesSortColumn::Name,
                                            label: "Name".to_string(),
                                            current_sort: current_state.sort_column,
                                            direction: sort_dir,
                                            on_sort: move |col| update_process_sort(state, col),
                                        }
                                        SortableHeader {
                                            column: ProcessesSortColumn::Memory,
                                            label: "Memory".to_string(),
                                            current_sort: current_state.sort_column,
                                            direction: sort_dir,
                                            on_sort: move |col| update_process_sort(state, col),
                                        }
                                        SortableHeader {
                                            column: ProcessesSortColumn::Handles,
                                            label: "Handles".to_string(),
                                            current_sort: current_state.sort_column,
                                            direction: sort_dir,
                                            on_sort: move |col| update_process_sort(state, col),
                                        }
                                        SortableHeader {
                                            column: ProcessesSortColumn::Cpu,
                                            label: "CPU %".to_string(),
                                            current_sort: current_state.sort_column,
                                            direction: sort_dir,
                                            on_sort: move |col| update_process_sort(state, col),
                                        }
                                        SortableHeader {
                                            column: ProcessesSortColumn::Status,
                                            label: "Status".to_string(),
                                            current_sort: current_state.sort_column,
                                            direction: sort_dir,
                                            on_sort: move |col| update_process_sort(state, col),
                                        }
                                    }
                                }
                                tbody {
                                    for proc in sorted_processes.iter() {
                                        {
                                            let is_selected = current_state.selected_pid == Some(proc.pid);
                                            let pid = proc.pid;
                                            let handle_display = if proc.handle_count > 0 { 
                                                proc.handle_count.to_string() 
                                            } else { 
                                                "-".to_string() 
                                            };
                                            rsx! {
                                                tr {
                                                    class: if is_selected { "selected" } else { "" },
                                                    onclick: move |_| {
                                                        state.write().selected_pid = Some(pid);
                                                    },
                                                    td {
                                                        input {
                                                            r#type: "radio",
                                                            checked: is_selected,
                                                            onchange: move |_| {},
                                                        }
                                                    }
                                                    td { class: "mono", "{proc.pid}" }
                                                    td { "{proc.name}" }
                                                    td { class: "mono", "{proc.memory_mb:.1} MB" }
                                                    td { class: "mono", "{handle_display}" }
                                                    td { class: "mono", "{proc.cpu_percent:.1}%" }
                                                    td { class: "muted", "{proc.status}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    div { class: "section",
                        div { class: "empty-state",
                            div { class: "empty-state-icon", "📊" }
                            p { class: "empty-state-text", "Click Refresh to load process list" }
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
