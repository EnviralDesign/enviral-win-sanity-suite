# Windows Sanity Suite

Windows Sanity Suite is a DearPyGui-based desktop companion that centralizes common Windows diagnostics and remediation workflows. The app is orchestrated with `uv` to keep dependency management and packaging lightweight.

## Getting Started

```powershell
uv sync
uv run python -m app
```

On first run `uv` will create a `.venv` in the project directory. The application launches a DearPyGui window containing stubs for the planned tabs (Ports, Network, Services, Hosts, Workflows, Settings).

### Useful Commands

| Command | Description |
|---------|-------------|
| `uv run app` | Launch the GUI via the `python -m app` entry point |
| `uv run python -m app` | Equivalent explicit invocation |
| `uv run python -m pytest` | Run the test suite (when available) |
| `uv build` | Produce wheel/sdist artifacts |

## Project Layout

```
app/
  core/
  tools/
  ui/
workflows/
  samples/
```

- `app.core` holds configuration, logging, command execution, and admin helpers.
- `app.tools` collects tool descriptors for ports, network, services, and hosts functionality.
- `app.ui` builds DearPyGui layout primitives and widgets.
- `workflows/samples` contains example TOML workflows, e.g. `Fix Port 3010` and `Network Reset`.

## Packaging

Use PyInstaller to produce a one-folder build:

```powershell
uv run pyinstaller app/main.py --noconfirm --onedir --name WindowsSanitySuite
```

The resulting artifacts will be located in `dist/WindowsSanitySuite`. For distribution, bundle the entire directory.

## Contributing

- Ensure commands guard against long-running hangs by setting reasonable timeouts.
- Follow the module structure and expand tool actions or UI panels as needed.
- Submit PRs with clear descriptions and include workflow samples when adding new automation sequences.
