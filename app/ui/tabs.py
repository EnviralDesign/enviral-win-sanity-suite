"""Tab builders for the Windows Sanity Suite."""

from __future__ import annotations

from dearpygui import dearpygui as dpg

from app.tools import PortBinding


def build_dashboard_tab(parent: str) -> None:
    with dpg.group(parent=parent):
        dpg.add_text("Dashboard controls coming soon")


def build_ports_tab(parent: str) -> None:
    with dpg.group(parent=parent):
        dpg.add_text("Scan a port to list active listeners")
        dpg.add_input_int(label="Port", default_value=3010, min_value=1, max_value=65535, tag="ports_input_port")
        with dpg.group(horizontal=True):
            dpg.add_button(label="Scan", tag="ports_scan_button")
            dpg.add_button(label="Clear", tag="ports_clear_button")
            dpg.add_button(label="Copy Report", tag="ports_copy_button")
            dpg.add_button(label="Suggest Free Port", tag="ports_suggest_button")
        dpg.add_separator()

        with dpg.table(
            tag="ports_table",
            header_row=True,
            borders_innerH=True,
            borders_innerV=True,
            borders_outerH=True,
            borders_outerV=True,
            row_background=True,
        ):
            dpg.add_table_column(label="PID")
            dpg.add_table_column(label="Process")
            dpg.add_table_column(label="Local Address")
            dpg.add_table_column(label="State")
            dpg.add_table_column(label="Scope")
            dpg.add_table_column(label="Actions")

        dpg.add_text("", tag="ports_status_text")


def build_network_tab(parent: str) -> None:
    with dpg.group(parent=parent):
        dpg.add_text("Network quick fixes")

        with dpg.group(horizontal=True):
            dpg.add_button(label="Flush DNS", tag="network_flush_button")
            dpg.add_button(label="Renew IP", tag="network_renew_button")
            dpg.add_button(label="Winsock Reset", tag="network_winsock_button")
            dpg.add_button(label="Ping", tag="network_ping_button")
            dpg.add_button(label="HTTP HEAD", tag="network_head_button")

        dpg.add_separator()

        with dpg.group(horizontal=True):
            dpg.add_text("Adapter Summary")
            dpg.add_spacer(width=12)
            dpg.add_button(label="Refresh", tag="network_refresh_button")
        with dpg.table(
            tag="network_adapters_table",
            header_row=True,
            borders_innerH=True,
            borders_innerV=True,
            borders_outerH=True,
            borders_outerV=True,
            row_background=True,
        ):
            dpg.add_table_column(label="Name")
            dpg.add_table_column(label="IP Addresses")
            dpg.add_table_column(label="Status")

        dpg.add_separator()
        dpg.add_text("Command Output")
        dpg.add_input_text(tag="network_output", multiline=True, readonly=True, width=-1, height=200)
        dpg.add_text("", tag="network_status_text")


def build_services_tab(parent: str) -> None:
    with dpg.group(parent=parent):
        dpg.add_text("Services manager placeholder")


def build_hosts_tab(parent: str) -> None:
    with dpg.group(parent=parent):
        dpg.add_text("Hosts file viewer placeholder")


def build_workflows_tab(parent: str) -> None:
    with dpg.group(parent=parent):
        dpg.add_text("Workflow runner placeholder")


def build_settings_tab(parent: str) -> None:
    with dpg.group(parent=parent):
        dpg.add_text("Settings placeholder")


__all__ = [
    "build_dashboard_tab",
    "build_ports_tab",
    "build_network_tab",
    "build_services_tab",
    "build_hosts_tab",
    "build_workflows_tab",
    "build_settings_tab",
]

