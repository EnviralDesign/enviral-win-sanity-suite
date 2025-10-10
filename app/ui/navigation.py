"""Navigation controller for the Windows Sanity Suite UI."""

from __future__ import annotations

from dataclasses import dataclass
from typing import Callable, Dict, Iterable, List

from dearpygui import dearpygui as dpg

from .layout import LayoutHandles


TabBuilder = Callable[[str], None]


@dataclass(slots=True)
class TabDefinition:
    id: str
    label: str
    builder: TabBuilder


class NavigationController:
    """Manages sidebar navigation and tab content switching."""

    def __init__(self, handles: LayoutHandles, tabs: Iterable[TabDefinition]) -> None:
        self.handles = handles
        self._tabs: List[TabDefinition] = list(tabs)
        self._content_tags: Dict[str, str] = {}
        self._nav_tags: Dict[str, str] = {}
        self._active_tab: str | None = None

        if not self._tabs:
            raise ValueError("At least one tab definition is required")

        self._build_sidebar()
        self._build_content()
        self.show_tab(self._tabs[0].id)

    def _build_sidebar(self) -> None:
        with dpg.group(parent=self.handles.sidebar_container, tag="sanity_sidebar_nav"):
            for tab in self._tabs:
                nav_tag = f"sanity_nav_{tab.id}"
                self._nav_tags[tab.id] = nav_tag
                dpg.add_selectable(
                    label=tab.label,
                    tag=nav_tag,
                    span_columns=True,
                    callback=self._on_tab_selected,
                    user_data=tab.id,
                )

    def _build_content(self) -> None:
        for tab in self._tabs:
            content_tag = f"sanity_content_{tab.id}"
            self._content_tags[tab.id] = content_tag
            dpg.add_child_window(
                parent=self.handles.content_container,
                tag=content_tag,
                show=False,
                border=False,
                autosize_x=True,
                autosize_y=True,
                no_scrollbar=False,
            )
            tab.builder(content_tag)

    def show_tab(self, tab_id: str) -> None:
        if tab_id == self._active_tab:
            return

        if tab_id not in self._content_tags:
            raise KeyError(f"Unknown tab: {tab_id}")

        for key, content_tag in self._content_tags.items():
            dpg.configure_item(content_tag, show=key == tab_id)
            nav_tag = self._nav_tags[key]
            dpg.set_value(nav_tag, key == tab_id)

        self._active_tab = tab_id

    def _on_tab_selected(self, sender: int | str, app_data: bool, user_data: str) -> None:
        self.show_tab(user_data)


__all__ = ["NavigationController", "TabDefinition", "TabBuilder"]

