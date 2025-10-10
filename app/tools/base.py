"""Base types for tools and actions."""

from __future__ import annotations

from abc import ABC, abstractmethod
from typing import Dict, List

from pydantic import BaseModel, Field

from app.core.exec import CommandSpec


class ActionParameters(BaseModel):
    """Base class for action parameter schemas."""

    model_config = {"arbitrary_types_allowed": True}


class Action(BaseModel):
    """Declarative description of a tool action."""

    id: str
    label: str
    description: str
    parameters: type[ActionParameters] | None = None
    requires_admin: bool = False
    exec_steps: List[CommandSpec] = Field(default_factory=list)


class Tool(ABC):
    """Abstract base class for tool groups."""

    name: str
    description: str

    def __init__(self) -> None:
        self.actions: Dict[str, Action] = {}

    @abstractmethod
    def register_actions(self) -> None:
        """Populate the actions dictionary."""

    def get_action(self, action_id: str) -> Action:
        return self.actions[action_id]


__all__ = ["Action", "ActionParameters", "Tool"]
