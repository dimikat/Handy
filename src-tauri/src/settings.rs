use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::{App, AppHandle};
use tauri_plugin_store::StoreExt;
use crate::utils::get_default_log_path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShortcutBinding {
    pub id: String,
    pub name: String,
    pub description: String,
    pub default_binding: String,
    pub current_binding: String,
    pub action_type: String,
}

/* still handy for composing the initial JSON in the store ------------- */
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppSettings {
    pub bindings: HashMap<String, ShortcutBinding>,
    pub push_to_talk: bool,
    pub audio_feedback: bool,
    #[serde(default = "default_model")]
    pub selected_model: String,
    #[serde(default = "default_always_on_microphone")]
    pub always_on_microphone: bool,
    #[serde(default)]
    pub selected_microphone: Option<String>,
    #[serde(default)]
    pub selected_output_device: Option<String>,
    #[serde(default = "default_translate_to_english")]
    pub translate_to_english: bool,
    #[serde(default = "default_shortcuts_enabled")]
    pub shortcuts_enabled: HashMap<String, bool>,
    #[serde(default = "default_append_file_path")]
    pub append_file_path: String,
    #[serde(default = "default_show_notifications")]
    pub show_notifications: bool,
}

fn default_model() -> String {
    // Default to empty string if no models are available yet
    // The UI will handle prompting for model download
    "".to_string()
}

fn default_always_on_microphone() -> bool {
    // Default to false for better user experience
    // True would be the old behavior (always-on for low latency)
    false
}

fn default_translate_to_english() -> bool {
    // Default to false - users need to opt-in to translation
    false
}

fn default_shortcuts_enabled() -> HashMap<String, bool> {
    let mut enabled = HashMap::new();
    enabled.insert("transcribe".to_string(), true);
    enabled.insert("notepad_transcribe".to_string(), true);
    enabled.insert("clipboard_only".to_string(), true);
    enabled.insert("append_to_file".to_string(), true);
    enabled
}

fn default_append_file_path() -> String {
    // Use the utility function to get the default path, fallback to a safe default
    get_default_log_path().unwrap_or_else(|_| "transcriptions.txt".to_string())
}

fn default_show_notifications() -> bool {
    true
}

pub const SETTINGS_STORE_PATH: &str = "settings_store.json";

pub fn get_default_settings() -> AppSettings {
    // Set platform-specific default keyboard shortcuts
    #[cfg(target_os = "windows")]
    let default_shortcut = "ctrl+space";
    #[cfg(target_os = "macos")]
    let default_shortcut = "alt+space"; // Alt key on macOS (Option key)
    #[cfg(target_os = "linux")]
    let default_shortcut = "ctrl+space";
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    let default_shortcut = "alt+space"; // Fallback for other platforms

    let mut bindings = HashMap::new();
    bindings.insert(
        "transcribe".to_string(),
        ShortcutBinding {
            id: "transcribe".to_string(),
            name: "Transcribe".to_string(),
            description: "Converts your speech into text.".to_string(),
            default_binding: default_shortcut.to_string(),
            current_binding: default_shortcut.to_string(),
            action_type: "transcribe".to_string(),
        },
    );

    // Set platform-specific default keyboard shortcuts for notepad mode
    #[cfg(target_os = "windows")]
    let notepad_default_shortcut = "ctrl+shift+space";
    #[cfg(target_os = "macos")]
    let notepad_default_shortcut = "alt+shift+space";
    #[cfg(target_os = "linux")]
    let notepad_default_shortcut = "ctrl+shift+space";
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    let notepad_default_shortcut = "alt+shift+space";

    bindings.insert(
        "notepad_transcribe".to_string(),
        ShortcutBinding {
            id: "notepad_transcribe".to_string(),
            name: "Notepad Transcribe".to_string(),
            description: "Opens notepad and transcribes speech into it.".to_string(),
            default_binding: notepad_default_shortcut.to_string(),
            current_binding: notepad_default_shortcut.to_string(),
            action_type: "notepad_transcribe".to_string(),
        },
    );

    // Set platform-specific default keyboard shortcuts for clipboard-only mode
    #[cfg(target_os = "windows")]
    let clipboard_default_shortcut = "ctrl+alt+space";
    #[cfg(target_os = "macos")]
    let clipboard_default_shortcut = "alt+cmd+space";
    #[cfg(target_os = "linux")]
    let clipboard_default_shortcut = "ctrl+alt+space";
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    let clipboard_default_shortcut = "alt+cmd+space";

    bindings.insert(
        "clipboard_only".to_string(),
        ShortcutBinding {
            id: "clipboard_only".to_string(),
            name: "Clipboard Only".to_string(),
            description: "Transcribes speech and copies to clipboard without pasting.".to_string(),
            default_binding: clipboard_default_shortcut.to_string(),
            current_binding: clipboard_default_shortcut.to_string(),
            action_type: "clipboard_only".to_string(),
        },
    );

    // Set platform-specific default keyboard shortcuts for append-to-file mode
    #[cfg(target_os = "windows")]
    let append_default_shortcut = "ctrl+shift+alt+space";
    #[cfg(target_os = "macos")]
    let append_default_shortcut = "alt+shift+cmd+space";
    #[cfg(target_os = "linux")]
    let append_default_shortcut = "ctrl+shift+alt+space";
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    let append_default_shortcut = "alt+shift+cmd+space";

    bindings.insert(
        "append_to_file".to_string(),
        ShortcutBinding {
            id: "append_to_file".to_string(),
            name: "Append to File".to_string(),
            description: "Transcribes speech and appends to log file with timestamp.".to_string(),
            default_binding: append_default_shortcut.to_string(),
            current_binding: append_default_shortcut.to_string(),
            action_type: "append_to_file".to_string(),
        },
    );
    // bindings.insert(
    //     "test".to_string(),
    //     ShortcutBinding {
    //         id: "test".to_string(),
    //         name: "Test".to_string(),
    //         description: "This is a test binding.".to_string(),
    //         default_binding: "ctrl+d".to_string(),
    //         current_binding: "ctrl+d".to_string(),
    //     },
    // );

    AppSettings {
        bindings,
        push_to_talk: true,
        audio_feedback: false,
        selected_model: "".to_string(),
        always_on_microphone: false,
        selected_microphone: None,
        selected_output_device: None,
        translate_to_english: false,
        shortcuts_enabled: default_shortcuts_enabled(),
        append_file_path: default_append_file_path(),
        show_notifications: default_show_notifications(),
    }
}

pub fn load_or_create_app_settings(app: &App) -> AppSettings {
    // Initialize store
    let store = app
        .store(SETTINGS_STORE_PATH)
        .expect("Failed to initialize store");

    let settings = if let Some(settings_value) = store.get("settings") {
        // Parse the entire settings object
        match serde_json::from_value::<AppSettings>(settings_value) {
            Ok(settings) => {
                println!("Found existing settings: {:?}", settings);

                settings
            }
            Err(e) => {
                println!("Failed to parse settings: {}", e);
                // Fall back to default settings if parsing fails
                let default_settings = get_default_settings();

                // Store the default settings
                store.set("settings", serde_json::to_value(&default_settings).unwrap());

                default_settings
            }
        }
    } else {
        // Create default settings
        let default_settings = get_default_settings();

        // Store the settings
        store.set("settings", serde_json::to_value(&default_settings).unwrap());

        default_settings
    };

    settings
}

pub fn get_settings(app: &AppHandle) -> AppSettings {
    let store = app
        .store(SETTINGS_STORE_PATH)
        .expect("Failed to initialize store");

    if let Some(settings_value) = store.get("settings") {
        serde_json::from_value::<AppSettings>(settings_value)
            .unwrap_or_else(|_| get_default_settings())
    } else {
        get_default_settings()
    }
}

pub fn write_settings(app: &AppHandle, settings: AppSettings) {
    let store = app
        .store(SETTINGS_STORE_PATH)
        .expect("Failed to initialize store");

    store.set("settings", serde_json::to_value(&settings).unwrap());
}

pub fn get_bindings(app: &AppHandle) -> HashMap<String, ShortcutBinding> {
    let settings = get_settings(app);

    settings.bindings
}

pub fn get_stored_binding(app: &AppHandle, id: &str) -> ShortcutBinding {
    let bindings = get_bindings(app);

    let binding = bindings.get(id).unwrap().clone();

    binding
}
