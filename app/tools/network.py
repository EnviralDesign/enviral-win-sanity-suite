"""Network quick fix actions."""

from __future__ import annotations

from app.core.exec import CommandSpec
from app.tools.base import Action, Tool


class NetworkTool(Tool):
    name = "network"
    description = "Network diagnostics and quick fixes"

    def register_actions(self) -> None:
        self.actions["flush_dns"] = Action(
            id="flush_dns",
            label="Flush DNS",
            description="ipconfig /flushdns",
            exec_steps=[CommandSpec(executor="cmd", command="ipconfig /flushdns", timeout=15)],
        )

        self.actions["winsock_reset"] = Action(
            id="winsock_reset",
            label="Reset Winsock",
            description="netsh winsock reset",
            exec_steps=[
                CommandSpec(
                    executor="cmd",
                    command="netsh winsock reset",
                    timeout=15,
                    elevate=True,
                )
            ],
            requires_admin=True,
        )

        self.actions["renew_ip"] = Action(
            id="renew_ip",
            label="Renew IP",
            description="ipconfig /release && ipconfig /renew",
            exec_steps=[
                CommandSpec(executor="cmd", command="ipconfig /release", timeout=30),
                CommandSpec(executor="cmd", command="ipconfig /renew", timeout=30),
            ],
        )

        self.actions["ping_host"] = Action(
            id="ping_host",
            label="Ping Host",
            description="ping -n 4 <host>",
            exec_steps=[
                CommandSpec(
                    executor="cmd",
                    command="ping -n 4 $env:TARGET_HOST",
                    timeout=20,
                    env={"TARGET_HOST": "8.8.8.8"},
                )
            ],
        )

        self.actions["curl_head"] = Action(
            id="curl_head",
            label="HTTP HEAD",
            description="curl -I <url>",
            exec_steps=[
                CommandSpec(
                    executor="cmd",
                    command="curl -I $env:TARGET_URL",
                    timeout=20,
                    env={"TARGET_URL": "https://www.microsoft.com"},
                )
            ],
        )


__all__ = ["NetworkTool"]
