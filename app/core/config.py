"""Configuration management for the Windows Sanity Suite."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any, Dict

import tomllib
import tomli_w

from .admin import get_appdata_dir


APP_DIR_NAME = "SanitySuite"
CONFIG_FILE_NAME = "config.toml"
WORKFLOWS_DIR_NAME = "workflows"


def get_app_root() -> Path:
    """Return the root application data directory."""

    return Path(get_appdata_dir()) / APP_DIR_NAME


def ensure_app_dirs() -> None:
    """Ensure the application data directory structure exists."""

    root = get_app_root()
    (root / "logs").mkdir(parents=True, exist_ok=True)
    (root / WORKFLOWS_DIR_NAME).mkdir(parents=True, exist_ok=True)


def load_config() -> Dict[str, Any]:
    """Load configuration from the TOML file, returning defaults if missing."""

    ensure_app_dirs()
    config_path = get_app_root() / CONFIG_FILE_NAME
    if not config_path.exists():
        return {
            "ui": {
                "theme": "dark",
                "last_active_tab": "dashboard",
                "default_port": 3010,
                "default_host": "localhost",
            },
            "network": {
                "probe_host": "https://www.microsoft.com",
            },
        }

    with config_path.open("rb") as fh:
        data = tomllib.load(fh)

    return data


def save_config(config: Dict[str, Any]) -> None:
    """Persist configuration to disk."""

    ensure_app_dirs()
    config_path = get_app_root() / CONFIG_FILE_NAME
    with config_path.open("wb") as fh:
        tomli_w.dump(config, fh)


def load_workflow(path: Path) -> Dict[str, Any]:
    """Load a workflow definition from TOML or JSON."""

    if path.suffix.lower() == ".toml":
        with path.open("rb") as fh:
            return tomllib.load(fh)

    if path.suffix.lower() == ".json":
        with path.open("r", encoding="utf-8") as fh:
            return json.load(fh)

    raise ValueError(f"Unsupported workflow format: {path.suffix}")


def workflows_dir() -> Path:
    return get_app_root() / WORKFLOWS_DIR_NAME


__all__ = [
    "ensure_app_dirs",
    "get_app_root",
    "load_config",
    "load_workflow",
    "save_config",
    "workflows_dir",
]
