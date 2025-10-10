"""Reusable Dear PyGui widgets."""

from __future__ import annotations

from typing import Iterable

from dearpygui import dearpygui as dpg


def add_toast(message: str, *, severity: str = "info") -> None:
    """Show a toast notification with the given severity."""

    colors = {
        "info": (0, 122, 204, 255),
        "success": (76, 175, 80, 255),
        "warning": (255, 193, 7, 255),
        "error": (244, 67, 54, 255),
    }
    fg_color = colors.get(severity, colors["info"])

    with dpg.window(label="Notification", no_title_bar=True, modal=False, tag="toast"):
        dpg.add_text(message, color=fg_color)


__all__ = ["add_toast"]
