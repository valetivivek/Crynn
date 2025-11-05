const translations: Record<string, Record<string, string>> = {
  en: {
    "app.name": "CrynN",
    "tab.new": "New Tab",
    "tab.close": "Close",
    "tab.duplicate": "Duplicate",
    "tab.pin": "Pin",
    "tab.unpin": "Unpin",
    "tab.mute": "Mute",
    "tab.unmute": "Unmute",
    "address.bar.placeholder": "Search or enter address",
    "downloads.title": "Downloads",
    "settings.title": "Settings",
    "settings.theme": "Theme",
    "settings.keybindings": "Keybindings",
    "settings.privacy": "Privacy",
  },
};

export const i18n = {
  t: (key: string, locale: string = "en"): string => {
    return translations[locale]?.[key] || key;
  },
};

