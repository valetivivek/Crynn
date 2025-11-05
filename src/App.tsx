import { useEffect, useState } from "react";
import { useTabsStore } from "./stores/tabsStore";
import { useSettingsStore } from "./stores/settingsStore";
import VerticalTabs from "./components/VerticalTabs";
import AddressBar from "./components/AddressBar";
import DownloadsPanel from "./components/DownloadsPanel";
import SettingsPage from "./pages/SettingsPage";
import EssentialsPage from "./components/EssentialsPage";
import { setupShortcuts } from "./lib/shortcuts";
import { applyTheme } from "./lib/theme";
import { Settings } from "lucide-react";
import { invoke } from "@tauri-apps/api/tauri";

function App() {
  const { initializeTabs, tabs, activeTabId, updateTab } = useTabsStore();
  const { loadSettings, settings, theme } = useSettingsStore();
  const [showSettings, setShowSettings] = useState(false);

  const handleNavigate = async (url: string) => {
    const activeTab = tabs.find((t) => t.id === activeTabId);
    if (activeTab) {
      // Update local state immediately (iframe will load directly)
      updateTab(activeTab.id, { url: url });
      
      // Try to update Firefox in background (non-blocking)
      try {
        await invoke("navigate_tab", {
          firefoxId: activeTab.firefoxId,
          url: url,
        });
      } catch (error) {
        console.error("Failed to navigate in Firefox:", error);
        // Continue anyway - iframe will work
      }
    }
  };

  useEffect(() => {
    loadSettings();
    initializeTabs();
    const cleanup = setupShortcuts();
    return cleanup;
  }, [loadSettings, initializeTabs]);

  useEffect(() => {
    if (settings) {
      applyTheme(settings.theme);
    }
  }, [settings]);

  useEffect(() => {
    // Apply theme immediately on mount
    const root = document.documentElement;
    if (!root.hasAttribute("data-theme")) {
      root.setAttribute("data-theme", "light"); // Default to light
    }
  }, []);

  useEffect(() => {
    // Apply theme when it changes
    const root = document.documentElement;
    root.setAttribute("data-theme", theme);
  }, [theme]);

  if (showSettings) {
    return <SettingsPage onClose={() => setShowSettings(false)} />;
  }

  return (
    <div className="flex h-screen bg-bg text-fg overflow-hidden flex-col">
      {/* Top Bar - Full Width */}
      <div className="flex items-center border-b border-border bg-bg/95 backdrop-blur-sm">
        <div className="w-[280px] border-r border-border flex-shrink-0">
          <div className="h-12 flex items-center justify-center">
            <span className="text-sm font-medium text-fg">Crynn</span>
          </div>
        </div>
        <div className="flex items-center flex-1 min-w-0">
          <AddressBar />
          <button
            onClick={() => setShowSettings(true)}
            className="p-2 hover:bg-muted/10 border-l border-border flex-shrink-0"
            title="Settings"
          >
            <Settings className="w-5 h-5" />
          </button>
        </div>
      </div>
      
      {/* Main Content */}
      <div className="flex flex-1 min-h-0 overflow-hidden">
        <VerticalTabs />
        <div className="flex flex-col flex-1 min-w-0">
          <div className="flex-1 relative">
          {/* Browser content area */}
          {tabs.length === 0 ? (
            <div className="absolute inset-0 flex items-center justify-center bg-bg">
              <div className="text-center">
                <h1 className="text-2xl font-light mb-4 text-fg">Welcome to Crynn</h1>
                <p className="text-muted">Press Ctrl/Cmd+T to open a new tab</p>
                <p className="text-sm text-muted mt-4">Or click the "New Tab" button below</p>
              </div>
            </div>
          ) : (
            <div className="absolute inset-0 bg-bg">
              {tabs.map((tab) => (
                <div
                  key={tab.id}
                  className={`absolute inset-0 ${
                    tab.id === activeTabId ? "z-10" : "hidden"
                  }`}
                >
                  {tab.id === activeTabId && (
                    <>
                      {tab.url === "about:blank" || tab.url.startsWith("about:") || tab.url.startsWith("crynn://") ? (
                        <EssentialsPage onNavigate={handleNavigate} />
                      ) : (
                        <iframe
                          key={tab.url}
                          src={tab.url}
                          className="w-full h-full border-0 bg-white"
                          title={tab.title}
                          sandbox="allow-same-origin allow-scripts allow-forms allow-popups allow-modals allow-top-navigation allow-downloads"
                          allow="fullscreen; microphone; camera"
                          allowFullScreen
                          onLoad={() => console.log("Frame loaded:", tab.url)}
                          onError={(e) => {
                            console.error("Frame error:", e);
                            // Fallback: show error message
                          }}
                        />
                      )}
                    </>
                  )}
                </div>
              ))}
            </div>
          )}
          </div>
          <DownloadsPanel />
        </div>
      </div>
    </div>
  );
}

export default App;

