"""Workflow loading and execution."""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Dict, List
from pathlib import Path

from app.core.config import load_workflow, workflows_dir
from app.core.exec import AsyncCommandRunner, CommandResult, CommandSpec
from app.tools import PortsTool, NetworkTool


@dataclass
class WorkflowStep:
    action_ref: str
    params: Dict[str, Any]
    on_fail: str = "stop"


class WorkflowEngine:
    """Execute workflows composed of registered tool actions."""

    def __init__(self) -> None:
        self.runner = AsyncCommandRunner()
        self.tools = {
            "ports": PortsTool(),
            "network": NetworkTool(),
        }
        for tool in self.tools.values():
            tool.register_actions()

    async def execute(self, steps: List[WorkflowStep], *, is_admin: bool) -> List[CommandResult]:
        results: List[CommandResult] = []
        for step in steps:
            tool_name, action_id = step.action_ref.split(".")
            tool = self.tools[tool_name]
            action = tool.get_action(action_id)

            specs = action.exec_steps
            step_results = await self.runner.run_sequence_async(specs, is_admin=is_admin)
            results.extend(step_results)
            if any(not r.succeeded for r in step_results) and step.on_fail == "stop":
                break
        return results

    def load_from_file(self, path: Path) -> List[WorkflowStep]:
        data = load_workflow(path)
        steps: List[WorkflowStep] = []
        for raw in data.get("steps", []):
            steps.append(
                WorkflowStep(
                    action_ref=raw["action_ref"],
                    params=raw.get("params", {}),
                    on_fail=raw.get("on_fail", "stop"),
                )
            )
        return steps

    def list_samples(self) -> Dict[str, Path]:
        samples: Dict[str, Path] = {}
        for path in workflows_dir().glob("*.toml"):
            samples[path.stem] = path
        for path in workflows_dir().glob("*.json"):
            samples[path.stem] = path
        return samples


__all__ = ["WorkflowEngine", "WorkflowStep"]

