//! Docker page component
//!
//! Docker container management and diagnostics.

use dioxus::prelude::*;

use crate::state::DockerPageState;
use crate::system::docker;

/// Docker page with container management and diagnostics
#[component]
pub fn DockerPage() -> Element {
    // Get persistent state from context
    let mut state: Signal<DockerPageState> = use_context();

    // Check if Docker is available
    let docker_available = docker::is_docker_available();

    // Refresh container list
    let refresh_containers = move || {
        spawn(async move {
            let names = docker::get_container_names().await;
            state.write().container_names = names;
        });
    };

    // Initial container refresh
    use_effect(move || {
        if docker_available {
            refresh_containers();
        }
    });

    // Docker Info handler
    let on_docker_info = move |_| {
        state.write().is_running = true;
        state.write().running_action = "Info".to_string();
        state.write().status_message = "Getting Docker info...".to_string();
        state.write().status_type = String::new();

        spawn(async move {
            let output = docker::docker_info().await;
            let success = output.succeeded();
            
            let mut s = state.write();
            s.command_outputs = vec![output];
            s.is_running = false;
            s.running_action = String::new();

            if success {
                s.status_message = "Docker daemon is running".to_string();
                s.status_type = "success".to_string();
            } else {
                s.status_message = "Docker daemon connection failed".to_string();
                s.status_type = "error".to_string();
            }
        });
    };

    // List Containers handler
    let on_list_containers = move |_| {
        state.write().is_running = true;
        state.write().running_action = "Containers".to_string();
        state.write().status_message = "Listing containers...".to_string();
        state.write().status_type = String::new();

        spawn(async move {
            let output = docker::docker_ps_all().await;
            let success = output.succeeded();
            
            // Also refresh container names
            let names = docker::get_container_names().await;
            
            let mut s = state.write();
            s.command_outputs = vec![output];
            s.container_names = names;
            s.is_running = false;
            s.running_action = String::new();

            if success {
                s.status_message = "Container list refreshed".to_string();
                s.status_type = "success".to_string();
            } else {
                s.status_message = "Failed to list containers".to_string();
                s.status_type = "error".to_string();
            }
        });
    };

    // List Images handler
    let on_list_images = move |_| {
        state.write().is_running = true;
        state.write().running_action = "Images".to_string();
        state.write().status_message = "Listing images...".to_string();
        state.write().status_type = String::new();

        spawn(async move {
            let output = docker::docker_images().await;
            let success = output.succeeded();
            
            let mut s = state.write();
            s.command_outputs = vec![output];
            s.is_running = false;
            s.running_action = String::new();

            if success {
                s.status_message = "Image list refreshed".to_string();
                s.status_type = "success".to_string();
            } else {
                s.status_message = "Failed to list images".to_string();
                s.status_type = "error".to_string();
            }
        });
    };

    // Disk Usage handler
    let on_disk_usage = move |_| {
        state.write().is_running = true;
        state.write().running_action = "Disk".to_string();
        state.write().status_message = "Getting disk usage...".to_string();
        state.write().status_type = String::new();

        spawn(async move {
            let output = docker::docker_disk_usage().await;
            let success = output.succeeded();
            
            let mut s = state.write();
            s.command_outputs = vec![output];
            s.is_running = false;
            s.running_action = String::new();

            if success {
                s.status_message = "Disk usage retrieved".to_string();
                s.status_type = "success".to_string();
            } else {
                s.status_message = "Failed to get disk usage".to_string();
                s.status_type = "error".to_string();
            }
        });
    };

    // Container Logs handler
    let on_get_logs = move |_| {
        let container = state().selected_container.clone();
        if container.is_empty() {
            state.write().status_message = "Select a container first".to_string();
            state.write().status_type = "warning".to_string();
            return;
        }

        let tail_lines = state().log_tail_lines;
        state.write().is_running = true;
        state.write().running_action = "Logs".to_string();
        state.write().status_message = format!("Getting logs for {}...", container);
        state.write().status_type = String::new();

        spawn(async move {
            let output = docker::docker_logs_follow(&container, tail_lines).await;
            let success = output.succeeded();
            
            let mut s = state.write();
            s.command_outputs = vec![output];
            s.is_running = false;
            s.running_action = String::new();

            if success {
                s.status_message = format!("Logs retrieved for {}", container);
                s.status_type = "success".to_string();
            } else {
                s.status_message = format!("Failed to get logs for {}", container);
                s.status_type = "error".to_string();
            }
        });
    };

    // Restart Container handler
    let on_restart_container = move |_| {
        let container = state().selected_container.clone();
        if container.is_empty() {
            state.write().status_message = "Select a container first".to_string();
            state.write().status_type = "warning".to_string();
            return;
        }

        state.write().is_running = true;
        state.write().running_action = "Restart".to_string();
        state.write().status_message = format!("Restarting {}...", container);
        state.write().status_type = String::new();

        spawn(async move {
            let output = docker::docker_restart(&container).await;
            let success = output.succeeded();
            
            let mut s = state.write();
            s.command_outputs = vec![output];
            s.is_running = false;
            s.running_action = String::new();

            if success {
                s.status_message = format!("Container {} restarted", container);
                s.status_type = "success".to_string();
            } else {
                s.status_message = format!("Failed to restart {}", container);
                s.status_type = "error".to_string();
            }
        });
    };

    // Pull Image handler
    let on_pull_image = move |_| {
        let image = state().image_input.clone();
        if image.is_empty() {
            state.write().status_message = "Enter an image name first".to_string();
            state.write().status_type = "warning".to_string();
            return;
        }

        state.write().is_running = true;
        state.write().running_action = "Pull".to_string();
        state.write().status_message = format!("Pulling {}...", image);
        state.write().status_type = String::new();

        spawn(async move {
            let output = docker::docker_pull(&image).await;
            let success = output.succeeded();
            
            let mut s = state.write();
            s.command_outputs = vec![output];
            s.is_running = false;
            s.running_action = String::new();

            if success {
                s.status_message = format!("Successfully pulled {}", image);
                s.status_type = "success".to_string();
            } else {
                s.status_message = format!("Failed to pull {}", image);
                s.status_type = "error".to_string();
            }
        });
    };

    let on_compose_refresh = move |_| {
        let path = state().compose_path.clone();

        state.write().is_running = true;
        state.write().running_action = "Compose".to_string();
        state.write().status_message = "Running docker compose pull && up -d...".to_string();
        state.write().status_type = String::new();

        spawn(async move {
            let path_owned = if path.is_empty() { None } else { Some(path.clone()) };
            let outputs = docker::docker_compose_refresh(path_owned.as_deref()).await;
            let all_success = outputs.iter().all(|o| o.succeeded());
            
            let mut s = state.write();
            s.command_outputs = outputs;
            s.is_running = false;
            s.running_action = String::new();

            if all_success {
                s.status_message = "Compose refresh completed".to_string();
                s.status_type = "success".to_string();
            } else {
                s.status_message = "Compose refresh had errors".to_string();
                s.status_type = "error".to_string();
            }
        });
    };

    // Image Prune handler
    let on_image_prune = move |_| {
        state.write().is_running = true;
        state.write().running_action = "Prune".to_string();
        state.write().status_message = "Pruning unused images...".to_string();
        state.write().status_type = String::new();

        spawn(async move {
            let output = docker::docker_image_prune().await;
            let success = output.succeeded();
            
            let mut s = state.write();
            s.command_outputs = vec![output];
            s.is_running = false;
            s.running_action = String::new();

            if success {
                s.status_message = "Unused images pruned".to_string();
                s.status_type = "success".to_string();
            } else {
                s.status_message = "Image prune failed".to_string();
                s.status_type = "error".to_string();
            }
        });
    };

    // System Prune handler
    let on_system_prune = move |_| {
        state.write().is_running = true;
        state.write().running_action = "System Prune".to_string();
        state.write().status_message = "Pruning Docker system...".to_string();
        state.write().status_type = String::new();

        spawn(async move {
            let output = docker::docker_system_prune().await;
            let success = output.succeeded();
            
            let mut s = state.write();
            s.command_outputs = vec![output];
            s.is_running = false;
            s.running_action = String::new();

            if success {
                s.status_message = "System prune completed".to_string();
                s.status_type = "success".to_string();
            } else {
                s.status_message = "System prune failed".to_string();
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

    rsx! {
        // Page header
        header { class: "page-header",
            h1 { class: "page-title", "🐳 Docker" }
            p { class: "page-subtitle", "Container management and diagnostics" }
        }

        // Split layout
        div { class: "page-split-layout",
            // Scrollable controls
            div { class: "page-controls",
                if !docker_available {
                    div { class: "status-bar error",
                        "❌ Docker not found in PATH. Install Docker Desktop or add docker to your PATH."
                    }
                } else {
                    // Quick Info Section
                    div { class: "section",
                        h3 { class: "section-title", "Quick Info" }
                        div { class: "quick-actions",
                            button {
                                class: "quick-action-btn",
                                disabled: current_state.is_running,
                                onclick: on_docker_info,
                                span { class: "quick-action-icon", "ℹ️" }
                                span { class: "quick-action-label",
                                    if current_state.running_action == "Info" { "Running..." } else { "Docker Info" }
                                }
                            }
                            button {
                                class: "quick-action-btn",
                                disabled: current_state.is_running,
                                onclick: on_list_containers,
                                span { class: "quick-action-icon", "📦" }
                                span { class: "quick-action-label",
                                    if current_state.running_action == "Containers" { "Running..." } else { "List Containers" }
                                }
                            }
                            button {
                                class: "quick-action-btn",
                                disabled: current_state.is_running,
                                onclick: on_list_images,
                                span { class: "quick-action-icon", "🖼️" }
                                span { class: "quick-action-label",
                                    if current_state.running_action == "Images" { "Running..." } else { "List Images" }
                                }
                            }
                            button {
                                class: "quick-action-btn",
                                disabled: current_state.is_running,
                                onclick: on_disk_usage,
                                span { class: "quick-action-icon", "💾" }
                                span { class: "quick-action-label",
                                    if current_state.running_action == "Disk" { "Running..." } else { "Disk Usage" }
                                }
                            }
                        }
                    }

                    // Container Actions Section
                    div { class: "section",
                        h3 { class: "section-title", "Container Actions" }
                        div { class: "action-bar",
                            div { class: "action-bar-group",
                                label { "Container:" }
                                select {
                                    class: "input",
                                    value: "{current_state.selected_container}",
                                    onchange: move |e| {
                                        state.write().selected_container = e.value();
                                    },
                                    option { value: "", "Select container..." }
                                    for name in current_state.container_names.iter() {
                                        option { value: "{name}", "{name}" }
                                    }
                                }
                            }

                            div { class: "action-bar-group",
                                label { "Tail:" }
                                input {
                                    r#type: "number",
                                    class: "input input-number",
                                    value: "{current_state.log_tail_lines}",
                                    min: 10,
                                    max: 1000,
                                    oninput: move |e| {
                                        if let Ok(v) = e.value().parse::<u32>() {
                                            state.write().log_tail_lines = v;
                                        }
                                    },
                                }
                            }

                            button {
                                class: "btn btn-primary",
                                disabled: current_state.is_running || current_state.selected_container.is_empty(),
                                onclick: on_get_logs,
                                if current_state.running_action == "Logs" { "Loading..." } else { "Get Logs" }
                            }

                            button {
                                class: "btn btn-secondary",
                                disabled: current_state.is_running || current_state.selected_container.is_empty(),
                                onclick: on_restart_container,
                                if current_state.running_action == "Restart" { "Restarting..." } else { "Restart" }
                            }
                        }
                    }

                    // Image Pull Section
                    div { class: "section",
                        h3 { class: "section-title", "Pull Image" }
                        div { class: "action-bar",
                            div { class: "action-bar-group",
                                label { "Image:" }
                                input {
                                    r#type: "text",
                                    class: "input",
                                    placeholder: "nginx:latest",
                                    value: "{current_state.image_input}",
                                    oninput: move |e| {
                                        state.write().image_input = e.value();
                                    },
                                }
                            }

                            button {
                                class: "btn btn-primary",
                                disabled: current_state.is_running || current_state.image_input.is_empty(),
                                onclick: on_pull_image,
                                if current_state.running_action == "Pull" { "Pulling..." } else { "Pull" }
                            }
                        }
                    }

                    // Compose Section
                    div { class: "section",
                        h3 { class: "section-title", "Docker Compose" }
                        div { class: "action-bar",
                            div { class: "action-bar-group",
                                label { "Compose file (optional):" }
                                input {
                                    r#type: "text",
                                    class: "input",
                                    placeholder: "docker-compose.yml",
                                    value: "{current_state.compose_path}",
                                    oninput: move |e| {
                                        state.write().compose_path = e.value();
                                    },
                                }
                            }

                            button {
                                class: "btn btn-primary",
                                disabled: current_state.is_running,
                                onclick: on_compose_refresh,
                                title: "Pull latest images and restart containers",
                                if current_state.running_action == "Compose" { "Running..." } else { "Pull & Up" }
                            }
                        }
                    }

                    // Cleanup Section  
                    div { class: "section",
                        h3 { class: "section-title", "Cleanup" }
                        div { class: "quick-actions",
                            button {
                                class: "quick-action-btn",
                                disabled: current_state.is_running,
                                onclick: on_image_prune,
                                title: "Remove dangling/unused images",
                                span { class: "quick-action-icon", "🧹" }
                                span { class: "quick-action-label",
                                    if current_state.running_action == "Prune" { "Pruning..." } else { "Image Prune" }
                                }
                            }
                            button {
                                class: "quick-action-btn quick-action-warning",
                                disabled: current_state.is_running,
                                onclick: on_system_prune,
                                title: "⚠️ Remove all unused containers, networks, images, and cache",
                                span { class: "quick-action-icon", "🗑️" }
                                span { class: "quick-action-label",
                                    if current_state.running_action == "System Prune" { "Pruning..." } else { "System Prune" }
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

            // Fixed output panel at bottom
            crate::ui::components::OutputPanel {
                outputs: outputs_for_panel,
                on_clear: move |_| on_clear(()),
            }
        }
    }
}

