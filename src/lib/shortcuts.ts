import { useTabsStore } from "../stores/tabsStore";
import { useSettingsStore } from "../stores/settingsStore";

export const setupShortcuts = () => {
  const handleKeyDown = (e: KeyboardEvent) => {
    const state = useSettingsStore.getState();
    if (!state.settings) return;

    const { addTab, closeTab, setActiveTab, tabs, activeTabId, reopenClosedTab } =
      useTabsStore.getState();
    const { settings } = state;

    const keybindings = settings.keybindings;
    const pressedKeys: string[] = [];
    if (e.ctrlKey || e.metaKey) pressedKeys.push("Control");
    if (e.shiftKey) pressedKeys.push("Shift");
    if (e.altKey) pressedKeys.push("Alt");
    pressedKeys.push(e.code || e.key);

    const matchesBinding = (binding: { keys: string[] }) => {
      if (binding.keys.length !== pressedKeys.length) return false;
      return binding.keys.every((key) => pressedKeys.includes(key));
    };

    // Check each keybinding
    if (matchesBinding(keybindings.newTab)) {
      e.preventDefault();
      addTab();
    } else if (matchesBinding(keybindings.closeTab)) {
      e.preventDefault();
      if (activeTabId) {
        closeTab(activeTabId);
      }
    } else if (matchesBinding(keybindings.nextTab)) {
      e.preventDefault();
      if (tabs.length > 0) {
        const currentIndex = tabs.findIndex((t) => t.id === activeTabId);
        const nextIndex = (currentIndex + 1) % tabs.length;
        setActiveTab(tabs[nextIndex].id);
      }
    } else if (matchesBinding(keybindings.prevTab)) {
      e.preventDefault();
      if (tabs.length > 0) {
        const currentIndex = tabs.findIndex((t) => t.id === activeTabId);
        const prevIndex = currentIndex === 0 ? tabs.length - 1 : currentIndex - 1;
        setActiveTab(tabs[prevIndex].id);
      }
    } else if (matchesBinding(keybindings.reopenClosed)) {
      e.preventDefault();
      reopenClosedTab();
    }
  };

  window.addEventListener("keydown", handleKeyDown);
  return () => window.removeEventListener("keydown", handleKeyDown);
};

