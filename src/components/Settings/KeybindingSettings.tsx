import React, { useState } from "react";
import { useSettingsStore } from "../../stores/settingsStore";

const KeybindingSettings: React.FC = () => {
  const { settings, updateKeybinding } = useSettingsStore();
  const [capturing, setCapturing] = useState<string | null>(null);

  if (!settings) return null;

  const actionLabels: Record<string, string> = {
    newTab: "New Tab",
    closeTab: "Close Tab",
    nextTab: "Next Tab",
    prevTab: "Previous Tab",
    reopenClosed: "Reopen Closed Tab",
    focusAddress: "Focus Address Bar",
    newWindow: "New Window",
    tabSearch: "Tab Search",
  };

  const formatKeys = (keys: string[]): string => {
    return keys
      .map((key) => {
        if (key === "Control") return "Ctrl";
        if (key === "Meta") return "Cmd";
        if (key.startsWith("Key")) return key.slice(3);
        return key;
      })
      .join(" + ");
  };

  const handleStartCapture = (action: string) => {
    setCapturing(action);
    const handleKeyDown = (e: KeyboardEvent) => {
      e.preventDefault();
      const keys: string[] = [];
      if (e.ctrlKey) keys.push("Control");
      if (e.metaKey) keys.push("Meta");
      if (e.shiftKey) keys.push("Shift");
      if (e.altKey) keys.push("Alt");
      if (e.key && !["Control", "Meta", "Shift", "Alt"].includes(e.key)) {
        keys.push(e.key);
      }

      if (keys.length > 0) {
        updateKeybinding(action as any, keys);
        setCapturing(null);
        window.removeEventListener("keydown", handleKeyDown);
      }
    };
    window.addEventListener("keydown", handleKeyDown);
  };

  return (
    <div className="space-y-4">
      <h2 className="text-lg font-medium">Keybindings</h2>
      <div className="space-y-2">
        {Object.entries(settings.keybindings).map(([action, keybinding]) => (
          <div key={action} className="flex items-center justify-between">
            <span>{actionLabels[action] || action}</span>
            <button
              onClick={() => handleStartCapture(action)}
              className="px-3 py-1.5 text-sm border border-border rounded hover:bg-muted/10"
            >
              {capturing === action
                ? "Press keys..."
                : formatKeys(keybinding.keys)}
            </button>
          </div>
        ))}
      </div>
    </div>
  );
};

export default KeybindingSettings;

