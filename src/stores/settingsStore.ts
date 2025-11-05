import { create } from "zustand";
import { invoke } from "@tauri-apps/api/tauri";

export interface Keybinding {
  action:
    | "newTab"
    | "closeTab"
    | "nextTab"
    | "prevTab"
    | "reopenClosed"
    | "focusAddress"
    | "newWindow"
    | "tabSearch";
  keys: string[];
}

export interface Settings {
  theme: "light" | "dark" | "auto" | "highContrast";
  accent?: string;
  keybindings: Record<Keybinding["action"], Keybinding>;
  locale: string;
  defaultSearch: string;
  dnt: boolean;
  profilePath: string;
}

interface SettingsState {
  settings: Settings | null;
  theme: string;
  loadSettings: () => Promise<void>;
  saveSettings: (settings: Settings) => Promise<void>;
  updateTheme: (theme: Settings["theme"]) => void;
  updateKeybinding: (action: Keybinding["action"], keys: string[]) => void;
}

const getEffectiveTheme = (theme: Settings["theme"]): string => {
  if (theme === "auto") {
    return window.matchMedia("(prefers-color-scheme: dark)").matches
      ? "dark"
      : "light";
  }
  return theme;
};

export const useSettingsStore = create<SettingsState>((set, get) => ({
  settings: null,
  theme: "light",

  loadSettings: async () => {
    try {
      const settings = await invoke<Settings>("get_settings");
      const effectiveTheme = getEffectiveTheme(settings.theme);
      set({ settings, theme: effectiveTheme });
    } catch (error) {
      console.error("Failed to load settings:", error);
      // Use defaults
      const defaultSettings: Settings = {
        theme: "auto",
        keybindings: {
          newTab: { action: "newTab", keys: ["Control", "KeyT"] },
          closeTab: { action: "closeTab", keys: ["Control", "KeyW"] },
          nextTab: { action: "nextTab", keys: ["Control", "Tab"] },
          prevTab: { action: "prevTab", keys: ["Control", "Shift", "Tab"] },
          reopenClosed: {
            action: "reopenClosed",
            keys: ["Control", "Shift", "KeyT"],
          },
          focusAddress: { action: "focusAddress", keys: ["Control", "KeyL"] },
          newWindow: { action: "newWindow", keys: ["Control", "KeyN"] },
          tabSearch: { action: "tabSearch", keys: ["Control", "KeyK"] },
        },
        locale: "en",
        defaultSearch: "https://www.google.com/search?q=",
        dnt: false,
        profilePath: "",
      };
      set({ settings: defaultSettings, theme: "light" });
    }
  },

  saveSettings: async (settings: Settings) => {
    try {
      await invoke("save_settings_command", { settings });
      const effectiveTheme = getEffectiveTheme(settings.theme);
      set({ settings, theme: effectiveTheme });
    } catch (error) {
      console.error("Failed to save settings:", error);
    }
  },

  updateTheme: (theme: Settings["theme"]) => {
    const settings = get().settings;
    if (settings) {
      const updated = { ...settings, theme };
      get().saveSettings(updated);
    }
  },

  updateKeybinding: (action: Keybinding["action"], keys: string[]) => {
    const settings = get().settings;
    if (settings) {
      const updated = {
        ...settings,
        keybindings: {
          ...settings.keybindings,
          [action]: { action, keys },
        },
      };
      get().saveSettings(updated);
    }
  },
}));

