"""Network tab controller wiring."""

from __future__ import annotations

import asyncio
from typing import Iterable, Optional

import psutil
from dearpygui import dearpygui as dpg

from app.core.exec import AsyncCommandRunner, CommandResult
from app.tools.network import NetworkTool


class NetworkController:
    """Handle interactions for the network quick-fix tab."""

    def __init__(self, *, is_admin: bool) -> None:
        self._loop = asyncio.get_running_loop()
        self._runner = AsyncCommandRunner(max_workers=2)
        self._is_admin = is_admin
        self._tool = NetworkTool()
        self._tool.register_actions()

        self._output_tag = "network_output"
        self._status_tag = "network_status_text"
        self._adapters_table_tag = "network_adapters_table"

        dpg.set_item_callback("network_flush_button", self._make_action_callback("flush_dns"))
        dpg.set_item_callback("network_renew_button", self._make_action_callback("renew_ip"))
        dpg.set_item_callback("network_winsock_button", self._make_action_callback("winsock_reset"))
        dpg.set_item_callback("network_ping_button", self._make_action_callback("ping_host"))
        dpg.set_item_callback("network_head_button", self._make_action_callback("curl_head"))
        dpg.set_item_callback("network_refresh_button", self._on_refresh_clicked)

        if not is_admin:
            dpg.configure_item("network_winsock_button", enabled=False)

        self.refresh_adapters()

    def _make_action_callback(self, action_id: str):
        def _callback(sender: int | str, app_data: None, user_data: None) -> None:
            self._loop.create_task(self._run_action(action_id))

        return _callback

    async def _run_action(self, action_id: str) -> None:
        action = self._tool.get_action(action_id)
        self._set_status(f"Running {action.label}...")
        self._set_output("")
        try:
            results = await self._runner.run_sequence_async(
                action.exec_steps,
                is_admin=self._is_admin,
            )
        except Exception as exc:  # noqa: BLE001 - surface to UI
            self._set_status(f"Action failed: {exc}")
            return

        self._set_output(self._render_results(results))
        if all(result.succeeded for result in results):
            self._set_status(f"{action.label} completed successfully")
        else:
            failed = [str(result.exit_code) for result in results if not result.succeeded]
            self._set_status(f"{action.label} completed with errors (exit codes: {', '.join(failed)})")
        self.refresh_adapters()

    def _render_results(self, results: Iterable[CommandResult]) -> str:
        lines: list[str] = []
        for result in results:
            lines.append(f"$ {result.spec.format_for_logging()} (exit {result.exit_code})")
            if result.stdout:
                lines.append(result.stdout.strip())
            if result.stderr:
                lines.append(result.stderr.strip())
            lines.append("")
        return "\n".join(line for line in lines if line)

    def _set_output(self, text: str) -> None:
        dpg.set_value(self._output_tag, text)

    def _set_status(self, text: str) -> None:
        dpg.set_value(self._status_tag, text)

    def _on_refresh_clicked(self, sender: int | str, app_data: None, user_data: None) -> None:
        self.refresh_adapters()
        self._set_status("Adapter list refreshed")

    def refresh_adapters(self) -> None:
        children = dpg.get_item_children(self._adapters_table_tag, 1) or []
        for child in children:
            dpg.delete_item(child)

        addrs = psutil.net_if_addrs()
        stats = psutil.net_if_stats()
        for name, entries in addrs.items():
            ip_addresses = ", ".join(
                addr.address
                for addr in entries
                if addr.family.name in {"AF_INET", "AF_INET6"}
            ) or "—"

            state = "Up" if stats.get(name, None) and stats[name].isup else "Down"
            with dpg.table_row(parent=self._adapters_table_tag):
                dpg.add_text(name)
                dpg.add_text(ip_addresses)
                dpg.add_text(state)


__all__ = ["NetworkController"]
