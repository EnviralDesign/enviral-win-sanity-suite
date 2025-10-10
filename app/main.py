"""Application entry point."""

from __future__ import annotations

import asyncio
import getpass
import logging

from dearpygui import dearpygui as dpg

from . import get_version
from .core import admin as admin_core
from .core.logging import configure_logging
from .network import NetworkController
from .ports import PortsController
from .ui.layout import LayoutHandles, WindowState, create_main_window, sync_layout_to_viewport
from .ui.navigation import NavigationController, TabDefinition
from .ui.tabs import (
    build_dashboard_tab,
    build_hosts_tab,
    build_network_tab,
    build_ports_tab,
    build_services_tab,
    build_settings_tab,
    build_workflows_tab,
)


LOG = logging.getLogger(__name__)


async def _async_main() -> None:
    configure_logging()

    is_admin = admin_core.is_user_admin()
    state = WindowState(
        is_admin=is_admin,
        active_user=getpass.getuser(),
    )

    dpg.create_context()
    dpg.create_viewport(
        title=f"Windows Sanity Suite {get_version()}",
        width=1100,
        height=700,
        resizable=True,
        decorated=True,
    )
    handles = create_main_window(state)
    nav = NavigationController(
        handles,
        tabs=[
            TabDefinition("dashboard", "Dashboard", build_dashboard_tab),
            TabDefinition("ports", "Ports", build_ports_tab),
            TabDefinition("network", "Network", build_network_tab),
            TabDefinition("services", "Services", build_services_tab),
            TabDefinition("hosts", "Hosts", build_hosts_tab),
            TabDefinition("workflows", "Workflows", build_workflows_tab),
            TabDefinition("settings", "Settings", build_settings_tab),
        ],
    )
    PortsController()
    NetworkController(is_admin=is_admin)
    dpg.setup_dearpygui()
    dpg.show_viewport()
    dpg.set_primary_window(handles.root_window, True)
    sync_layout_to_viewport(handles)
    dpg.set_viewport_resize_callback(lambda sender, data: sync_layout_to_viewport(handles))

    LOG.info("Application started")
    while dpg.is_dearpygui_running():
        dpg.render_dearpygui_frame()
        await asyncio.sleep(0)

    dpg.destroy_context()


def main() -> None:
    asyncio.run(_async_main())


if __name__ == "__main__":
    main()
