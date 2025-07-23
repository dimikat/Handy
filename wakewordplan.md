# Wake Word Detection Implementation Plan

## Overview
Implement "Hey Handy" wake word detection as an alternative to keyboard shortcuts, leveraging the existing VAD system and audio infrastructure for seamless voice-activated transcription.

## Architecture Analysis

### Current Audio System Strengths
- **Existing VAD Pipeline**: Silero VAD + SmoothedVad already processing 30ms frames at 16kHz
- **Audio Manager**: `AudioRecordingManager` with microphone modes (AlwaysOn/OnDemand)
- **Stream Infrastructure**: CPAL-based continuous audio streaming with worker threads
- **Transcription Pipeline**: Whisper integration via `TranscriptionManager`

### Technical Approach
Use a **hybrid detection system**: VAD for speech detection + lightweight keyword spotting for wake word recognition.

## Phase 1: Wake Word Detection Engine

### 1.1 Detection Algorithm Options
**Option A: Template Matching (Recommended for MVP)**
- Record multiple "Hey Handy" samples at startup
- Use MFCC (Mel-frequency cepstral coefficients) feature extraction
- Cross-correlation matching with threshold-based detection
- Lightweight, works offline, fast response time

**Option B: Small Neural Model**
- Train/use a small keyword spotting model (e.g., tensorflow-lite)
- More accurate but requires model files and higher CPU usage
- Future enhancement after template matching proves concept

### 1.2 New Rust Components
```rust
// src-tauri/src/audio_toolkit/wake_word/
mod template_matcher;
mod feature_extractor; 
mod wake_word_detector;

pub struct WakeWordDetector {
    templates: Vec<MfccTemplate>,
    feature_extractor: MfccExtractor,
    threshold: f32,
    buffer: RingBuffer<f32>,
}

impl WakeWordDetector {
    fn process_frame(&mut self, audio: &[f32]) -> WakeWordResult;
    fn add_template(&mut self, audio: &[f32]) -> Result<()>;
    fn set_sensitivity(&mut self, threshold: f32);
}
```

### 1.3 Integration with Existing VAD
- Extend `VadFrame` enum to include `WakeWord` variant
- Chain wake word detection after VAD: `VAD → WakeWordDetector → Action`
- Only process wake word detection on `VadFrame::Speech` frames

## Phase 2: Audio Pipeline Enhancement

### 2.1 Continuous Listening Mode
```rust
pub enum ListeningMode {
    Disabled,           // No wake word, shortcuts only
    Background,         // Always listening, low power
    Foreground,         // Active listening, full power
}

pub struct WakeWordManager {
    detector: Arc<Mutex<WakeWordDetector>>,
    mode: Arc<Mutex<ListeningMode>>,
    audio_buffer: RingBuffer<f32>,
}
```

### 2.2 Audio Manager Integration
- Extend `AudioRecordingManager` with wake word capabilities
- Add wake word processing to the existing audio stream worker
- Buffer management for continuous analysis without recording full sessions

### 2.3 Power Management
- **Background Mode**: Process every 3rd frame (100ms intervals) to save CPU
- **Foreground Mode**: Process all frames (30ms) for maximum responsiveness
- **Sleep Detection**: Pause wake word when system is idle for >1 hour

## Phase 3: User Experience & Settings

### 3.1 Wake Word Setup Flow
1. **Initial Training**: Record "Hey Handy" 3-5 times during onboarding
2. **Template Quality Check**: Validate recordings meet quality thresholds
3. **Sensitivity Calibration**: Test in user's environment with background noise
4. **Fallback Options**: Always keep keyboard shortcuts as backup

### 3.2 Settings Integration
```typescript
// Add to existing settings schema
interface WakeWordSettings {
  enabled: boolean;
  sensitivity: number;        // 0.1-1.0 scale
  listening_mode: 'background' | 'foreground';
  wake_phrases: string[];     // Support multiple phrases later
  cooldown_ms: number;        // Prevent rapid re-triggering
  require_confirmation: boolean; // "Yes" after wake word
}
```

### 3.3 UI Components
- **Wake Word Tab** in settings with setup wizard
- **Recording Interface** for template capture
- **Sensitivity Slider** with real-time testing
- **Status Indicator** showing listening state in system tray
- **Audio Level Meter** during setup for optimal positioning

## Phase 4: Advanced Features

### 4.1 Multi-Modal Activation
```rust
enum ActivationMethod {
    WakeWord { phrase: String },
    Keyboard { shortcut: String },
    Combined { wake_word: String, then_action: KeyAction },
}
```

### 4.2 Context-Aware Behavior
- **Follow-up Commands**: "Hey Handy, notepad mode" → directly opens notepad
- **Command Chaining**: "Hey Handy, transcribe and save to file"
- **Smart Timeouts**: Different modes have different active listening durations

### 4.3 Privacy & Security
- **Local Processing**: All wake word detection happens on-device
- **Template Encryption**: Store voice templates encrypted in app data
- **Mic Privacy**: Visual/audio indicators when wake word is actively listening
- **Disable Options**: Easy global disable for privacy-sensitive environments

## Phase 5: Performance Optimization

### 5.1 CPU Efficiency
- **SIMD Optimization**: Use platform-specific vector instructions for MFCC
- **Frame Skipping**: Dynamic frame processing based on system load
- **Thread Priorities**: Lower priority for background wake word processing

### 5.2 Memory Management
- **Circular Buffers**: Fixed-size audio buffers to prevent memory growth
- **Template Compression**: Compress stored voice templates
- **Lazy Loading**: Load wake word engine only when enabled

### 5.3 Accuracy Improvements
- **Noise Adaptation**: Adjust sensitivity based on background noise levels
- **Speaker Adaptation**: Improve templates over time with successful detections
- **False Positive Reduction**: Implement speaker verification for multi-user systems

## Implementation Priority

### Phase 1 (Core MVP - 1-2 weeks)
1. MFCC feature extraction implementation
2. Template matching algorithm
3. Basic wake word detector integration
4. Simple "Hey Handy" detection

### Phase 2 (Integration - 1 week)
1. Audio pipeline integration
2. Continuous listening mode
3. Basic settings UI

### Phase 3 (Polish - 1 week)
1. Setup wizard and training flow
2. Sensitivity calibration
3. Status indicators and feedback

### Phase 4 (Advanced - Future)
1. Multi-phrase support
2. Context-aware commands
3. Performance optimizations

## Technical Challenges & Solutions

### Challenge 1: False Positives
**Solution**: Multi-stage verification (VAD + template matching + optional confirmation)

### Challenge 2: CPU Usage
**Solution**: Adaptive processing rates and efficient algorithms

### Challenge 3: Different Accents/Voices
**Solution**: Multiple template storage and similarity scoring

### Challenge 4: Background Noise
**Solution**: Noise-adaptive thresholding and robust feature extraction

This implementation leverages Handy's existing robust audio infrastructure while adding wake word capability that feels natural and responsive. The phased approach allows for iterative improvement while delivering immediate value to users who want hands-free operation.