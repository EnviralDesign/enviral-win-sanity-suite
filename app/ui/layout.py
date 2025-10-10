"""UI layout builders for the Windows Sanity Suite."""

from __future__ import annotations

from dataclasses import dataclass

from dearpygui import dearpygui as dpg


@dataclass(slots=True)
class WindowState:
    is_admin: bool
    active_user: str
    status_message: str = "Ready"


@dataclass(slots=True)
class LayoutHandles:
    root_window: str
    badge_text: str
    user_text: str
    search_input: str
    sidebar_container: str
    content_container: str
    log_output: str
    status_text: str


def create_main_window(state: WindowState) -> LayoutHandles:
    """Build the primary Dear PyGui window structure and return widget handles."""

    root_tag = "sanity_main_window"
    badge_tag = "sanity_badge"
    user_tag = "sanity_user"
    search_tag = "sanity_search"
    sidebar_tag = "sanity_sidebar"
    content_tag = "sanity_content"
    log_tag = "sanity_log_output"
    status_tag = "sanity_status_text"

    with dpg.window(
        label="",
        tag=root_tag,
        no_title_bar=True,
        no_resize=True,
        no_move=True,
        no_close=True,
        no_scrollbar=True,
        pos=(0, 0),
    ):
        with dpg.group(horizontal=True, tag="sanity_top_bar_group"):
            dpg.add_text("🛡️ Elevated" if state.is_admin else "🔒 Standard", tag=badge_tag)
            dpg.add_spacer(width=8)
            dpg.add_text(f"User: {state.active_user}", tag=user_tag)
            dpg.add_spacer(width=16)
            dpg.add_input_text(
                hint="Quick search",
                tag=search_tag,
                width=240,
            )

        dpg.add_separator()

        with dpg.child_window(
            tag="sanity_body_container",
            autosize_x=True,
            height=-1,
            border=False,
            no_scrollbar=True,
        ):
            with dpg.group(horizontal=True, tag="sanity_body_group"):
                dpg.add_child_window(
                    tag=sidebar_tag,
                    border=False,
                    width=220,
                height=-1,
                no_scrollbar=True,
                )

                dpg.add_child_window(
                    tag=content_tag,
                    border=False,
                    width=-1,
                    height=-1,
                no_scrollbar=True,
                )

        dpg.add_separator()

        with dpg.child_window(
            tag="sanity_log_panel",
            autosize_x=True,
            height=160,
            border=True,
            no_scrollbar=True,
        ):
            dpg.add_input_text(
                tag=log_tag,
                multiline=True,
                readonly=True,
                width=-1,
                height=-1,
            )

        dpg.add_separator()
        dpg.add_text(state.status_message, tag=status_tag)

    return LayoutHandles(
        root_window=root_tag,
        badge_text=badge_tag,
        user_text=user_tag,
        search_input=search_tag,
        sidebar_container=sidebar_tag,
        content_container=content_tag,
        log_output=log_tag,
        status_text=status_tag,
    )


def sync_layout_to_viewport(handles: LayoutHandles) -> None:
    """Resize the root window to match the viewport and keep it anchored."""

    width = dpg.get_viewport_width()
    height = dpg.get_viewport_height()
    dpg.configure_item(handles.root_window, width=width, height=height, pos=(0, 0))


__all__ = ["LayoutHandles", "WindowState", "create_main_window", "sync_layout_to_viewport"]
