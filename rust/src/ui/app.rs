//! Main application component

use dioxus::prelude::*;

use crate::state::{
    CurrentPage, DiskPageState, DockerPageState, HostsPageState, 
    NetworkPageState, PortsPageState, ProcessesPageState, ServicesPageState
};
use crate::system;

use super::components::Sidebar;
use super::pages::{DiskPage, DockerPage, HostsPage, NetworkPage, PortsPage, ProcessesPage, ServicesPage};
use super::STYLES;

/// Main application component
#[component]
pub fn App() -> Element {
    // Global navigation state
    let mut current_page = use_signal(|| CurrentPage::Ports);
    let is_admin = use_signal(|| system::admin::is_elevated());

    // Lifted page state - persists across tab switches
    let ports_state = use_signal(PortsPageState::new);
    let network_state = use_signal(NetworkPageState::default);
    let docker_state = use_signal(DockerPageState::new);
    let services_state = use_signal(ServicesPageState::default);
    let processes_state = use_signal(ProcessesPageState::new);
    let hosts_state = use_signal(HostsPageState::default);
    let disk_state = use_signal(DiskPageState::default);

    // Provide state to child components via context
    use_context_provider(|| ports_state);
    use_context_provider(|| network_state);
    use_context_provider(|| docker_state);
    use_context_provider(|| services_state);
    use_context_provider(|| processes_state);
    use_context_provider(|| hosts_state);
    use_context_provider(|| disk_state);

    rsx! {
        // Inject styles
        style { {STYLES} }

        // App container
        div { class: "app-container",
            // Sidebar navigation
            Sidebar {
                current_page: current_page(),
                is_admin: is_admin(),
                on_navigate: move |page| current_page.set(page),
            }

            // Main content area
            main { class: "main-content",
                match current_page() {
                    CurrentPage::Ports => rsx! { PortsPage { is_admin: is_admin() } },
                    CurrentPage::Network => rsx! { NetworkPage { is_admin: is_admin() } },
                    CurrentPage::Docker => rsx! { DockerPage {} },
                    CurrentPage::Services => rsx! { ServicesPage { is_admin: is_admin() } },
                    CurrentPage::Processes => rsx! { ProcessesPage { is_admin: is_admin() } },
                    CurrentPage::Hosts => rsx! { HostsPage { is_admin: is_admin() } },
                    CurrentPage::Disk => rsx! { DiskPage { is_admin: is_admin() } },
                }
            }
        }
    }
}
