import { create } from "zustand";
import { invoke } from "@tauri-apps/api/tauri";
import { v4 as uuidv4 } from "uuid";

// Simple UUID fallback if uuid package fails
const generateId = () => {
  try {
    return uuidv4();
  } catch {
    return `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
  }
};

export interface Tab {
  id: string;
  firefoxId: string;
  title: string;
  url: string;
  pinned: boolean;
  muted: boolean;
  audible: boolean;
  groupId?: string;
}

export interface TabGroup {
  id: string;
  name: string;
  collapsed: boolean;
  tabIds: string[];
  color?: string;
}

interface TabsState {
  tabs: Tab[];
  groups: TabGroup[];
  activeTabId: string | null;
  closedTabs: Tab[];
  addTab: (url?: string) => Promise<void>;
  closeTab: (id: string) => Promise<void>;
  updateTab: (id: string, updates: Partial<Tab>) => void;
  setActiveTab: (id: string) => void;
  pinTab: (id: string) => void;
  unpinTab: (id: string) => void;
  muteTab: (id: string) => void;
  unmuteTab: (id: string) => void;
  reopenClosedTab: () => void;
  duplicateTab: (id: string) => Promise<void>;
  moveTabToGroup: (tabId: string, groupId: string | null) => void;
  createGroup: (name: string) => string;
  deleteGroup: (id: string) => void;
  toggleGroupCollapse: (id: string) => void;
  initializeTabs: () => Promise<void>;
  syncWithFirefox: () => Promise<void>;
}

export const useTabsStore = create<TabsState>((set, get) => ({
  tabs: [],
  groups: [],
  activeTabId: null,
  closedTabs: [],

  addTab: async (url = "about:blank") => {
    try {
      // Ensure Firefox is running
      let isRunning = false;
      try {
        isRunning = await invoke<boolean>("is_firefox_running");
      } catch (e) {
        console.log("Checking Firefox status...");
      }

      if (!isRunning) {
        console.log("Launching Firefox...");
        try {
          const settings = await invoke<any>("get_settings");
          const profilePath = settings?.profilePath || "";
          await invoke("launch_firefox", { profilePath });
          // Wait for Firefox to start
          await new Promise((resolve) => setTimeout(resolve, 3000));
        } catch (error) {
          console.error("Failed to launch Firefox:", error);
          // Create tab anyway - might work if Firefox is already running externally
        }
      }

      console.log("Opening tab with URL:", url);
      const firefoxId = await invoke<string>("open_tab", { url });
      console.log("Got Firefox tab ID:", firefoxId);
      
      const newTab: Tab = {
        id: generateId(),
        firefoxId,
        title: "New Tab",
        url,
        pinned: false,
        muted: false,
        audible: false,
      };
      
      set((state) => ({
        tabs: [...state.tabs, newTab],
        activeTabId: newTab.id,
      }));
      
      // Sync with Firefox to get actual tab info
      setTimeout(() => {
        get().syncWithFirefox();
      }, 1000);
    } catch (error) {
      console.error("Failed to add tab:", error);
      // Still create a local tab even if Firefox fails
      const newTab: Tab = {
        id: generateId(),
        firefoxId: generateId(),
        title: "New Tab",
        url,
        pinned: false,
        muted: false,
        audible: false,
      };
      set((state) => ({
        tabs: [...state.tabs, newTab],
        activeTabId: newTab.id,
      }));
    }
  },

  closeTab: async (id: string) => {
    const state = get();
    const tab = state.tabs.find((t) => t.id === id);
    if (!tab) return;

    try {
      // Try to close in Firefox
      await invoke("close_tab", { firefoxId: tab.firefoxId });
    } catch (error) {
      console.error("Failed to close tab in Firefox:", error);
      // Continue anyway - close locally
    }

    // Always close locally
    const newTabs = state.tabs.filter((t) => t.id !== id);
    const closedTabs = [...state.closedTabs, tab];
    const activeTabId =
      state.activeTabId === id
        ? newTabs.length > 0
          ? newTabs[newTabs.length - 1].id
          : null
        : state.activeTabId;

    set({ tabs: newTabs, closedTabs, activeTabId });
  },

  updateTab: (id: string, updates: Partial<Tab>) => {
    set((state) => ({
      tabs: state.tabs.map((tab) =>
        tab.id === id ? { ...tab, ...updates } : tab
      ),
    }));
  },

  setActiveTab: (id: string) => set({ activeTabId: id }),

  pinTab: (id: string) => {
    get().updateTab(id, { pinned: true });
  },

  unpinTab: (id: string) => {
    get().updateTab(id, { pinned: false });
  },

  muteTab: (id: string) => {
    get().updateTab(id, { muted: true });
  },

  unmuteTab: (id: string) => {
    get().updateTab(id, { muted: false });
  },

  reopenClosedTab: () => {
    const state = get();
    if (state.closedTabs.length === 0) return;
    const tab = state.closedTabs[state.closedTabs.length - 1];
    const closedTabs = state.closedTabs.slice(0, -1);
    set((state) => ({
      tabs: [...state.tabs, tab],
      closedTabs,
      activeTabId: tab.id,
    }));
    // Reopen in Firefox
    get()
      .addTab(tab.url)
      .then(() => {
        get().updateTab(state.tabs[state.tabs.length - 1].id, {
          firefoxId: tab.firefoxId,
        });
      });
  },

  duplicateTab: async (id: string) => {
    const tab = get().tabs.find((t) => t.id === id);
    if (!tab) return;
    await get().addTab(tab.url);
  },

  moveTabToGroup: (tabId: string, groupId: string | null) => {
    get().updateTab(tabId, { groupId: groupId || undefined });
  },

  createGroup: (name: string) => {
    const id = generateId();
    const group: TabGroup = {
      id,
      name,
      collapsed: false,
      tabIds: [],
    };
    set((state) => ({
      groups: [...state.groups, group],
    }));
    return id;
  },

  deleteGroup: (id: string) => {
    set((state) => ({
      groups: state.groups.filter((g) => g.id !== id),
      tabs: state.tabs.map((tab) =>
        tab.groupId === id ? { ...tab, groupId: undefined } : tab
      ),
    }));
  },

  toggleGroupCollapse: (id: string) => {
    set((state) => ({
      groups: state.groups.map((group) =>
        group.id === id ? { ...group, collapsed: !group.collapsed } : group
      ),
    }));
  },

  initializeTabs: async () => {
    try {
      const isRunning = await invoke<boolean>("is_firefox_running");
      if (!isRunning) {
        // Launch Firefox if not running
        const settings = await invoke<any>("get_settings");
        const profilePath = settings.profilePath || "";
        if (profilePath) {
          await invoke("launch_firefox", { profilePath });
        } else {
          // Use default profile path
          const defaultProfile = "";
          await invoke("launch_firefox", { profilePath: defaultProfile });
        }
      }
      await get().syncWithFirefox();
    } catch (error) {
      console.error("Failed to initialize tabs:", error);
    }
  },

  syncWithFirefox: async () => {
    try {
      const firefoxTabs = await invoke<any[]>("list_tabs");
      const state = get();
      const existingTabs = new Map(state.tabs.map((t) => [t.firefoxId, t]));

      // Update existing tabs and add new ones
      const tabs: Tab[] = firefoxTabs.map((ft) => {
        const existing = existingTabs.get(ft.id);
        if (existing) {
          return {
            ...existing,
            title: ft.title,
            url: ft.url,
            audible: ft.audible,
          };
        }
        return {
          id: generateId(),
          firefoxId: ft.id,
          title: ft.title,
          url: ft.url,
          pinned: false,
          muted: false,
          audible: ft.audible,
        };
      });

      // Remove tabs that no longer exist in Firefox
      const firefoxIds = new Set(firefoxTabs.map((ft) => ft.id));
      const filteredTabs = tabs.filter((t) => firefoxIds.has(t.firefoxId));

      set({ tabs: filteredTabs });
    } catch (error) {
      console.error("Failed to sync with Firefox:", error);
    }
  },
}));

