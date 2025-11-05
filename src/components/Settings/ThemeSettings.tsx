import React from "react";
import { useSettingsStore } from "../../stores/settingsStore";

const ThemeSettings: React.FC = () => {
  const { settings, updateTheme } = useSettingsStore();

  if (!settings) return null;

  return (
    <div className="space-y-4">
      <h2 className="text-lg font-medium">Theme</h2>
      <div className="space-y-2">
        <label className="flex items-center gap-2">
          <input
            type="radio"
            name="theme"
            value="light"
            checked={settings.theme === "light"}
            onChange={() => updateTheme("light")}
          />
          <span>Light</span>
        </label>
        <label className="flex items-center gap-2">
          <input
            type="radio"
            name="theme"
            value="dark"
            checked={settings.theme === "dark"}
            onChange={() => updateTheme("dark")}
          />
          <span>Dark</span>
        </label>
        <label className="flex items-center gap-2">
          <input
            type="radio"
            name="theme"
            value="auto"
            checked={settings.theme === "auto"}
            onChange={() => updateTheme("auto")}
          />
          <span>Auto (follow system)</span>
        </label>
        <label className="flex items-center gap-2">
          <input
            type="radio"
            name="theme"
            value="highContrast"
            checked={settings.theme === "highContrast"}
            onChange={() => updateTheme("highContrast")}
          />
          <span>High Contrast</span>
        </label>
      </div>

      {settings.accent && (
        <div>
          <label className="block text-sm font-medium mb-2">Accent Color</label>
          <input
            type="color"
            value={settings.accent}
            onChange={() => {
              // Update accent color (TODO: implement accent color update)
            }}
            className="w-20 h-10 rounded border border-border"
          />
        </div>
      )}
    </div>
  );
};

export default ThemeSettings;

