"""Processes tab controller wiring."""

from __future__ import annotations

import datetime
from typing import Iterable, List

from dearpygui import dearpygui as dpg

from app.tools import ProcessHandleInfo, get_top_processes_by_handles


class ProcessesController:
    """Handle interactions for the processes handle monitor tab."""

    def __init__(self) -> None:
        self._table_tag = "processes_table"
        self._status_tag = "processes_status_text"
        self._refresh_button_tag = "processes_refresh_button"
        self._copy_button_tag = "processes_copy_button"

        dpg.set_item_callback(self._refresh_button_tag, self._on_refresh_clicked)
        dpg.set_item_callback(self._copy_button_tag, self._on_copy_clicked)
        self._processes_cache: List[ProcessHandleInfo] = []

        # Initial refresh
        self.refresh_processes()

    def _on_refresh_clicked(self, sender: int | str, app_data: None, user_data: None) -> None:
        self.refresh_processes()

    def _on_copy_clicked(self, sender: int | str, app_data: None, user_data: None) -> None:
        if not self._processes_cache:
            dpg.set_clipboard_text("No data to copy")
            return

        lines = ["Process Name\tPID\tHandles\tPath"]
        for proc in self._processes_cache:
            lines.append(
                "\t".join(
                    [
                        proc.process_name,
                        str(proc.pid),
                        str(proc.handle_count),
                        proc.path,
                    ]
                )
            )
        dpg.set_clipboard_text("\n".join(lines))
        dpg.set_value(self._status_tag, f"Process report copied to clipboard (last updated: {datetime.datetime.now().strftime('%H:%M:%S')})")

    def refresh_processes(self) -> None:
        """Refresh the process list and update the UI."""
        try:
            processes = get_top_processes_by_handles()
            self._processes_cache = processes
            self._populate_table(processes)

            timestamp = datetime.datetime.now().strftime('%H:%M:%S')
            if processes:
                dpg.set_value(self._status_tag, f"Found {len(processes)} processes (updated: {timestamp})")
            else:
                dpg.set_value(self._status_tag, f"No processes found (updated: {timestamp})")

        except Exception as exc:  # noqa: BLE001 - surface to UI
            dpg.set_value(self._status_tag, f"Error refreshing processes: {exc}")
            self._clear_table()

    def _populate_table(self, processes: Iterable[ProcessHandleInfo]) -> None:
        """Populate the table with process data."""
        self._clear_table()
        for proc in processes:
            with dpg.table_row(parent=self._table_tag):
                dpg.add_text(proc.process_name)
                dpg.add_text(str(proc.pid))
                dpg.add_text(str(proc.handle_count))
                dpg.add_text(proc.path)

    def _clear_table(self) -> None:
        """Clear all rows from the table."""
        children = dpg.get_item_children(self._table_tag, 1) or []
        for child in children:
            dpg.delete_item(child)


__all__ = ["ProcessesController"]
