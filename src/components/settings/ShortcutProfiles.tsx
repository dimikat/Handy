import React, { useEffect, useState, useRef } from "react";
import { BindingResponseSchema, ShortcutBindingsMap } from "../../lib/types";
import { type } from "@tauri-apps/plugin-os";
import { getKeyName } from "../../lib/utils/keyboard";
import ResetIcon from "../icons/ResetIcon";
import { InfoTooltip } from "../ui/InfoTooltip";
import { useSettings } from "../../hooks/useSettings";

interface ShortcutProfilesProps {
  className?: string;
}

export const ShortcutProfiles: React.FC<ShortcutProfilesProps> = ({ className = "" }) => {
  const { getSetting, updateBinding, resetBinding, isUpdating, isLoading } = useSettings();
  const [keyPressed, setKeyPressed] = useState<string[]>([]);
  const [recordedKeys, setRecordedKeys] = useState<string[]>([]);
  const [editingShortcutId, setEditingShortcutId] = useState<string | null>(null);
  const [originalBinding, setOriginalBinding] = useState<string>("");
  const [isMacOS, setIsMacOS] = useState<boolean>(false);
  const [shortcutsEnabled, setShortcutsEnabled] = useState<Record<string, boolean>>({});
  const shortcutRefs = useRef<Map<string, HTMLDivElement | null>>(new Map());

  const bindings = getSetting("bindings") || {};

  // Check if running on macOS
  useEffect(() => {
    const checkOsType = async () => {
      try {
        const osType = await type();
        setIsMacOS(osType === "macos");
      } catch (error) {
        console.error("Error detecting OS type:", error);
        setIsMacOS(false);
      }
    };

    checkOsType();
  }, []);

  // Initialize shortcuts enabled state
  useEffect(() => {
    const enabledState = getSetting("shortcuts_enabled") || {};
    setShortcutsEnabled(enabledState);
  }, [getSetting]);

  // Tooltip content for each shortcut type
  const getTooltipContent = (actionType: string) => {
    switch (actionType) {
      case "transcribe":
        return (
          <div>
            <p className="font-medium mb-2">Quick Transcribe</p>
            <p>Records audio and pastes text directly into the currently active window.</p>
            <p className="mt-2 text-xs text-mid-gray">Perfect for filling forms, documents, or any text field.</p>
          </div>
        );
      case "notepad_transcribe":
        return (
          <div>
            <p className="font-medium mb-2">Notepad Mode</p>
            <p>Automatically opens Notepad (Windows) or TextEdit (macOS), focuses it, and transcribes directly into the new document.</p>
            <p className="mt-2 text-xs text-mid-gray">Great for taking standalone notes or drafting content.</p>
          </div>
        );
      case "clipboard_only":
        return (
          <div>
            <p className="font-medium mb-2">Clipboard Only</p>
            <p>Transcribes audio and copies the text to your clipboard without pasting.</p>
            <p className="mt-2 text-xs text-mid-gray">Use Ctrl+V (or Cmd+V) to paste wherever you need it. Perfect for when you want to review before pasting.</p>
          </div>
        );
      case "append_to_file":
        return (
          <div>
            <p className="font-medium mb-2">Append to File</p>
            <p>Transcribes audio and saves it to a designated log file with timestamps.</p>
            <p className="mt-2 text-xs text-mid-gray">Perfect for maintaining a searchable transcript log or meeting notes.</p>
          </div>
        );
      default:
        return <p>Configure keyboard shortcut for this transcription mode.</p>;
    }
  };

  // Toggle shortcut enabled/disabled
  const toggleShortcutEnabled = (shortcutId: string) => {
    const newState = !shortcutsEnabled[shortcutId];
    setShortcutsEnabled(prev => ({
      ...prev,
      [shortcutId]: newState
    }));
    // TODO: Call Tauri command to update backend
    console.log(`Toggle shortcut ${shortcutId}: ${newState}`);
  };

  // Normalize modifier keys (unify left/right variants)
  const normalizeKey = (key: string): string => {
    if (key.startsWith("left ") || key.startsWith("right ")) {
      const parts = key.split(" ");
      if (parts.length === 2) {
        return parts[1];
      }
    }
    return key;
  };

  // Format keys for macOS display
  const formatMacOSKeys = (key: string): string => {
    if (!isMacOS) return key;
    const keyMap: Record<string, string> = {
      alt: "option",
    };
    return keyMap[key.toLowerCase()] || key;
  };

  // Format a key combination for display
  const formatKeyCombination = (combination: string): string => {
    if (!isMacOS) return combination;
    return combination.split("+").map(formatMacOSKeys).join(" + ");
  };

  useEffect(() => {
    if (editingShortcutId === null) return;

    const handleKeyDown = async (e: KeyboardEvent) => {
      e.preventDefault();
      const rawKey = getKeyName(e);
      const key = normalizeKey(rawKey);

      if (!keyPressed.includes(key)) {
        setKeyPressed((prev) => [...prev, key]);
        if (!recordedKeys.includes(key)) {
          setRecordedKeys((prev) => [...prev, key]);
        }
      }
    };

    const handleKeyUp = async (e: KeyboardEvent) => {
      e.preventDefault();
      const rawKey = getKeyName(e);
      const key = normalizeKey(rawKey);

      setKeyPressed((prev) => prev.filter((k) => k !== key));

      const updatedKeyPressed = keyPressed.filter((k) => k !== key);
      if (updatedKeyPressed.length === 0 && recordedKeys.length > 0) {
        const newShortcut = recordedKeys.join("+");

        if (editingShortcutId && bindings[editingShortcutId]) {
          try {
            await updateBinding(editingShortcutId, newShortcut);
          } catch (error) {
            console.error("Failed to change binding:", error);
          }

          setEditingShortcutId(null);
          setKeyPressed([]);
          setRecordedKeys([]);
          setOriginalBinding("");
        }
      }
    };

    const handleClickOutside = (e: MouseEvent) => {
      const activeElement = shortcutRefs.current.get(editingShortcutId);
      if (activeElement && !activeElement.contains(e.target as Node)) {
        setEditingShortcutId(null);
        setKeyPressed([]);
        setRecordedKeys([]);
        setOriginalBinding("");
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    window.addEventListener("keyup", handleKeyUp);
    window.addEventListener("click", handleClickOutside);

    return () => {
      window.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("keyup", handleKeyUp);
      window.removeEventListener("click", handleClickOutside);
    };
  }, [keyPressed, recordedKeys, editingShortcutId, bindings, originalBinding, updateBinding]);

  // Start recording a new shortcut
  const startRecording = (id: string) => {
    if (editingShortcutId === id) return;
    setOriginalBinding(bindings[id]?.current_binding || "");
    setEditingShortcutId(id);
    setKeyPressed([]);
    setRecordedKeys([]);
  };

  // Format the current shortcut keys being recorded
  const formatCurrentKeys = () => {
    if (recordedKeys.length === 0) return "Press keys...";
    if (!isMacOS) {
      return recordedKeys.join("+");
    }
    return recordedKeys.map(formatMacOSKeys).join(" + ");
  };

  // Store references to shortcut elements
  const setShortcutRef = (id: string, ref: HTMLDivElement | null) => {
    shortcutRefs.current.set(id, ref);
  };

  // Render a single shortcut profile
  const renderShortcutProfile = (binding: any, bindingId: string) => {
    const isEnabled = shortcutsEnabled[bindingId] !== false; // Default to true if not set
    
    return (
      <div key={bindingId} className="border border-mid-gray/20 rounded-lg p-4">
        <div className="flex items-center justify-between mb-3">
          <div className="flex items-center gap-3">
            <div className="flex items-center gap-2">
              <h3 className="font-medium text-sm">{binding.name}</h3>
              <InfoTooltip content={getTooltipContent(binding.action_type)} />
            </div>
            
            {/* Enable/Disable Toggle */}
            <label className="inline-flex items-center cursor-pointer">
              <input
                type="checkbox"
                className="sr-only peer"
                checked={isEnabled}
                onChange={() => toggleShortcutEnabled(bindingId)}
              />
              <div className="relative w-9 h-5 bg-mid-gray/20 peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-logo-primary rounded-full peer peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-logo-primary"></div>
            </label>
          </div>
        </div>

        <div className="flex items-center justify-between">
          <div 
            className={`flex items-center space-x-2 transition-opacity duration-200 ${
              isEnabled ? 'opacity-100' : 'opacity-50'
            }`}
          >
            {editingShortcutId === bindingId ? (
              <div
                ref={(ref) => setShortcutRef(bindingId, ref)}
                className="px-3 py-2 text-sm font-semibold border border-logo-primary bg-logo-primary/30 rounded min-w-[120px] text-center"
              >
                {formatCurrentKeys()}
              </div>
            ) : (
              <div
                className={`px-3 py-2 text-sm font-semibold bg-mid-gray/10 border border-mid-gray/80 rounded min-w-[120px] text-center transition-colors ${
                  isEnabled 
                    ? 'hover:bg-logo-primary/10 cursor-pointer hover:border-logo-primary' 
                    : 'cursor-not-allowed'
                }`}
                onClick={() => isEnabled && startRecording(bindingId)}
              >
                {formatKeyCombination(binding.current_binding)}
              </div>
            )}
            <button
              className={`px-2 py-2 rounded fill-text border border-transparent transition-all duration-150 ${
                isEnabled
                  ? 'hover:bg-logo-primary/30 active:bg-logo-primary/50 active:scale-95 hover:cursor-pointer hover:border-logo-primary'
                  : 'opacity-50 cursor-not-allowed'
              }`}
              onClick={() => isEnabled && resetBinding(bindingId)}
              disabled={!isEnabled || isUpdating(`binding_${bindingId}`)}
            >
              <ResetIcon className="w-4 h-4" />
            </button>
          </div>
        </div>

        {binding.action_type === "append_to_file" && isEnabled && (
          <div className="mt-3 pt-3 border-t border-mid-gray/10">
            <div className="flex items-center justify-between">
              <span className="text-sm text-mid-gray">Log file location:</span>
              <button className="px-3 py-1 text-sm bg-mid-gray/10 hover:bg-logo-primary/10 border border-mid-gray/80 hover:border-logo-primary rounded transition-colors">
                Choose File...
              </button>
            </div>
            <p className="text-xs text-mid-gray mt-1 break-all">
              ~/Documents/Handy/transcriptions.txt
            </p>
          </div>
        )}
      </div>
    );
  };

  if (isLoading) {
    return (
      <div className={`space-y-4 ${className}`}>
        <div className="text-sm text-mid-gray">Loading shortcuts...</div>
      </div>
    );
  }

  if (Object.keys(bindings).length === 0) {
    return (
      <div className={`space-y-4 ${className}`}>
        <div className="text-sm text-mid-gray">No shortcuts configured</div>
      </div>
    );
  }

  return (
    <div className={`space-y-4 ${className}`}>
      <div className="mb-6">
        <h2 className="text-lg font-semibold mb-2">Transcription Modes</h2>
        <p className="text-sm text-mid-gray">
          Configure keyboard shortcuts for different transcription modes. Toggle shortcuts on/off and customize key combinations.
        </p>
      </div>
      
      {Object.entries(bindings).map(([bindingId, binding]) => 
        renderShortcutProfile(binding, bindingId)
      )}
    </div>
  );
};