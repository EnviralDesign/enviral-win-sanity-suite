@echo off
setlocal

rem Resolve the repository root regardless of invocation location
set "SCRIPT_DIR=%~dp0"
pushd "%SCRIPT_DIR%" >nul

rem Execute the entrypoint via uv and forward all CLI arguments
uv run main.py %*
set "EXIT_CODE=%ERRORLEVEL%"

popd >nul
endlocal & exit /b %EXIT_CODE%
