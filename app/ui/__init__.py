"""UI package for the Windows Sanity Suite."""

from .layout import LayoutHandles, WindowState, create_main_window, sync_layout_to_viewport
from .navigation import NavigationController, TabDefinition
from .tabs import (
    build_dashboard_tab,
    build_hosts_tab,
    build_network_tab,
    build_ports_tab,
    build_services_tab,
    build_settings_tab,
    build_workflows_tab,
)
from .widgets import add_toast

__all__ = [
    "LayoutHandles",
    "NavigationController",
    "TabDefinition",
    "WindowState",
    "add_toast",
    "build_dashboard_tab",
    "build_hosts_tab",
    "build_network_tab",
    "build_ports_tab",
    "build_services_tab",
    "build_settings_tab",
    "build_workflows_tab",
    "create_main_window",
    "sync_layout_to_viewport",
]
