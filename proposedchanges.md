 Core Feature Enhancements

  1. Notepad Integration (your suggestion): Add a hotkey that automatically opens notepad.exe, focuses it, and then
   begins transcription directly into that window. This would require extending the action system in actions.rs to
  support launching applications before transcription. //checked off 
  2. Transcription History & Log: Maintain a searchable database of all transcriptions with timestamps, allowing
  users to review, edit, and reuse previous transcriptions. Could integrate with the existing Tauri store system.
  3. Multiple Output Destinations: Beyond just pasting into the active window, add options for:
    - Copy to clipboard only (no paste)
    - Append to a dedicated log file
    - Send to specific applications (email drafts, note-taking apps)
    - Output to a floating overlay window
  4. Custom Voice Commands & Macros: Allow transcribed phrases to trigger system actions:
    - "Open calculator" → launches calculator app
    - "New email" → opens email client with new message
    - Custom text expansions/shortcuts
  5. Real-time Live Transcription: Display transcription in a floating, always-on-top window as the user speaks,
  before committing to paste/save.

  Audio & Input Improvements

  6. Multiple Hotkey Profiles: Different shortcuts for different transcription modes:
    - Quick paste (current behavior)
    - Notepad mode //checked off
    - Clipboard-only mode
    - Append-to-file mode
  7. Wake Word Detection: Add always-listening wake word capability (like "Hey Handy") as an alternative to
  hotkeys, using the existing VAD system.
  8. Audio Playback & Review: Let users hear their recorded audio before transcription is applied, with options to
  re-record or proceed.
  9. Noise Cancellation & Audio Enhancement: Integrate additional audio processing to improve transcription quality
   in noisy environments.

  Language & Processing Features

  10. Multi-language Support: Extend beyond English to support other languages that Whisper handles well (Spanish,
  French, German, etc.).
  11. Text Post-processing Pipeline:
    - Auto-capitalization of sentences
    - Smart punctuation insertion
    - Custom find/replace rules
    - Text formatting (bullet points, numbering)
  12. Voice-Controlled Editing: After transcription, allow voice commands like "delete last sentence", "capitalize
  that", "add period".

  Integration & Workflow Features

  13. Popular App Integrations: Direct integration with:
    - VS Code (insert at cursor)
    - Microsoft Office (Word, Outlook)
    - Slack/Discord/Teams
    - Browser text fields with extension support
  14. Template System: Pre-defined text templates that can be triggered by voice:
    - Email signatures
    - Common responses
    - Meeting notes templates
  15. Export & Sync Options:
    - Export transcription history to various formats
    - Cloud sync across devices
    - Integration with note-taking services (Notion, Obsidian)