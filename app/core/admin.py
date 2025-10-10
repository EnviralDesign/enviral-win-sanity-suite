"""Helpers for detecting elevation and relaunching with admin rights."""

from __future__ import annotations

import ctypes
import os
import sys
from dataclasses import dataclass
from typing import Iterable, Sequence


CSIDL_APPDATA = 26


def is_user_admin() -> bool:
    """Return True if the current process has administrative privileges."""

    try:
        return bool(ctypes.windll.shell32.IsUserAnAdmin())
    except OSError:
        return False


def get_appdata_dir() -> str:
    """Resolve the application data path for the current user."""

    # Prefer %APPDATA% if available, fallback to shell API.
    appdata = os.getenv("APPDATA")
    if appdata:
        return appdata

    buf = ctypes.create_unicode_buffer(1024)
    ctypes.windll.shell32.SHGetFolderPathW(None, CSIDL_APPDATA, None, 0, buf)
    return buf.value


def relaunch_as_admin(arguments: Sequence[str] | None = None) -> None:
    """Relaunch the current script with administrative privileges."""

    if arguments is None:
        arguments = sys.argv

    params = " ".join(arguments)
    ctypes.windll.shell32.ShellExecuteW(
        None,
        "runas",
        sys.executable,
        params,
        None,
        1,
    )


__all__ = ["get_appdata_dir", "is_user_admin", "relaunch_as_admin"]
