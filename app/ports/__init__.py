"""Ports tab controller wiring."""

from __future__ import annotations

from dataclasses import dataclass
from typing import Iterable, List

import psutil

from dearpygui import dearpygui as dpg

from app.tools import PortBinding, list_bindings


@dataclass(slots=True)
class PortScanResult:
    bindings: List[PortBinding]


class PortsController:
    """Handle interactions for the ports inspector tab."""

    def __init__(self) -> None:
        self._table_tag = "ports_table"
        self._port_input_tag = "ports_input_port"
        self._status_tag = "ports_status_text"
        self._scan_button_tag = "ports_scan_button"
        self._clear_button_tag = "ports_clear_button"
        self._copy_button_tag = "ports_copy_button"
        self._suggest_button_tag = "ports_suggest_button"

        dpg.set_item_callback(self._scan_button_tag, self._on_scan_clicked)
        dpg.set_item_callback(self._clear_button_tag, self._on_clear_clicked)
        dpg.set_item_callback(self._copy_button_tag, self._on_copy_clicked)
        dpg.set_item_callback(self._suggest_button_tag, self._on_suggest_clicked)
        self._bindings_cache: list[PortBinding] = []

    def _on_scan_clicked(self, sender: int | str, app_data: None, user_data: None) -> None:
        port = dpg.get_value(self._port_input_tag)
        self.scan_port(int(port))

    def _on_clear_clicked(self, sender: int | str, app_data: None, user_data: None) -> None:
        self.clear_results()

    def _on_copy_clicked(self, sender: int | str, app_data: None, user_data: None) -> None:
        if not self._bindings_cache:
            dpg.set_clipboard_text("No data to copy")
            return

        lines = ["PID\tProcess\tLocal Address\tState\tScope"]
        for binding in self._bindings_cache:
            scope = self._describe_scope(binding)
            lines.append(
                "\t".join(
                    [
                        str(binding.pid),
                        binding.process_name,
                        binding.address,
                        binding.state,
                        scope,
                    ]
                )
            )
        dpg.set_clipboard_text("\n".join(lines))
        dpg.set_value(self._status_tag, "Netstat report copied to clipboard")

    def _on_suggest_clicked(self, sender: int | str, app_data: None, user_data: None) -> None:
        suggestion = self.suggest_free_port(range_start=3000, range_end=3100)
        if suggestion is None:
            dpg.set_value(self._status_tag, "No free port found in range 3000-3100")
        else:
            dpg.set_value(self._port_input_tag, suggestion)
            dpg.set_value(self._status_tag, f"Suggested free port: {suggestion}")

    def clear_results(self) -> None:
        self._clear_table()
        dpg.set_value(self._status_tag, "")
        self._bindings_cache = []

    def scan_port(self, port: int) -> None:
        bindings = list_bindings(port)
        self._bindings_cache = bindings
        conflicts = self._detect_conflicts(bindings)
        self._populate_table(bindings, conflicts)
        if bindings:
            if conflicts:
                notice = f"Found {len(bindings)} binding(s); conflict detected between loopback and all-interfaces listeners"
            else:
                notice = f"Found {len(bindings)} binding(s) on port {port}"
            dpg.set_value(self._status_tag, notice)
        else:
            dpg.set_value(self._status_tag, f"No listeners detected on port {port}")

    def _populate_table(self, bindings: Iterable[PortBinding], conflicts: set[int]) -> None:
        self._clear_table()
        for binding in bindings:
            scope = "Loopback" if binding.is_loopback else ("All Interfaces" if binding.is_unspecified else "Specific IP")
            with dpg.table_row(parent=self._table_tag):
                pid_text = dpg.add_text(str(binding.pid))
                if binding.pid in conflicts:
                    dpg.bind_item_theme(pid_text, self._conflict_theme)
                dpg.add_text(binding.process_name)
                dpg.add_text(binding.address)
                dpg.add_text(binding.state)
                scope_cell = dpg.add_text(scope)
                if binding.pid in conflicts:
                    dpg.bind_item_theme(scope_cell, self._conflict_theme)
                dpg.add_button(label="Kill", callback=self._on_kill_clicked, user_data=binding.pid, enabled=binding.pid > 0)

    def _clear_table(self) -> None:
        children = dpg.get_item_children(self._table_tag, 1) or []
        for child in children:
            dpg.delete_item(child)

    @property
    def _conflict_theme(self) -> int:
        if not hasattr(self, "__conflict_theme"):
            with dpg.theme() as theme:
                with dpg.theme_component(dpg.mvAll):
                    dpg.add_theme_color(dpg.mvThemeCol_Text, (255, 99, 71, 255), category=dpg.mvThemeCat_Core)
            self.__conflict_theme = theme
        return self.__conflict_theme

    @staticmethod
    def _detect_conflicts(bindings: Iterable[PortBinding]) -> set[int]:
        loopback_pids = {b.pid for b in bindings if b.is_loopback}
        unspecified_pids = {b.pid for b in bindings if b.is_unspecified}
        return {pid for pid in loopback_pids if pid in unspecified_pids}

    def _on_kill_clicked(self, sender: int | str, app_data: None, user_data: int) -> None:
        pid = int(user_data)
        if pid <= 0:
            return
        try:
            proc = psutil.Process(pid)
            proc.terminate()
            try:
                proc.wait(timeout=5)
            except psutil.TimeoutExpired:
                proc.kill()
        except psutil.Error as exc:
            dpg.set_value(self._status_tag, f"Failed to terminate PID {pid}: {exc}")
        else:
            dpg.set_value(self._status_tag, f"Terminated PID {pid}; rescanning...")
            port = dpg.get_value(self._port_input_tag)
            self.scan_port(int(port))

    @staticmethod
    def _describe_scope(binding: PortBinding) -> str:
        if binding.is_loopback:
            return "Loopback"
        if binding.is_unspecified:
            return "All Interfaces"
        return "Specific IP"

    def suggest_free_port(self, range_start: int, range_end: int) -> int | None:
        occupied = {b.address.split(":")[-1] for b in self._bindings_cache if b.address}
        for port in range(range_start, range_end + 1):
            if str(port) not in occupied:
                try:
                    psutil.net_connections(kind="tcp")
                except psutil.Error:
                    pass
                if not list_bindings(port):
                    return port
        return None


__all__ = ["PortsController"]

