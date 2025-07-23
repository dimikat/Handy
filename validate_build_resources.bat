@echo off
echo ========================================
echo   Handy v0.3.1 Build Resource Validator
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
    pause
    exit /b 1
)

set "RESOURCES_DIR=src-tauri\resources"
set "ALL_GOOD=1"

echo Checking required resource files...
echo.

REM Check VAD model (critical)
if exist "%RESOURCES_DIR%\models\silero_vad_v4.onnx" (
    echo [OK] VAD Model: %RESOURCES_DIR%\models\silero_vad_v4.onnx
) else (
    echo [MISSING] VAD Model: %RESOURCES_DIR%\models\silero_vad_v4.onnx
    echo          This file is REQUIRED for voice activity detection
    echo          Download from: https://blob.handy.computer/silero_vad_v4.onnx
    set "ALL_GOOD=0"
)

REM Check audio feedback files
if exist "%RESOURCES_DIR%\rec_start.wav" (
    echo [OK] Audio Start: %RESOURCES_DIR%\rec_start.wav
) else (
    echo [MISSING] Audio Start: %RESOURCES_DIR%\rec_start.wav
    set "ALL_GOOD=0"
)

if exist "%RESOURCES_DIR%\rec_stop.wav" (
    echo [OK] Audio Stop: %RESOURCES_DIR%\rec_stop.wav
) else (
    echo [MISSING] Audio Stop: %RESOURCES_DIR%\rec_stop.wav
    set "ALL_GOOD=0"
)

REM Check tray icons
if exist "%RESOURCES_DIR%\tray_idle.png" (
    echo [OK] Tray Icon (Idle): %RESOURCES_DIR%\tray_idle.png
) else (
    echo [MISSING] Tray Icon (Idle): %RESOURCES_DIR%\tray_idle.png
    set "ALL_GOOD=0"
)

if exist "%RESOURCES_DIR%\tray_recording.png" (
    echo [OK] Tray Icon (Recording): %RESOURCES_DIR%\tray_recording.png
) else (
    echo [MISSING] Tray Icon (Recording): %RESOURCES_DIR%\tray_recording.png
    set "ALL_GOOD=0"
)

REM Check default settings
if exist "%RESOURCES_DIR%\default_settings.json" (
    echo [OK] Default Settings: %RESOURCES_DIR%\default_settings.json
) else (
    echo [MISSING] Default Settings: %RESOURCES_DIR%\default_settings.json
    set "ALL_GOOD=0"
)

REM Check app icons
if exist "src-tauri\icons\icon.ico" (
    echo [OK] App Icon: src-tauri\icons\icon.ico
) else (
    echo [MISSING] App Icon: src-tauri\icons\icon.ico
    set "ALL_GOOD=0"
)

echo.
echo Frontend build check...
if exist "dist\index.html" (
    echo [OK] Frontend Built: dist\index.html exists
) else (
    echo [MISSING] Frontend not built. Run: npm run build
    set "ALL_GOOD=0"
)

echo.
echo ========================================

if "%ALL_GOOD%"=="1" (
    echo         ALL RESOURCES PRESENT!
    echo    Ready to build Handy installer
    echo.
    echo Run: build_installer.bat
) else (
    echo      MISSING RESOURCES DETECTED!
    echo Please resolve the missing files above
    echo before attempting to build the installer.
)

echo ========================================
echo.
echo Press any key to continue...
pause >nul