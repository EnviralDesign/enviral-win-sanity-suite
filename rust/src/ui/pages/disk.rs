//! Disk page component
//!
//! Drive space, temp folders, cleanup.

use dioxus::prelude::*;

use crate::state::{DiskPageState, DiskSortColumn};
use crate::system::disk;
use crate::ui::components::{SortableHeader, StaticHeader, SortDirection};

fn update_disk_sort(mut state: Signal<DiskPageState>, column: DiskSortColumn) {
    let mut s = state.write();
    if s.sort_column == Some(column) {
        s.sort_ascending = !s.sort_ascending;
    } else {
        s.sort_column = Some(column);
        match column {
             DiskSortColumn::Size | DiskSortColumn::Files => s.sort_ascending = false,
             _ => s.sort_ascending = true,
        }
    }
}

/// Disk page
#[component]
pub fn DiskPage(is_admin: bool) -> Element {
    let mut state: Signal<DiskPageState> = use_context();

    // Refresh disk info
    let on_refresh = move |_| {
        state.write().is_running = true;
        state.write().running_action = "Refresh".to_string();
        state.write().status_message = "Scanning drives...".to_string();
        state.write().status_type = String::new();

        spawn(async move {
            let disks = disk::get_disks();
            let temps = disk::get_temp_folder_sizes();

            let mut s = state.write();
            s.disks = disks;
            s.temp_folders = temps;
            s.is_running = false;
            s.running_action = String::new();
            s.status_message = format!("Found {} drives, {} temp folders", s.disks.len(), s.temp_folders.len());
            s.status_type = "success".to_string();
        });
    };

    // Clean temp folder
    let mut on_clean_temp = move |path: String| {
        state.write().is_running = true;
        state.write().running_action = "Clean".to_string();
        state.write().status_message = format!("Cleaning {}...", path);
        state.write().status_type = String::new();

        spawn(async move {
            match disk::clean_temp_folder(&path) {
                Ok((count, size)) => {
                    // Refresh temp folder sizes
                    let temps = disk::get_temp_folder_sizes();
                    let mut s = state.write();
                    s.temp_folders = temps;
                    s.status_message = format!("Deleted {} files ({:.1} MB)", count, size);
                    s.status_type = "success".to_string();
                }
                Err(e) => {
                    state.write().status_message = e;
                    state.write().status_type = "error".to_string();
                }
            }
            state.write().is_running = false;
            state.write().running_action = String::new();
        });
    };



    let current_state = state();

    // Sort temp folders based on current sort state
    let mut sorted_folders = current_state.temp_folders.clone();
    if let Some(sort_col) = current_state.sort_column {
        let asc = current_state.sort_ascending;
        sorted_folders.sort_by(|a, b| {
            let cmp = match sort_col {
                DiskSortColumn::Folder => a.path.to_lowercase().cmp(&b.path.to_lowercase()),
                DiskSortColumn::Size => a.size_mb.partial_cmp(&b.size_mb).unwrap_or(std::cmp::Ordering::Equal),
                DiskSortColumn::Files => a.file_count.cmp(&b.file_count),
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
            h1 { class: "page-title", "💾 Disk" }
            p { class: "page-subtitle", "Drive space and temp folder management" }
        }

        div { class: "page-split-layout",
            div { class: "page-controls",
                // Refresh button
                div { class: "section",
                    div { class: "action-bar",
                        button {
                            class: "btn btn-primary",
                            disabled: current_state.is_running,
                            onclick: on_refresh,
                            if current_state.running_action == "Refresh" { "Scanning..." } else { "🔄 Refresh" }
                        }
                    }
                }

                // Drives
                if !current_state.disks.is_empty() {
                    div { class: "section",
                        h3 { class: "section-title", "Drives" }
                        div { class: "stats-grid",
                            for disk_info in current_state.disks.iter() {
                                {
                                    let usage_class = if disk_info.usage_percent > 90.0 {
                                        "stat-card stat-card-danger"
                                    } else if disk_info.usage_percent > 75.0 {
                                        "stat-card stat-card-warning"
                                    } else {
                                        "stat-card"
                                    };
                                    rsx! {
                                        div { class: "{usage_class}",
                                            span { class: "stat-value", "{disk_info.mount_point}" }
                                            span { class: "stat-label", 
                                                "{disk_info.available_gb:.1} GB free / {disk_info.total_gb:.1} GB ({disk_info.usage_percent:.0}%)"
                                            }
                                            div { class: "progress-bar",
                                                div { 
                                                    class: "progress-fill",
                                                    style: "width: {disk_info.usage_percent}%",
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Temp folders
                if !current_state.temp_folders.is_empty() {
                    div { class: "section",
                        h3 { class: "section-title", "Temp Folders" }
                        table { class: "data-table",
                            thead {
                                tr {
                                    SortableHeader {
                                        column: DiskSortColumn::Folder,
                                        label: "Folder".to_string(),
                                        current_sort: current_state.sort_column,
                                        direction: sort_dir,
                                        on_sort: move |col| update_disk_sort(state, col),
                                    }
                                    SortableHeader {
                                        column: DiskSortColumn::Size,
                                        label: "Size".to_string(),
                                        current_sort: current_state.sort_column,
                                        direction: sort_dir,
                                        on_sort: move |col| update_disk_sort(state, col),
                                    }
                                    SortableHeader {
                                        column: DiskSortColumn::Files,
                                        label: "Files".to_string(),
                                        current_sort: current_state.sort_column,
                                        direction: sort_dir,
                                        on_sort: move |col| update_disk_sort(state, col),
                                    }
                                    StaticHeader { label: "Actions".to_string() }
                                }
                            }
                            tbody {
                                for folder in sorted_folders.iter() {
                                    {
                                        let path = folder.path.clone();
                                        let path_for_click = path.clone();
                                        let size_class = if folder.size_mb > 1000.0 { "warning" } else { "" };
                                        rsx! {
                                            tr {
                                                td { class: "mono", "{path}" }
                                                td { class: "mono {size_class}", "{folder.size_mb:.1} MB" }
                                                td { class: "mono muted", "{folder.file_count}" }
                                                td {
                                                    button {
                                                        class: "btn btn-danger btn-sm",
                                                        disabled: current_state.is_running,
                                                        onclick: move |_| on_clean_temp(path_for_click.clone()),
                                                        "🧹 Clean"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        if !is_admin {
                            div { class: "status-bar warning", style: "margin-top: 1rem;",
                                "⚠️ Some folders (Windows\\Temp, Prefetch) require Administrator to clean"
                            }
                        }
                    }
                }

                // Empty state
                if current_state.disks.is_empty() && current_state.temp_folders.is_empty() {
                    div { class: "section",
                        div { class: "empty-state",
                            div { class: "empty-state-icon", "💾" }
                            p { class: "empty-state-text", "Click 'Refresh' to scan drives and temp folders" }
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
