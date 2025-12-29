//! Sanity Suite - Windows Diagnostic Utility
//!
//! A beautiful, fast desktop application for common Windows diagnostics
//! and quick fixes, built in Rust with Dioxus.

#![allow(non_snake_case)]
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod state;
mod system;
mod ui;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::ui::App;

fn main() {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sanity_suite=debug,info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Sanity Suite...");

    // Check admin status
    let is_admin = system::admin::is_elevated();
    tracing::info!("Running as admin: {}", is_admin);

    // Launch the application
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            dioxus::desktop::Config::new()
                .with_window(
                    dioxus::desktop::WindowBuilder::new()
                        .with_title(if is_admin {
                            "Sanity Suite [Administrator]"
                        } else {
                            "Sanity Suite"
                        })
                        .with_inner_size(dioxus::desktop::LogicalSize::new(1100.0, 700.0))
                        .with_min_inner_size(dioxus::desktop::LogicalSize::new(800.0, 500.0)),
                )
                .with_menu(None)
                .with_background_color((15, 15, 20, 255))
                .with_disable_context_menu(true),
        )
        .launch(App);
}
