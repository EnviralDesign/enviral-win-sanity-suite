"""Command execution utilities for the Windows Sanity Suite."""

from __future__ import annotations

import asyncio
import logging
import os
import shlex
import shutil
import sys
import time
from concurrent.futures import Future, ThreadPoolExecutor
from typing import Dict, List, Literal, Sequence

from pydantic import BaseModel, Field


LOG = logging.getLogger(__name__)

ExecutorType = Literal["process", "powershell", "cmd", "python"]
DEFAULT_TIMEOUT_SECONDS = 45.0


def _pwsh_executable() -> str:
    """Return the preferred PowerShell executable for the host system."""

    return shutil.which("pwsh") or shutil.which("powershell") or "powershell"


class CommandTimedOut(RuntimeError):
    """Raised when a command exceeds the configured timeout."""

    def __init__(self, spec: "CommandSpec", timeout: float) -> None:
        super().__init__(
            f"Command '{spec.command}' timed out after {timeout:.1f} seconds"
        )
        self.spec = spec
        self.timeout = timeout


class CommandSpec(BaseModel):
    """Specification for running a single command step."""

    executor: ExecutorType = "process"
    command: Sequence[str] | str
    timeout: float | None = Field(default=DEFAULT_TIMEOUT_SECONDS)
    elevate: bool = False
    env: Dict[str, str] | None = None
    cwd: str | None = None

    model_config = {
        "arbitrary_types_allowed": True,
        "frozen": True,
    }

    def format_for_logging(self) -> str:
        """Return a human-readable representation of the command."""

        if isinstance(self.command, str):
            return self.command
        return " ".join(shlex.quote(part) for part in self.command)


class CommandResult(BaseModel):
    """Outcome for a completed command step."""

    spec: CommandSpec
    stdout: str
    stderr: str
    exit_code: int
    duration_seconds: float

    @property
    def succeeded(self) -> bool:
        return self.exit_code == 0


async def run_command(
    spec: CommandSpec,
    *,
    is_admin: bool,
    logger: logging.Logger | None = None,
) -> CommandResult:
    """Run a command asynchronously and capture the result."""

    if spec.elevate and not is_admin:
        raise PermissionError(
            "Command requested elevation but the application is not running "
            "with administrative privileges."
        )

    cmd = _build_command(spec)
    env = os.environ.copy()
    if spec.env:
        env.update(spec.env)

    start = time.perf_counter()
    process = await asyncio.create_subprocess_exec(
        *cmd,
        stdout=asyncio.subprocess.PIPE,
        stderr=asyncio.subprocess.PIPE,
        cwd=spec.cwd,
        env=env,
    )

    timeout = spec.timeout
    try:
        if timeout is None:
            stdout_bytes, stderr_bytes = await process.communicate()
        else:
            stdout_bytes, stderr_bytes = await asyncio.wait_for(
                process.communicate(), timeout=timeout
            )
    except asyncio.TimeoutError as exc:
        process.kill()
        await process.communicate()
        raise CommandTimedOut(spec, timeout or 0.0) from exc

    duration = time.perf_counter() - start
    stdout = stdout_bytes.decode("utf-8", errors="replace")
    stderr = stderr_bytes.decode("utf-8", errors="replace")

    if logger:
        logger.debug(
            "Command finished", extra={"command": cmd, "exit_code": process.returncode}
        )

    return CommandResult(
        spec=spec,
        stdout=stdout,
        stderr=stderr,
        exit_code=process.returncode,
        duration_seconds=duration,
    )


def run_command_sync(
    spec: CommandSpec,
    *,
    is_admin: bool,
    logger: logging.Logger | None = None,
) -> CommandResult:
    """Synchronously execute a command using an isolated asyncio loop."""

    return asyncio.run(run_command(spec, is_admin=is_admin, logger=logger))


def _build_command(spec: CommandSpec) -> List[str]:
    """Convert a command specification into an argv list."""

    if spec.executor == "powershell":
        command = spec.command
        if isinstance(command, (list, tuple)):
            command = " ".join(command)
        return [
            _pwsh_executable(),
            "-NoLogo",
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            command,
        ]

    if spec.executor == "cmd":
        command = spec.command
        if isinstance(command, (list, tuple)):
            command = " ".join(command)
        return ["cmd.exe", "/C", command]

    if spec.executor == "python":
        command = spec.command
        if isinstance(command, str):
            argv = shlex.split(command, posix=False)
        else:
            argv = list(command)
        return [sys.executable, "-m", *argv]

    # Default: process execution
    command = spec.command
    if isinstance(command, str):
        return shlex.split(command, posix=False)
    return list(command)


class AsyncCommandRunner:
    """Thread-backed command runner to keep the UI responsive."""

    def __init__(self, max_workers: int = 4) -> None:
        self._executor = ThreadPoolExecutor(
            max_workers=max_workers, thread_name_prefix="sanity-exec"
        )

    def submit_sequence(
        self,
        specs: Sequence[CommandSpec],
        *,
        is_admin: bool,
        logger: logging.Logger | None = None,
    ) -> Future[List[CommandResult]]:
        """Run a sequence of commands in order on a worker thread."""

        specs = list(specs)

        def _runner() -> List[CommandResult]:
            results: List[CommandResult] = []
            for step in specs:
                LOG.debug("Executing step %s", step.format_for_logging())
                result = run_command_sync(step, is_admin=is_admin, logger=logger)
                results.append(result)
            return results

        return self._executor.submit(_runner)

    async def run_sequence_async(
        self,
        specs: Sequence[CommandSpec],
        *,
        is_admin: bool,
        logger: logging.Logger | None = None,
    ) -> List[CommandResult]:
        """Asynchronously execute a sequence of commands."""

        future = self.submit_sequence(specs, is_admin=is_admin, logger=logger)
        return await asyncio.wrap_future(future)

    def shutdown(self) -> None:
        self._executor.shutdown(wait=False, cancel_futures=True)


__all__ = [
    "AsyncCommandRunner",
    "CommandResult",
    "CommandSpec",
    "CommandTimedOut",
    "run_command",
    "run_command_sync",
]
