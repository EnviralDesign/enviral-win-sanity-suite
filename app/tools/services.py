"""Windows services tool."""

from __future__ import annotations

from app.core.exec import CommandSpec
from app.tools.base import Action, Tool


class ServicesTool(Tool):
    name = "services"
    description = "Manage Windows services"

    def register_actions(self) -> None:
        self.actions["list"] = Action(
            id="list",
            label="List Services",
            description="Get-Service",
            exec_steps=[
                CommandSpec(
                    executor="powershell",
                    command="Get-Service | ConvertTo-Json",
                    timeout=15,
                )
            ],
        )

        self.actions["start"] = Action(
            id="start",
            label="Start Service",
            description="Start-Service",
            exec_steps=[
                CommandSpec(
                    executor="powershell",
                    command="Start-Service -Name $env:TARGET_SERVICE",
                    env={"TARGET_SERVICE": ""},
                    elevate=True,
                )
            ],
            requires_admin=True,
        )

        self.actions["stop"] = Action(
            id="stop",
            label="Stop Service",
            description="Stop-Service",
            exec_steps=[
                CommandSpec(
                    executor="powershell",
                    command="Stop-Service -Name $env:TARGET_SERVICE",
                    env={"TARGET_SERVICE": ""},
                    elevate=True,
                )
            ],
            requires_admin=True,
        )


__all__ = ["ServicesTool"]
