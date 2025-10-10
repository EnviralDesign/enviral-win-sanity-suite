"""Port inspection utilities."""

from __future__ import annotations

from dataclasses import dataclass
from ipaddress import ip_address
from typing import List

import psutil


@dataclass(slots=True)
class PortBinding:
    address: str
    pid: int
    state: str
    process_name: str
    ip: str
    is_loopback: bool
    is_unspecified: bool


def list_bindings(port: int) -> List[PortBinding]:
    """Return all TCP bindings for the given port."""

    bindings: List[PortBinding] = []
    for conn in psutil.net_connections(kind="tcp"):
        if not conn.laddr:
            continue
        if conn.laddr.port != port:
            continue

        pid = conn.pid or 0
        try:
            process_name = psutil.Process(pid).name() if pid else "<unknown>"
        except psutil.Error:
            process_name = "<unknown>"

        ip = conn.laddr.ip
        address = f"{ip}:{conn.laddr.port}"
        try:
            ip_obj = ip_address(ip)
            is_loopback = ip_obj.is_loopback
            is_unspecified = ip_obj.is_unspecified
        except ValueError:
            is_loopback = ip.startswith("127.") or ip in {"::1"}
            is_unspecified = ip in {"0.0.0.0", "::"}

        bindings.append(
            PortBinding(
                address=address,
                pid=pid,
                state=conn.status,
                process_name=process_name,
                ip=ip,
                is_loopback=is_loopback,
                is_unspecified=is_unspecified,
            )
        )

    return bindings


__all__ = ["PortBinding", "list_bindings"]
