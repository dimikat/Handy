# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Handy is a cross-platform desktop speech-to-text application built with Tauri (Rust backend + React/TypeScript frontend). It provides completely offline transcription using Whisper models, with privacy-focused design where voice data never leaves the user's computer.

## Key Architecture Components

### Tauri Hybrid Architecture
- **Frontend**: React + TypeScript with Tailwind CSS for settings UI
- **Backend**: Rust for system integration, audio processing, and ML inference
- **Communication**: Tauri commands for frontend-backend communication
- **State Management**: Tauri's managed state for cross-component data sharing

### Core Rust Modules (src-tauri/src/)
- `audio_toolkit/`: Self-contained audio processing toolkit with VAD, recording, resampling
- `managers/`: High-level business logic managers (audio, models, transcription)
- `commands/`: Tauri command handlers for frontend communication
- `shortcut.rs`: Global keyboard shortcut handling with platform-specific implementations
- `settings.rs`: Persistent settings management using Tauri's store plugin

### Frontend Structure (src/)
- `components/`: Organized by feature (settings, model-selector, onboarding, etc.)
- `hooks/`: Custom hooks for Tauri state management (useSettings, useModels)
- `lib/types.ts`: Zod schemas and TypeScript types for type-safe communication

## Development Commands

### Basic Development
```bash
# Start development server
bun run tauri dev

# Build for production
bun run tauri build

# Frontend only (for UI development)
bun run dev
```

### Model Setup for Development
Models are downloaded dynamically by users, but for development you need the VAD model:
```bash
mkdir -p src-tauri/resources/models
curl -o src-tauri/resources/models/silero_vad_v4.onnx https://blob.handy.computer/silero_vad_v4.onnx
```

### Rust Development
```bash
# Run Rust tests
cd src-tauri && cargo test

# Check Rust code
cd src-tauri && cargo check

# Format Rust code
cd src-tauri && cargo fmt
```

## Key Technical Patterns

### Manager Pattern
The Rust backend uses a manager pattern for complex stateful operations:
- `AudioRecordingManager`: Handles microphone access, recording, VAD filtering
- `ModelManager`: Manages Whisper model downloading, loading, and switching
- `TranscriptionManager`: Coordinates audio processing with model inference

### Tauri Command Pattern
Frontend communicates with backend via strongly-typed Tauri commands:
```rust
#[tauri::command]
fn command_name(param: Type) -> Result<ReturnType, String>
```

### Settings Architecture
Settings use Tauri's store plugin with Zod validation:
- Backend: Settings stored/loaded via `tauri_plugin_store`
- Frontend: Type-safe access via custom hooks with Zod schemas
- Persistence: Automatic save/load with change detection

### Cross-Platform Considerations
- Platform-specific Whisper features (Metal on macOS, Vulkan on Windows/Linux)
- Different keyboard shortcut handling per OS
- macOS-specific permission requests for accessibility and microphone

## Important Development Notes

### Audio Pipeline
Audio flows through: Microphone → VAD filtering → Whisper transcription → Text output
- CPAL for cross-platform audio I/O
- Silero VAD for voice activity detection
- Rubato for audio resampling to 16kHz mono (Whisper requirement)

### Model Management
- No models bundled with app (user downloads on demand)
- Supports Small, Medium, Turbo, and Large Whisper models
- Dynamic switching without app restart
- Progress tracking for downloads with resume capability

### Global Shortcuts
- Uses `rdev` for cross-platform keyboard hook
- Supports both toggle and push-to-talk modes
- Platform-specific key code handling and permission requirements

### Build Configuration
- Tauri bundle includes `resources/` directory
- Models stored in app data directory (not bundle)
- Platform-specific signing and distribution settings in `tauri.conf.json`

## Common Tasks

### Adding New Settings
1. Update Zod schema in `src/lib/types.ts`
2. Add Tauri command in `src-tauri/src/commands/` 
3. Update settings manager in `src-tauri/src/settings.rs`
4. Create UI component in `src/components/settings/`

### Adding New Tauri Commands
1. Define command in appropriate `src-tauri/src/commands/` module
2. Add to `invoke_handler!` macro in `src-tauri/src/lib.rs`
3. Call from frontend using `invoke()` from `@tauri-apps/api/core`

### Model Integration
All model operations go through `ModelManager` - never directly access model files or whisper-rs from other modules.