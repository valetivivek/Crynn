import React from "react";
import { useSettingsStore } from "../../stores/settingsStore";

const PrivacySettings: React.FC = () => {
  const { settings, saveSettings } = useSettingsStore();

  if (!settings) return null;

  const searchEngines = [
    { value: "https://www.google.com/search?q=", label: "Google" },
    { value: "https://duckduckgo.com/?q=", label: "DuckDuckGo" },
    { value: "https://www.bing.com/search?q=", label: "Bing" },
    { value: "https://search.brave.com/search?q=", label: "Brave Search" },
  ];

  return (
    <div className="space-y-4">
      <h2 className="text-lg font-medium">Privacy</h2>
      <div className="space-y-4">
        <div>
          <label className="block text-sm font-medium mb-2">
            Default Search Engine
          </label>
          <select
            value={settings.defaultSearch}
            onChange={(e) => {
              saveSettings({ ...settings, defaultSearch: e.target.value });
            }}
            className="w-full px-3 py-2 border border-border rounded bg-bg"
          >
            {searchEngines.map((engine) => (
              <option key={engine.value} value={engine.value}>
                {engine.label}
              </option>
            ))}
          </select>
        </div>
        <label className="flex items-center gap-2">
          <input
            type="checkbox"
            checked={settings.dnt}
            onChange={(e) => {
              saveSettings({ ...settings, dnt: e.target.checked });
            }}
          />
          <span>Send Do-Not-Track header</span>
        </label>
      </div>
    </div>
  );
};

export default PrivacySettings;

