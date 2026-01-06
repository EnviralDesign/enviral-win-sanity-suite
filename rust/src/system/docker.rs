//! Docker utilities
//!
//! Docker container management and diagnostics.

#![allow(dead_code)]

use crate::state::CommandOutput;
use crate::system::command::run_command;


/// Check if Docker is available
pub fn is_docker_available() -> bool {
    which::which("docker").is_ok()
}

/// Get Docker daemon info
pub async fn docker_info() -> CommandOutput {
    run_command("docker", &["info"]).await
}

/// List all containers (running and stopped)
pub async fn docker_ps_all() -> CommandOutput {
    run_command("docker", &["ps", "-a", "--format", "table {{.ID}}\t{{.Image}}\t{{.Status}}\t{{.Names}}\t{{.Ports}}"]).await
}

/// List only running containers
pub async fn docker_ps() -> CommandOutput {
    run_command("docker", &["ps", "--format", "table {{.ID}}\t{{.Image}}\t{{.Status}}\t{{.Names}}\t{{.Ports}}"]).await
}

/// Get container logs
pub async fn docker_logs(container: &str, tail_lines: u32) -> CommandOutput {
    run_command("docker", &["logs", "--tail", &tail_lines.to_string(), container]).await
}

/// Get container logs with follow (last N lines)
pub async fn docker_logs_follow(container: &str, tail_lines: u32) -> CommandOutput {
    run_command("docker", &["logs", "--tail", &tail_lines.to_string(), "--timestamps", container]).await
}

/// Pull an image
pub async fn docker_pull(image: &str) -> CommandOutput {
    run_command("docker", &["pull", image]).await
}

/// Restart a container
pub async fn docker_restart(container: &str) -> CommandOutput {
    run_command("docker", &["restart", container]).await
}

/// Stop a container
pub async fn docker_stop(container: &str) -> CommandOutput {
    run_command("docker", &["stop", container]).await
}

/// Start a container
pub async fn docker_start(container: &str) -> CommandOutput {
    run_command("docker", &["start", container]).await
}

/// Inspect a container (JSON output)
pub async fn docker_inspect(container: &str) -> CommandOutput {
    run_command("docker", &["inspect", container]).await
}

/// List Docker images
pub async fn docker_images() -> CommandOutput {
    run_command("docker", &["images", "--format", "table {{.Repository}}\t{{.Tag}}\t{{.Size}}\t{{.CreatedSince}}"]).await
}

/// Prune unused images
pub async fn docker_image_prune() -> CommandOutput {
    run_command("docker", &["image", "prune", "-f"]).await
}

/// Prune entire system (containers, images, networks, cache)
pub async fn docker_system_prune() -> CommandOutput {
    run_command("docker", &["system", "prune", "-f"]).await
}

/// Docker Compose pull (in current directory or specified path)
pub async fn docker_compose_pull(path: Option<&str>) -> CommandOutput {
    if let Some(p) = path {
        run_command("docker", &["compose", "-f", p, "pull"]).await
    } else {
        run_command("docker", &["compose", "pull"]).await
    }
}

/// Docker Compose up -d (in current directory or specified path)
pub async fn docker_compose_up(path: Option<&str>) -> CommandOutput {
    if let Some(p) = path {
        run_command("docker", &["compose", "-f", p, "up", "-d"]).await
    } else {
        run_command("docker", &["compose", "up", "-d"]).await
    }
}

/// Docker Compose down (in current directory or specified path)
pub async fn docker_compose_down(path: Option<&str>) -> CommandOutput {
    if let Some(p) = path {
        run_command("docker", &["compose", "-f", p, "down"]).await
    } else {
        run_command("docker", &["compose", "down"]).await
    }
}

/// Docker Compose pull and up (force refresh)
pub async fn docker_compose_refresh(path: Option<&str>) -> Vec<CommandOutput> {
    let pull = docker_compose_pull(path).await;
    let up = docker_compose_up(path).await;
    vec![pull, up]
}

/// Get container names for dropdown
pub async fn get_container_names() -> Vec<String> {
    let output = run_command("docker", &["ps", "-a", "--format", "{{.Names}}"]).await;
    if output.exit_code == 0 {
        output.stdout
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        Vec::new()
    }
}

/// Get Docker disk usage
pub async fn docker_disk_usage() -> CommandOutput {
    run_command("docker", &["system", "df"]).await
}

/// Check Docker socket (Windows named pipe or Unix socket)
#[cfg(target_os = "windows")]
pub async fn check_docker_socket() -> CommandOutput {
    // On Windows, Docker uses a named pipe
    run_command("docker", &["version"]).await
}

#[cfg(not(target_os = "windows"))]
pub async fn check_docker_socket() -> CommandOutput {
    // On Unix, check the socket file
    run_command("ls", &["-la", "/var/run/docker.sock"]).await
}
