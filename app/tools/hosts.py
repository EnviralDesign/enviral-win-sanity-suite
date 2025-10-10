"""Hosts file tooling."""

from __future__ import annotations

from pathlib import Path
from typing import List


HOSTS_PATH = Path(r"C:\Windows\System32\drivers\etc\hosts")


def read_hosts() -> List[str]:
    with HOSTS_PATH.open("r", encoding="utf-8") as fh:
        return fh.readlines()


def write_hosts(lines: List[str]) -> None:
    with HOSTS_PATH.open("w", encoding="utf-8") as fh:
        fh.writelines(lines)


__all__ = ["read_hosts", "write_hosts", "HOSTS_PATH"]
