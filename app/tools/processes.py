"""Process handle inspection utilities."""

from __future__ import annotations

import platform
from dataclasses import dataclass
from typing import List

import psutil


@dataclass(slots=True)
class ProcessHandleInfo:
    process_name: str
    pid: int
    handle_count: int
    path: str


def get_top_processes_by_handles(limit: int = 25) -> List[ProcessHandleInfo]:
    """Return the top processes by handle count (Windows only)."""

    if platform.system() != "Windows":
        # num_handles() is Windows-specific
        return []

    processes: List[ProcessHandleInfo] = []

    for proc in psutil.process_iter(attrs=["pid", "name"]):
        try:
            pid = proc.pid
            process_name = proc.name()

            # Get handle count (Windows-specific)
            handle_count = proc.num_handles()

            # Try to get the process executable path
            try:
                path = proc.exe()
            except (psutil.AccessDenied, psutil.NoSuchProcess):
                path = "<access denied>"

            processes.append(
                ProcessHandleInfo(
                    process_name=process_name,
                    pid=pid,
                    handle_count=handle_count,
                    path=path,
                )
            )

        except (psutil.NoSuchProcess, psutil.AccessDenied):
            # Process may have terminated or we don't have access
            continue
        except AttributeError:
            # num_handles() might not be available on this platform
            continue

    # Sort by handle count descending and return top N
    processes.sort(key=lambda p: p.handle_count, reverse=True)
    return processes[:limit]


__all__ = ["ProcessHandleInfo", "get_top_processes_by_handles"]
