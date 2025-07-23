use crate::settings;

use enigo::Enigo;
use enigo::Key;
use enigo::Keyboard;
use enigo::Settings;
use std::process::Command;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use cpal::traits::{DeviceTrait, HostTrait};
use rodio::OutputStreamBuilder;
use std::fs::File;
use std::io::BufReader;
use std::thread;
use tauri::image::Image;
use tauri::tray::TrayIcon;
use tauri::AppHandle;
use tauri::Manager;
use tauri_plugin_clipboard_manager::ClipboardExt;

fn send_paste() -> Result<(), String> {
    // Determine the modifier key based on the OS
    #[cfg(target_os = "macos")]
    let modifier_key = Key::Meta; // Command key on macOS
    #[cfg(not(target_os = "macos"))]
    let modifier_key = Key::Control; // Control key on other systems

    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| format!("Failed to initialize Enigo: {}", e))?;

    // Press both keys
    enigo
        .key(modifier_key, enigo::Direction::Press)
        .map_err(|e| format!("Failed to press modifier key: {}", e))?;
    enigo
        .key(Key::Unicode('v'), enigo::Direction::Press)
        .map_err(|e| format!("Failed to press V key: {}", e))?;

    // Release both keys
    enigo
        .key(Key::Unicode('v'), enigo::Direction::Release)
        .map_err(|e| format!("Failed to release V key: {}", e))?;
    enigo
        .key(modifier_key, enigo::Direction::Release)
        .map_err(|e| format!("Failed to release modifier key: {}", e))?;

    Ok(())
}

pub fn paste(text: String, app_handle: AppHandle) -> Result<(), String> {
    let clipboard = app_handle.clipboard();

    // get the current clipboard content
    let clipboard_content = clipboard.read_text().unwrap_or_default();

    clipboard
        .write_text(&text)
        .map_err(|e| format!("Failed to write to clipboard: {}", e))?;

    // small delay to ensure the clipboard content has been written to
    std::thread::sleep(std::time::Duration::from_millis(50));

    send_paste()?;

    std::thread::sleep(std::time::Duration::from_millis(50));

    // restore the clipboard
    clipboard
        .write_text(&clipboard_content)
        .map_err(|e| format!("Failed to restore clipboard: {}", e))?;

    Ok(())
}

pub enum TrayIconState {
    Idle,
    Recording,
}

pub fn change_tray_icon(app: &AppHandle, icon: TrayIconState) {
    let tray = app.state::<TrayIcon>();

    let icon_path = match icon {
        TrayIconState::Idle => "resources/tray_idle.png",
        TrayIconState::Recording => "resources/tray_recording.png",
    };

    let _ = tray.set_icon(Some(
        Image::from_path(
            app.path()
                .resolve(icon_path, tauri::path::BaseDirectory::Resource)
                .expect("failed to resolve"),
        )
        .expect("failed to set icon"),
    ));
}

/// Plays an audio resource from the resources directory.
/// Checks if audio feedback is enabled in settings before playing.
pub fn play_sound(app: &AppHandle, resource_path: &str) {
    // Check if audio feedback is enabled
    let settings = settings::get_settings(app);
    if !settings.audio_feedback {
        return;
    }

    let app_handle = app.clone();
    let resource_path = resource_path.to_string();

    // Spawn a new thread to play the audio without blocking the main thread
    thread::spawn(move || {
        // Get the path to the audio file in resources
        let audio_path = match app_handle
            .path()
            .resolve(&resource_path, tauri::path::BaseDirectory::Resource)
        {
            Ok(path) => path,
            Err(e) => {
                eprintln!(
                    "Failed to resolve audio file path '{}': {}",
                    resource_path, e
                );
                return;
            }
        };

        // Get the selected output device from settings
        let settings = settings::get_settings(&app_handle);
        let selected_device = settings.selected_output_device.clone();

        // Try to play the audio file
        if let Err(e) = play_audio_file(&audio_path, selected_device) {
            eprintln!("Failed to play sound '{}': {}", resource_path, e);
        }
    });
}

/// Convenience function to play the recording start sound
pub fn play_recording_start_sound(app: &AppHandle) {
    play_sound(app, "resources/rec_start.wav");
}

/// Convenience function to play the recording stop sound
pub fn play_recording_stop_sound(app: &AppHandle) {
    play_sound(app, "resources/rec_stop.wav");
}

fn play_audio_file(
    path: &std::path::Path,
    selected_device: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let stream_builder = if let Some(device_name) = selected_device {
        if device_name == "Default" {
            println!("Using default device");
            // Use default device
            OutputStreamBuilder::from_default_device()?
        } else {
            // Try to find the device by name
            let host = cpal::default_host();
            let devices = host.output_devices()?;

            let mut found_device = None;
            for device in devices {
                if device.name()? == device_name {
                    found_device = Some(device);
                    break;
                }
            }

            match found_device {
                Some(device) => OutputStreamBuilder::from_device(device)?,
                None => {
                    eprintln!("Device '{}' not found, using default device", device_name);
                    OutputStreamBuilder::from_default_device()?
                }
            }
        }
    } else {
        println!("Using default device");
        // Use default device
        OutputStreamBuilder::from_default_device()?
    };

    let stream_handle = stream_builder.open_stream()?;
    let mixer = stream_handle.mixer();

    // Load the audio file
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);

    let sink = rodio::play(mixer, buf_reader)?;
    sink.sleep_until_end();

    Ok(())
}

/// Launch notepad.exe on Windows
/// Returns the process ID if successful
#[cfg(target_os = "windows")]
pub fn launch_notepad() -> Result<u32, String> {
    match Command::new("notepad.exe").spawn() {
        Ok(child) => {
            // Small delay to allow the process to fully start
            std::thread::sleep(std::time::Duration::from_millis(500));
            Ok(child.id())
        }
        Err(e) => Err(format!("Failed to launch notepad: {}", e)),
    }
}

/// Launch notepad.exe on non-Windows platforms (fallback to default text editor)
#[cfg(not(target_os = "windows"))]
pub fn launch_notepad() -> Result<u32, String> {
    // On macOS, try TextEdit
    #[cfg(target_os = "macos")]
    let editor_cmd = "open";
    #[cfg(target_os = "macos")]  
    let editor_args = vec!["-a", "TextEdit"];
    
    // On Linux, try gedit or nano
    #[cfg(target_os = "linux")]
    let editor_cmd = "gedit";
    #[cfg(target_os = "linux")]
    let editor_args: Vec<&str> = vec![];
    
    // Fallback for other platforms
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    let editor_cmd = "notepad";
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    let editor_args: Vec<&str> = vec![];

    let mut command = Command::new(editor_cmd);
    for arg in editor_args {
        command.arg(arg);
    }
    
    match command.spawn() {
        Ok(child) => {
            // Small delay to allow the process to fully start
            std::thread::sleep(std::time::Duration::from_millis(500));
            Ok(child.id())
        }
        Err(e) => Err(format!("Failed to launch text editor: {}", e)),
    }
}

/// Focus a window by attempting to bring it to the front
/// This is a simplified implementation that works by simulating Alt+Tab behavior
pub fn focus_notepad_window() -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| format!("Failed to initialize Enigo for window focusing: {}", e))?;

    // Small delay to ensure the window has had time to appear
    std::thread::sleep(std::time::Duration::from_millis(200));

    // On Windows, we can try to use Alt+Tab to cycle to notepad
    // This is a simplified approach - a more robust solution would use Windows API
    #[cfg(target_os = "windows")]
    {
        // Try to focus the most recently opened window (which should be notepad)
        // Note: This is a simplified approach. A more robust solution would 
        // enumerate windows and find notepad specifically.
        enigo
            .key(Key::Alt, enigo::Direction::Press)
            .map_err(|e| format!("Failed to press Alt key: {}", e))?;
        enigo
            .key(Key::Tab, enigo::Direction::Press)
            .map_err(|e| format!("Failed to press Tab key: {}", e))?;
        enigo
            .key(Key::Tab, enigo::Direction::Release)
            .map_err(|e| format!("Failed to release Tab key: {}", e))?;
        enigo
            .key(Key::Alt, enigo::Direction::Release)
            .map_err(|e| format!("Failed to release Alt key: {}", e))?;
    }

    // For macOS, use Cmd+Tab
    #[cfg(target_os = "macos")]
    {
        enigo
            .key(Key::Meta, enigo::Direction::Press)
            .map_err(|e| format!("Failed to press Cmd key: {}", e))?;
        enigo
            .key(Key::Tab, enigo::Direction::Press)
            .map_err(|e| format!("Failed to press Tab key: {}", e))?;
        enigo
            .key(Key::Tab, enigo::Direction::Release)
            .map_err(|e| format!("Failed to release Tab key: {}", e))?;
        enigo
            .key(Key::Meta, enigo::Direction::Release)
            .map_err(|e| format!("Failed to release Cmd key: {}", e))?;
    }

    // For Linux, use Alt+Tab
    #[cfg(target_os = "linux")]
    {
        enigo
            .key(Key::Alt, enigo::Direction::Press)
            .map_err(|e| format!("Failed to press Alt key: {}", e))?;
        enigo
            .key(Key::Tab, enigo::Direction::Press)
            .map_err(|e| format!("Failed to press Tab key: {}", e))?;
        enigo
            .key(Key::Tab, enigo::Direction::Release)
            .map_err(|e| format!("Failed to release Tab key: {}", e))?;
        enigo
            .key(Key::Alt, enigo::Direction::Release)
            .map_err(|e| format!("Failed to release Alt key: {}", e))?;
    }

    Ok(())
}

/// Append text to a file with timestamp
pub fn append_to_file(text: &str, file_path: &str) -> Result<(), String> {
    // Ensure the parent directory exists
    if let Some(parent) = Path::new(file_path).parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }
    }

    // Get current timestamp
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| format!("Failed to get timestamp: {}", e))?;
    
    // Format timestamp as readable string
    let timestamp = chrono::DateTime::from_timestamp(now.as_secs() as i64, 0)
        .ok_or("Invalid timestamp")?
        .format("%Y-%m-%d %H:%M:%S");

    // Format the entry with timestamp and text
    let entry = format!("[{}] {}\n", timestamp, text);

    // Append to file
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .map_err(|e| format!("Failed to open file '{}': {}", file_path, e))?;

    file.write_all(entry.as_bytes())
        .map_err(|e| format!("Failed to write to file: {}", e))?;

    file.flush()
        .map_err(|e| format!("Failed to flush file: {}", e))?;

    Ok(())
}

/// Get the default log file path for the current platform
pub fn get_default_log_path() -> Result<String, String> {
    // Get the user's documents directory
    let documents_dir = dirs::document_dir()
        .ok_or("Could not find documents directory")?;
    
    // Create Handy subdirectory path
    let handy_dir = documents_dir.join("Handy");
    
    // Ensure the directory exists
    if !handy_dir.exists() {
        std::fs::create_dir_all(&handy_dir)
            .map_err(|e| format!("Failed to create Handy directory: {}", e))?;
    }
    
    // Return the full path to the transcriptions log file
    let log_file = handy_dir.join("transcriptions.txt");
    Ok(log_file.to_string_lossy().to_string())
}
