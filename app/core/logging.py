"""Logging helpers for the Windows Sanity Suite."""

from __future__ import annotations

import logging
from logging.handlers import TimedRotatingFileHandler
from pathlib import Path

from .config import ensure_app_dirs, get_app_root


LOG_FILE_BASENAME = "app.log"


def configure_logging(level: int = logging.INFO) -> None:
    """Configure application-wide logging with rotation."""

    ensure_app_dirs()
    logs_dir = get_app_root() / "logs"
    logs_dir.mkdir(parents=True, exist_ok=True)

    handler = TimedRotatingFileHandler(
        logs_dir / LOG_FILE_BASENAME,
        when="midnight",
        backupCount=7,
        encoding="utf-8",
    )
    formatter = logging.Formatter(
        fmt="%(asctime)s [%(levelname)s] %(name)s: %(message)s",
        datefmt="%Y-%m-%d %H:%M:%S",
    )
    handler.setFormatter(formatter)

    root_logger = logging.getLogger()
    root_logger.setLevel(level)
    root_logger.addHandler(handler)


__all__ = ["configure_logging"]
