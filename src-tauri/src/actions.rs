use crate::managers::audio::AudioRecordingManager;
use crate::managers::transcription::TranscriptionManager;
use crate::settings::get_settings;
use crate::utils;
use crate::utils::change_tray_icon;
use crate::utils::append_to_file;
use crate::utils::focus_notepad_window;
use crate::utils::launch_notepad;
use crate::utils::play_recording_start_sound;
use crate::utils::play_recording_stop_sound;
use crate::utils::TrayIconState;
use log::debug;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tauri::AppHandle;
use tauri::Manager;

// Shortcut Action Trait
pub trait ShortcutAction: Send + Sync {
    fn start(&self, app: &AppHandle, binding_id: &str, shortcut_str: &str);
    fn stop(&self, app: &AppHandle, binding_id: &str, shortcut_str: &str);
}

// Transcribe Action
struct TranscribeAction;

impl ShortcutAction for TranscribeAction {
    fn start(&self, app: &AppHandle, binding_id: &str, _shortcut_str: &str) {
        let start_time = Instant::now();
        debug!("TranscribeAction::start called for binding: {}", binding_id);

        let binding_id = binding_id.to_string();
        change_tray_icon(app, TrayIconState::Recording);

        let rm = app.state::<Arc<AudioRecordingManager>>();

        // Get the microphone mode to determine audio feedback timing
        let settings = get_settings(app);
        let is_always_on = settings.always_on_microphone;
        debug!("Microphone mode - always_on: {}", is_always_on);

        if is_always_on {
            // Always-on mode: Play audio feedback immediately
            debug!("Always-on mode: Playing audio feedback immediately");
            play_recording_start_sound(app);
            let recording_started = rm.try_start_recording(&binding_id);
            debug!("Recording started: {}", recording_started);
        } else {
            // On-demand mode: Start recording first, then play audio feedback
            // This allows the microphone to be activated before playing the sound
            debug!("On-demand mode: Starting recording first, then audio feedback");
            let recording_start_time = Instant::now();
            if rm.try_start_recording(&binding_id) {
                debug!("Recording started in {:?}", recording_start_time.elapsed());
                // Small delay to ensure microphone stream is active
                let app_clone = app.clone();
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    debug!("Playing delayed audio feedback");
                    play_recording_start_sound(&app_clone);
                });
            } else {
                debug!("Failed to start recording");
            }
        }

        debug!(
            "TranscribeAction::start completed in {:?}",
            start_time.elapsed()
        );
    }

    fn stop(&self, app: &AppHandle, binding_id: &str, _shortcut_str: &str) {
        let stop_time = Instant::now();
        debug!("TranscribeAction::stop called for binding: {}", binding_id);

        let ah = app.clone();
        let rm = Arc::clone(&app.state::<Arc<AudioRecordingManager>>());
        let tm = Arc::clone(&app.state::<Arc<TranscriptionManager>>());

        change_tray_icon(app, TrayIconState::Idle);

        // Play audio feedback for recording stop
        play_recording_stop_sound(app);

        let binding_id = binding_id.to_string(); // Clone binding_id for the async task

        tauri::async_runtime::spawn(async move {
            let binding_id = binding_id.clone(); // Clone for the inner async task
            debug!(
                "Starting async transcription task for binding: {}",
                binding_id
            );

            let stop_recording_time = Instant::now();
            if let Some(samples) = rm.stop_recording(&binding_id) {
                debug!(
                    "Recording stopped and samples retrieved in {:?}, sample count: {}",
                    stop_recording_time.elapsed(),
                    samples.len()
                );

                let transcription_time = Instant::now();
                match tm.transcribe(samples) {
                    Ok(transcription) => {
                        debug!(
                            "Transcription completed in {:?}: '{}'",
                            transcription_time.elapsed(),
                            transcription
                        );
                        if !transcription.is_empty() {
                            let transcription_clone = transcription.clone();
                            let ah_clone = ah.clone();
                            let paste_time = Instant::now();
                            ah.run_on_main_thread(move || {
                                match utils::paste(transcription_clone, ah_clone) {
                                    Ok(()) => debug!(
                                        "Text pasted successfully in {:?}",
                                        paste_time.elapsed()
                                    ),
                                    Err(e) => eprintln!("Failed to paste transcription: {}", e),
                                }
                            })
                            .unwrap_or_else(|e| {
                                eprintln!("Failed to run paste on main thread: {:?}", e);
                            });
                        }
                    }
                    Err(err) => debug!("Global Shortcut Transcription error: {}", err),
                }
            } else {
                debug!("No samples retrieved from recording stop");
            }
        });

        debug!(
            "TranscribeAction::stop completed in {:?}",
            stop_time.elapsed()
        );
    }
}

// Notepad Transcribe Action
struct NotepadTranscribeAction;

impl ShortcutAction for NotepadTranscribeAction {
    fn start(&self, app: &AppHandle, binding_id: &str, _shortcut_str: &str) {
        let start_time = Instant::now();
        debug!("NotepadTranscribeAction::start called for binding: {}", binding_id);

        let binding_id = binding_id.to_string();
        change_tray_icon(app, TrayIconState::Recording);

        // Launch notepad first
        match launch_notepad() {
            Ok(process_id) => {
                debug!("Notepad launched successfully with PID: {}", process_id);
                
                // Focus the notepad window
                if let Err(e) = focus_notepad_window() {
                    debug!("Warning: Failed to focus notepad window: {}", e);
                    // Continue anyway - user can manually focus if needed
                }
            }
            Err(e) => {
                debug!("Failed to launch notepad: {}", e);
                // Reset tray icon and exit early
                change_tray_icon(app, TrayIconState::Idle);
                return;
            }
        }

        let rm = app.state::<Arc<AudioRecordingManager>>();

        // Get the microphone mode to determine audio feedback timing
        let settings = get_settings(app);
        let is_always_on = settings.always_on_microphone;
        debug!("Microphone mode - always_on: {}", is_always_on);

        if is_always_on {
            // Always-on mode: Play audio feedback immediately
            debug!("Always-on mode: Playing audio feedback immediately");
            play_recording_start_sound(app);
            let recording_started = rm.try_start_recording(&binding_id);
            debug!("Recording started: {}", recording_started);
        } else {
            // On-demand mode: Start recording first, then play audio feedback
            debug!("On-demand mode: Starting recording first, then audio feedback");
            let recording_start_time = Instant::now();
            if rm.try_start_recording(&binding_id) {
                debug!("Recording started in {:?}", recording_start_time.elapsed());
                // Small delay to ensure microphone stream is active
                let app_clone = app.clone();
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    debug!("Playing delayed audio feedback");
                    play_recording_start_sound(&app_clone);
                });
            } else {
                debug!("Failed to start recording");
                // Reset tray icon if recording failed
                change_tray_icon(app, TrayIconState::Idle);
                return;
            }
        }

        debug!(
            "NotepadTranscribeAction::start completed in {:?}",
            start_time.elapsed()
        );
    }

    fn stop(&self, app: &AppHandle, binding_id: &str, _shortcut_str: &str) {
        let stop_time = Instant::now();
        debug!("NotepadTranscribeAction::stop called for binding: {}", binding_id);

        let ah = app.clone();
        let rm = Arc::clone(&app.state::<Arc<AudioRecordingManager>>());
        let tm = Arc::clone(&app.state::<Arc<TranscriptionManager>>());

        change_tray_icon(app, TrayIconState::Idle);

        // Play audio feedback for recording stop
        play_recording_stop_sound(app);

        let binding_id = binding_id.to_string();

        tauri::async_runtime::spawn(async move {
            let binding_id = binding_id.clone();
            debug!(
                "Starting async transcription task for binding: {}",
                binding_id
            );

            let stop_recording_time = Instant::now();
            if let Some(samples) = rm.stop_recording(&binding_id) {
                debug!(
                    "Recording stopped and samples retrieved in {:?}, sample count: {}",
                    stop_recording_time.elapsed(),
                    samples.len()
                );

                let transcription_time = Instant::now();
                match tm.transcribe(samples) {
                    Ok(transcription) => {
                        debug!(
                            "Transcription completed in {:?}: '{}'",
                            transcription_time.elapsed(),
                            transcription
                        );
                        if !transcription.is_empty() {
                            let transcription_clone = transcription.clone();
                            let ah_clone = ah.clone();
                            let paste_time = Instant::now();
                            ah.run_on_main_thread(move || {
                                // Focus notepad window before pasting
                                if let Err(e) = focus_notepad_window() {
                                    debug!("Warning: Failed to focus notepad before pasting: {}", e);
                                }
                                
                                // Small delay to ensure focus takes effect
                                std::thread::sleep(std::time::Duration::from_millis(100));
                                
                                match utils::paste(transcription_clone, ah_clone) {
                                    Ok(()) => debug!(
                                        "Text pasted successfully to notepad in {:?}",
                                        paste_time.elapsed()
                                    ),
                                    Err(e) => eprintln!("Failed to paste transcription to notepad: {}", e),
                                }
                            })
                            .unwrap_or_else(|e| {
                                eprintln!("Failed to run paste on main thread: {:?}", e);
                            });
                        }
                    }
                    Err(err) => debug!("Notepad Transcription error: {}", err),
                }
            } else {
                debug!("No samples retrieved from recording stop");
            }
        });

        debug!(
            "NotepadTranscribeAction::stop completed in {:?}",
            stop_time.elapsed()
        );
    }
}

// Clipboard Only Action
struct ClipboardOnlyAction;

impl ShortcutAction for ClipboardOnlyAction {
    fn start(&self, app: &AppHandle, binding_id: &str, _shortcut_str: &str) {
        let start_time = Instant::now();
        debug!("ClipboardOnlyAction::start called for binding: {}", binding_id);

        let binding_id = binding_id.to_string();
        change_tray_icon(app, TrayIconState::Recording);

        let rm = app.state::<Arc<AudioRecordingManager>>();

        // Get the microphone mode to determine audio feedback timing
        let settings = get_settings(app);
        let is_always_on = settings.always_on_microphone;
        debug!("Microphone mode - always_on: {}", is_always_on);

        if is_always_on {
            // Always-on mode: Play audio feedback immediately
            debug!("Always-on mode: Playing audio feedback immediately");
            play_recording_start_sound(app);
            let recording_started = rm.try_start_recording(&binding_id);
            debug!("Recording started: {}", recording_started);
        } else {
            // On-demand mode: Start recording first, then play audio feedback
            debug!("On-demand mode: Starting recording first, then audio feedback");
            let recording_start_time = Instant::now();
            if rm.try_start_recording(&binding_id) {
                debug!("Recording started in {:?}", recording_start_time.elapsed());
                // Small delay to ensure microphone stream is active
                let app_clone = app.clone();
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    debug!("Playing delayed audio feedback");
                    play_recording_start_sound(&app_clone);
                });
            } else {
                debug!("Failed to start recording");
            }
        }

        debug!(
            "ClipboardOnlyAction::start completed in {:?}",
            start_time.elapsed()
        );
    }

    fn stop(&self, app: &AppHandle, binding_id: &str, _shortcut_str: &str) {
        let stop_time = Instant::now();
        debug!("ClipboardOnlyAction::stop called for binding: {}", binding_id);

        let ah = app.clone();
        let rm = Arc::clone(&app.state::<Arc<AudioRecordingManager>>());
        let tm = Arc::clone(&app.state::<Arc<TranscriptionManager>>());

        change_tray_icon(app, TrayIconState::Idle);

        // Play audio feedback for recording stop
        play_recording_stop_sound(app);

        let binding_id = binding_id.to_string();

        tauri::async_runtime::spawn(async move {
            let binding_id = binding_id.clone();
            debug!(
                "Starting async transcription task for binding: {}",
                binding_id
            );

            let stop_recording_time = Instant::now();
            if let Some(samples) = rm.stop_recording(&binding_id) {
                debug!(
                    "Recording stopped and samples retrieved in {:?}, sample count: {}",
                    stop_recording_time.elapsed(),
                    samples.len()
                );

                let transcription_time = Instant::now();
                match tm.transcribe(samples) {
                    Ok(transcription) => {
                        debug!(
                            "Transcription completed in {:?}: '{}'",
                            transcription_time.elapsed(),
                            transcription
                        );
                        if !transcription.is_empty() {
                            let transcription_clone = transcription.clone();
                            let ah_clone = ah.clone();
                            let clipboard_time = Instant::now();
                            ah.run_on_main_thread(move || {
                                // Copy to clipboard only - no pasting
                                let clipboard = ah_clone.clipboard();
                                match clipboard.write_text(&transcription_clone) {
                                    Ok(()) => debug!(
                                        "Text copied to clipboard successfully in {:?}",
                                        clipboard_time.elapsed()
                                    ),
                                    Err(e) => eprintln!("Failed to copy transcription to clipboard: {}", e),
                                }
                            })
                            .unwrap_or_else(|e| {
                                eprintln!("Failed to run clipboard copy on main thread: {:?}", e);
                            });
                        }
                    }
                    Err(err) => debug!("Clipboard Transcription error: {}", err),
                }
            } else {
                debug!("No samples retrieved from recording stop");
            }
        });

        debug!(
            "ClipboardOnlyAction::stop completed in {:?}",
            stop_time.elapsed()
        );
    }
}

// Append to File Action
struct AppendToFileAction;

impl ShortcutAction for AppendToFileAction {
    fn start(&self, app: &AppHandle, binding_id: &str, _shortcut_str: &str) {
        let start_time = Instant::now();
        debug!("AppendToFileAction::start called for binding: {}", binding_id);

        let binding_id = binding_id.to_string();
        change_tray_icon(app, TrayIconState::Recording);

        let rm = app.state::<Arc<AudioRecordingManager>>();

        // Get the microphone mode to determine audio feedback timing
        let settings = get_settings(app);
        let is_always_on = settings.always_on_microphone;
        debug!("Microphone mode - always_on: {}", is_always_on);

        if is_always_on {
            // Always-on mode: Play audio feedback immediately
            debug!("Always-on mode: Playing audio feedback immediately");
            play_recording_start_sound(app);
            let recording_started = rm.try_start_recording(&binding_id);
            debug!("Recording started: {}", recording_started);
        } else {
            // On-demand mode: Start recording first, then play audio feedback
            debug!("On-demand mode: Starting recording first, then audio feedback");
            let recording_start_time = Instant::now();
            if rm.try_start_recording(&binding_id) {
                debug!("Recording started in {:?}", recording_start_time.elapsed());
                // Small delay to ensure microphone stream is active
                let app_clone = app.clone();
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    debug!("Playing delayed audio feedback");
                    play_recording_start_sound(&app_clone);
                });
            } else {
                debug!("Failed to start recording");
            }
        }

        debug!(
            "AppendToFileAction::start completed in {:?}",
            start_time.elapsed()
        );
    }

    fn stop(&self, app: &AppHandle, binding_id: &str, _shortcut_str: &str) {
        let stop_time = Instant::now();
        debug!("AppendToFileAction::stop called for binding: {}", binding_id);

        let ah = app.clone();
        let rm = Arc::clone(&app.state::<Arc<AudioRecordingManager>>());
        let tm = Arc::clone(&app.state::<Arc<TranscriptionManager>>());

        change_tray_icon(app, TrayIconState::Idle);

        // Play audio feedback for recording stop
        play_recording_stop_sound(app);

        let binding_id = binding_id.to_string();

        tauri::async_runtime::spawn(async move {
            let binding_id = binding_id.clone();
            debug!(
                "Starting async transcription task for binding: {}",
                binding_id
            );

            let stop_recording_time = Instant::now();
            if let Some(samples) = rm.stop_recording(&binding_id) {
                debug!(
                    "Recording stopped and samples retrieved in {:?}, sample count: {}",
                    stop_recording_time.elapsed(),
                    samples.len()
                );

                let transcription_time = Instant::now();
                match tm.transcribe(samples) {
                    Ok(transcription) => {
                        debug!(
                            "Transcription completed in {:?}: '{}'",
                            transcription_time.elapsed(),
                            transcription
                        );
                        if !transcription.is_empty() {
                            let transcription_clone = transcription.clone();
                            let ah_clone = ah.clone();
                            let file_time = Instant::now();
                            ah.run_on_main_thread(move || {
                                // Get settings to get the file path
                                let settings = get_settings(&ah_clone);
                                match append_to_file(&transcription_clone, &settings.append_file_path) {
                                    Ok(()) => debug!(
                                        "Text appended to file successfully in {:?}",
                                        file_time.elapsed()
                                    ),
                                    Err(e) => eprintln!("Failed to append transcription to file: {}", e),
                                }
                            })
                            .unwrap_or_else(|e| {
                                eprintln!("Failed to run file append on main thread: {:?}", e);
                            });
                        }
                    }
                    Err(err) => debug!("File Append Transcription error: {}", err),
                }
            } else {
                debug!("No samples retrieved from recording stop");
            }
        });

        debug!(
            "AppendToFileAction::stop completed in {:?}",
            stop_time.elapsed()
        );
    }
}

// Test Action
struct TestAction;

impl ShortcutAction for TestAction {
    fn start(&self, app: &AppHandle, binding_id: &str, shortcut_str: &str) {
        println!(
            "Shortcut ID '{}': Started - {} (App: {})", // Changed "Pressed" to "Started" for consistency
            binding_id,
            shortcut_str,
            app.package_info().name
        );
    }

    fn stop(&self, app: &AppHandle, binding_id: &str, shortcut_str: &str) {
        println!(
            "Shortcut ID '{}': Stopped - {} (App: {})", // Changed "Released" to "Stopped" for consistency
            binding_id,
            shortcut_str,
            app.package_info().name
        );
    }
}

// Static Action Map  
pub static ACTION_MAP: Lazy<HashMap<String, Arc<dyn ShortcutAction>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        "transcribe".to_string(),
        Arc::new(TranscribeAction) as Arc<dyn ShortcutAction>,
    );
    map.insert(
        "notepad_transcribe".to_string(),
        Arc::new(NotepadTranscribeAction) as Arc<dyn ShortcutAction>,
    );
    map.insert(
        "clipboard_only".to_string(),
        Arc::new(ClipboardOnlyAction) as Arc<dyn ShortcutAction>,
    );
    map.insert(
        "append_to_file".to_string(),
        Arc::new(AppendToFileAction) as Arc<dyn ShortcutAction>,
    );
    map.insert(
        "test".to_string(),
        Arc::new(TestAction) as Arc<dyn ShortcutAction>,
    );
    map
});
