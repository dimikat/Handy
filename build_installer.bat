@echo off
echo ========================================
echo    Handy v0.3.1 Enhanced Build Script
echo ========================================
echo.

REM Show current directory for debugging
echo Current directory: %CD%
echo.

REM Ensure we're in the right directory
if not exist "package.json" (
    echo ERROR: Not in Handy project directory!
    echo Please run this script from the Handy project root folder.
    echo Expected to find package.json in current directory.
    echo.
    echo Press any key to exit...
    pause >nul
    exit /b 1
)

echo Checking prerequisites...

REM Check if Node.js is available
node --version >nul 2>&1
if errorlevel 1 (
    echo ERROR: Node.js not found. Please install Node.js first.
    echo Download from: https://nodejs.org/
    echo.
    echo Press any key to exit...
    pause >nul
    exit /b 1
)
echo [OK] Node.js found

REM Check if npm is available
npm --version >nul 2>&1
if errorlevel 1 (
    echo ERROR: npm not found. Please install npm first.
    echo.
    echo Press any key to exit...
    pause >nul
    exit /b 1
)
echo [OK] npm found

REM Check if Rust is available
cargo --version >nul 2>&1
if errorlevel 1 (
    echo ERROR: Rust/Cargo not found. Please install Rust first.
    echo Visit: https://rustup.rs/
    echo.
    echo Press any key to exit...
    pause >nul
    exit /b 1
)
echo [OK] Rust/Cargo found

REM Check if Tauri CLI is available
cargo tauri --version >nul 2>&1
if errorlevel 1 (
    echo Tauri CLI not found. Installing...
    cargo install tauri-cli --version "^2.0"
    if errorlevel 1 (
        echo ERROR: Failed to install Tauri CLI.
        echo.
        echo Press any key to exit...
        pause >nul
        exit /b 1
    )
    echo [OK] Tauri CLI installed
) else (
    echo [OK] Tauri CLI found
)

echo.
echo Prerequisites check passed!
echo.

echo Step 1: Installing Node.js dependencies...
npm install
if errorlevel 1 (
    echo ERROR: Failed to install Node.js dependencies.
    echo.
    echo Press any key to exit...
    pause >nul
    exit /b 1
)

echo.
echo Step 2: Building frontend...
npm run build
if errorlevel 1 (
    echo ERROR: Frontend build failed.
    echo Check the output above for TypeScript or build errors.
    echo.
    echo Press any key to exit...
    pause >nul
    exit /b 1
)

echo.
echo Step 3: Building Tauri application with installer...
cargo tauri build --target x86_64-pc-windows-msvc
if errorlevel 1 (
    echo ERROR: Tauri build failed.
    echo.
    echo Common solutions:
    echo - Ensure Windows build tools are installed
    echo - Check that all resource files exist in src-tauri/resources/
    echo - Verify VAD model file: src-tauri/resources/models/silero_vad_v4.onnx
    echo - Run validate_build_resources.bat to check missing files
    echo.
    echo Press any key to exit...
    pause >nul
    exit /b 1
)

echo.
echo ========================================
echo        BUILD COMPLETED SUCCESSFULLY!
echo ========================================
echo.
echo Installer created at:
echo src-tauri\target\release\bundle\nsis\Handy_0.3.1_x64-setup.exe
echo.
echo Additional files:
echo - Main executable: src-tauri\target\release\handy.exe
echo - MSI installer: src-tauri\target\release\bundle\msi\Handy_0.3.1_x64_en-US.msi
echo.
echo New features in v0.3.1:
echo - Multiple hotkey profiles (4 transcription modes)
echo - Modern tabbed settings interface
echo - Enhanced tooltips and individual shortcut toggles
echo - File logging with timestamps
echo.
echo Ready for distribution!
echo.
echo Press any key to exit...
pause >nul