//! Page components

mod disk;
mod docker;
mod hosts;
mod network;
mod ports;
mod processes;
mod services;

pub use disk::DiskPage;
pub use docker::DockerPage;
pub use hosts::HostsPage;
pub use network::NetworkPage;
pub use ports::PortsPage;
pub use processes::ProcessesPage;
pub use services::ServicesPage;
