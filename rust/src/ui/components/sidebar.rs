//! Sidebar navigation component

use dioxus::prelude::*;
use crate::state::CurrentPage;

/// Navigation item definition
pub struct NavItem {
    pub id: CurrentPage,
    pub icon: &'static str,
    pub label: &'static str,
}

/// Sidebar component with navigation
#[component]
pub fn Sidebar(
    current_page: CurrentPage,
    is_admin: bool,
    on_navigate: EventHandler<CurrentPage>,
) -> Element {
    let nav_items = vec![
        NavItem {
            id: CurrentPage::Ports,
            icon: "🔌",
            label: "Ports",
        },
        NavItem {
            id: CurrentPage::Network,
            icon: "🌐",
            label: "Network",
        },
        NavItem {
            id: CurrentPage::Docker,
            icon: "🐳",
            label: "Docker",
        },
        NavItem {
            id: CurrentPage::Services,
            icon: "⚙️",
            label: "Services",
        },
        NavItem {
            id: CurrentPage::Processes,
            icon: "📊",
            label: "Processes",
        },
        NavItem {
            id: CurrentPage::Hosts,
            icon: "📝",
            label: "Hosts",
        },
        NavItem {
            id: CurrentPage::Disk,
            icon: "💾",
            label: "Disk",
        },
    ];

    rsx! {
        aside { class: "sidebar",
            // Header
            div { class: "sidebar-header",
                span { class: "sidebar-logo", "⚡ Sanity Suite" }
            }

            // Navigation
            nav { class: "sidebar-nav",
                for item in nav_items {
                    button {
                        class: if current_page == item.id { "nav-item active" } else { "nav-item" },
                        onclick: move |_| on_navigate.call(item.id),
                        span { class: "nav-icon", "{item.icon}" }
                        span { "{item.label}" }
                    }
                }
            }

            // Footer with admin status
            div { class: "sidebar-footer",
                div {
                    class: if is_admin { "admin-badge elevated" } else { "admin-badge standard" },
                    if is_admin {
                        "🛡️ Administrator"
                    } else {
                        "👤 Standard User"
                    }
                }
            }
        }
    }
}
