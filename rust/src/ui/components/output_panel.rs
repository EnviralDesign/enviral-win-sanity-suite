//! Reusable output panel component
//!
//! A fixed-height output panel with copy functionality.

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

    rsx! {
        div { class: "output-panel-container",
            div { class: "output-panel-fixed",
                div { class: "output-panel-header",
                    span { class: "output-panel-title", "📋 Output" }
                    div { class: "output-panel-actions",
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
