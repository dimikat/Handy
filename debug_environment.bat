@echo off
echo ========================================
echo     Handy Build Environment Debugger
echo ========================================
echo.

echo Current directory: %CD%
echo.

echo Checking for project files...
if exist "package.json" (
    echo [OK] package.json found
) else (
    echo [MISSING] package.json - Not in project directory!
)

if exist "src-tauri" (
    echo [OK] src-tauri directory found
) else (
    echo [MISSING] src-tauri directory not found!
)

if exist "src" (
    echo [OK] src directory found
) else (
    echo [MISSING] src directory not found!
)

echo.
echo Checking system tools...

node --version >nul 2>&1
if errorlevel 1 (
    echo [MISSING] Node.js not installed
) else (
    echo [OK] Node.js installed - Version:
    node --version
)

npm --version >nul 2>&1
if errorlevel 1 (
    echo [MISSING] npm not installed
) else (
    echo [OK] npm installed - Version:
    npm --version
)

cargo --version >nul 2>&1
if errorlevel 1 (
    echo [MISSING] Rust/Cargo not installed
    echo          Download from: https://rustup.rs/
) else (
    echo [OK] Rust/Cargo installed - Version:
    cargo --version
)

cargo tauri --version >nul 2>&1
if errorlevel 1 (
    echo [MISSING] Tauri CLI not installed
    echo          Install with: cargo install tauri-cli
) else (
    echo [OK] Tauri CLI installed - Version:
    cargo tauri --version
)

echo.
echo ========================================
echo Environment check complete!
echo.
echo If all tools show [OK], you can run:
echo 1. validate_build_resources.bat
echo 2. build_installer.bat
echo.
echo Press any key to exit...
pause >nul