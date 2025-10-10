"""Windows Sanity Suite application package."""

from __future__ import annotations

from importlib import metadata


def get_version() -> str:
    """Return the installed package version.

    When running from a checkout (e.g. via ``uv run``) the metadata lookup
    falls back to the default development version if the distribution is not
    yet installed.
    """

    try:
        return metadata.version("windows-sanity-suite")
    except metadata.PackageNotFoundError:  # pragma: no cover - dev path
        return "0.1.0"


__all__ = ["get_version"]
