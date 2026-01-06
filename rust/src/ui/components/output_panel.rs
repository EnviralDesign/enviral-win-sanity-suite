//! Reusable output panel component
//!
//! A fixed-height output panel with copy functionality and expandable reading mode.

use dioxus::prelude::*;
use crate::state::CommandOutput;

/// Reusable output panel that displays command output with copy functionality
#[component]
pub fn OutputPanel(
    /// The command outputs to display
    outputs: Vec<CommandOutput>,
    /// Callback when clear is clicked
    on_clear: EventHandler<()>,
) -> Element {
    // Track expanded state
    let mut is_expanded = use_signal(|| false);

    // Copy handler
    let copy_outputs = outputs.clone();
    let on_copy = move |_| {
        let text: String = copy_outputs
            .iter()
            .map(|o| o.display_output())
            .collect::<Vec<_>>()
            .join("\n---\n");
        
        if let Ok(mut clipboard) = arboard::Clipboard::new() {
            let _ = clipboard.set_text(&text);
        }
    };

    // Clone outputs for expanded view
    let outputs_for_expanded = outputs.clone();
    let copy_outputs_expanded = outputs.clone();
    let on_copy_expanded = move |_| {
        let text: String = copy_outputs_expanded
            .iter()
            .map(|o| o.display_output())
            .collect::<Vec<_>>()
            .join("\n---\n");
        
        if let Ok(mut clipboard) = arboard::Clipboard::new() {
            let _ = clipboard.set_text(&text);
        }
    };

    rsx! {
        // Fullscreen overlay when expanded
        if *is_expanded.read() {
            div { class: "output-panel-overlay",
                onclick: move |_| is_expanded.set(false),
                div { class: "output-panel-expanded",
                    onclick: move |e| e.stop_propagation(),
                    div { class: "output-panel-header",
                        span { class: "output-panel-title", "📋 Output (Reading Mode)" }
                        div { class: "output-panel-actions",
                            button {
                                class: "btn btn-ghost btn-sm",
                                title: "Copy to clipboard",
                                onclick: on_copy_expanded,
                                "📋 Copy"
                            }
                            button {
                                class: "btn btn-ghost btn-sm",
                                onclick: move |_| {
                                    on_clear.call(());
                                },
                                "Clear"
                            }
                            button {
                                class: "btn btn-primary btn-sm",
                                onclick: move |_| is_expanded.set(false),
                                "✕ Close"
                            }
                        }
                    }
                    div { class: "output-panel-content-expanded",
                        if outputs_for_expanded.is_empty() {
                            div { class: "output-panel-empty",
                                "Run a command to see output here"
                            }
                        } else {
                            for output in outputs_for_expanded.iter() {
                                pre { class: "output-text", "{output.display_output()}" }
                            }
                        }
                    }
                }
            }
        }

        // Normal fixed panel
        div { class: "output-panel-container",
            div { class: "output-panel-fixed",
                div { class: "output-panel-header",
                    span { class: "output-panel-title", "📋 Output" }
                    div { class: "output-panel-actions",
                        button {
                            class: "btn btn-ghost btn-sm",
                            title: "Expand to reading mode",
                            onclick: move |_| is_expanded.set(true),
                            "⛶ Expand"
                        }
                        button {
                            class: "btn btn-ghost btn-sm",
                            title: "Copy to clipboard",
                            onclick: on_copy,
                            "📋 Copy"
                        }
                        button {
                            class: "btn btn-ghost btn-sm",
                            onclick: move |_| on_clear.call(()),
                            "Clear"
                        }
                    }
                }
                div { class: "output-panel-content",
                    if outputs.is_empty() {
                        div { class: "output-panel-empty",
                            "Run a command to see output here"
                        }
                    } else {
                        for output in outputs.iter() {
                            pre { class: "output-text", "{output.display_output()}" }
                        }
                    }
                }
            }
        }
    }
}

