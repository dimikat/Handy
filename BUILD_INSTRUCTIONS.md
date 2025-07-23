# Handy v0.3.1 Enhanced Build Instructions

## Overview
This document provides comprehensive instructions for building the Handy v0.3.1 Enhanced installer with all the new features including multiple hotkey profiles and the modern tabbed settings interface.

## Prerequisites

### Required Tools
1. **Rust Toolchain** (Latest Stable)
   ```bash
   # Install Rust via rustup
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Node.js and npm** (Version 18+)
   ```bash
   # Verify installation
   node --version
   npm --version
   ```

3. **Tauri CLI**
   ```bash
   # Install Tauri CLI
   cargo install tauri-cli --version "^2.0"
   ```

### Windows-Specific Requirements
- **Microsoft Visual Studio Build Tools** or **Visual Studio Community**
- **Windows SDK 10** or later
- **MSVC v143 toolset**

## Build Process

### Step 1: Prepare Environment
```bash
# Navigate to project directory
cd /path/to/handy

# Install Node.js dependencies
npm install

# Verify Tauri installation
cargo tauri --version
```

### Step 2: Frontend Build
```bash
# Build the React frontend
npm run build
```
This should generate the `dist/` directory with compiled frontend assets.

### Step 3: Rust Backend Build
```bash
# Navigate to Rust directory
cd src-tauri

# Install Rust dependencies
cargo build --release
```

### Step 4: Full Application Build
```bash
# Return to project root
cd ..

# Build complete Tauri application with installer
cargo tauri build --target x86_64-pc-windows-msvc
```

## Expected Build Artifacts

After successful build, you should find:

### Executable Files
- `src-tauri/target/release/handy.exe` - Main application executable
- `src-tauri/target/release/bundle/nsis/Handy_0.3.1_x64-setup.exe` - **Primary Windows Installer**
- `src-tauri/target/release/bundle/msi/Handy_0.3.1_x64_en-US.msi` - MSI installer (alternative)

### The installer file `Handy_0.3.1_x64-setup.exe` is your target output.

## New Features in v0.3.1

The built installer will include all these enhanced features:

### 🎯 Multiple Hotkey Profiles
- **Quick Transcribe** (`Ctrl+Space`): Original paste-to-active-window
- **Notepad Mode** (`Ctrl+Shift+Space`): Auto-opens notepad and transcribes
- **Clipboard Only** (`Ctrl+Alt+Space`): Transcribes to clipboard without pasting  
- **Append to File** (`Ctrl+Shift+Alt+Space`): Logs to timestamped file

### 🎨 Modern Tabbed Interface
- **Shortcuts Tab**: Configure all transcription modes with individual toggles
- **Audio Tab**: Microphone and audio device settings
- **Models Tab**: Future model management (placeholder)
- **Advanced Tab**: Language and other advanced settings

### 💡 Enhanced User Experience
- Rich tooltips explaining each feature
- Individual enable/disable toggles for shortcuts
- Professional UI with smooth animations
- File picker for log file locations

## Troubleshooting

### Common Build Issues

#### 1. Missing Dependencies Error
```
error: failed to run custom build command for `handy`
```
**Solution**: Ensure all Cargo.toml dependencies are available:
```bash
cargo clean
cargo build --release
```

#### 2. Frontend Build Failures
```
error TS2724: has no exported member
```
**Solution**: The TypeScript errors have been fixed in the current version. If issues persist:
```bash
npm run build -- --force
```

#### 3. Tauri Build Errors
```
Error failed to bundle project
```
**Solution**: 
- Verify Windows build tools are installed
- Check that all resource files exist in `src-tauri/resources/`
- Ensure VAD model file is present: `src-tauri/resources/models/silero_vad_v4.onnx`

#### 4. Code Signing Issues (Optional)
The current configuration includes Azure Code Signing. For development builds:
- Remove or comment out the `signCommand` in `src-tauri/tauri.conf.json`
- Or ensure signing certificates are properly configured

### Resource Requirements
Make sure these files exist in `src-tauri/resources/`:
- ✅ `models/silero_vad_v4.onnx` - VAD model (required)
- ✅ `rec_start.wav` - Audio feedback
- ✅ `rec_stop.wav` - Audio feedback  
- ✅ `tray_idle.png` - Tray icon
- ✅ `tray_recording.png` - Tray icon
- ✅ `default_settings.json` - Default configuration

## Installation Testing

After building, test the installer:

1. **Install on Clean System**: Test `Handy_0.3.1_x64-setup.exe` on a system without Handy
2. **Feature Verification**: Test all four shortcut modes work correctly
3. **Settings Interface**: Verify tabbed interface loads properly
4. **Permissions**: Ensure microphone and accessibility permissions work
5. **Upgrade Testing**: Install over existing Handy installation

## Distribution

### Installer Naming Convention
- **Recommended**: `Handy_v0.3.1_Enhanced_x64-setup.exe`
- **Alternative**: `Handy_0.3.1_x64-setup.exe` (Tauri default)

### Package Contents
Create a distribution package with:
- Main installer executable
- This BUILD_INSTRUCTIONS.md file
- CHANGELOG.md highlighting new features
- README.md with usage instructions

## Success Indicators

Your build is successful when:
- ✅ Frontend builds without TypeScript errors
- ✅ Rust backend compiles successfully  
- ✅ Tauri generates Windows installer (.exe)
- ✅ Installer is approximately 50-100MB (includes Whisper dependencies)
- ✅ All four shortcut modes work in built version
- ✅ Tabbed settings interface loads correctly

## Support

If you encounter build issues:
1. Check that all prerequisites are installed
2. Verify all resource files are present
3. Review error messages for missing dependencies
4. Ensure Windows build tools are properly configured

The enhanced Handy v0.3.1 provides a significantly improved user experience with professional-grade features while maintaining the privacy-focused, offline transcription that makes Handy unique.