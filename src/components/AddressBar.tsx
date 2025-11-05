import React, { useState, useEffect, useRef } from "react";
import { ArrowLeft, ArrowRight, RotateCcw, Home } from "lucide-react";
import { useTabsStore } from "../stores/tabsStore";
import { useHistoryStore } from "../stores/historyStore";
import { useSettingsStore } from "../stores/settingsStore";
import { invoke } from "@tauri-apps/api/tauri";

const AddressBar: React.FC = () => {
  const { tabs, activeTabId, updateTab } = useTabsStore();
  const { addEntry, searchHistory } = useHistoryStore();
  const { settings } = useSettingsStore();
  const [url, setUrl] = useState("");
  const [isFocused, setIsFocused] = useState(false);
  const [suggestions, setSuggestions] = useState<any[]>([]);
  const inputRef = useRef<HTMLInputElement>(null);

  const activeTab = tabs.find((t) => t.id === activeTabId);

  useEffect(() => {
    if (activeTab) {
      setUrl(activeTab.url);
    }
  }, [activeTab]);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key === "l") {
        e.preventDefault();
        inputRef.current?.focus();
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, []);

  const handleUrlChange = (value: string) => {
    setUrl(value);
    if (value.trim()) {
      const historyResults = searchHistory(value);
      setSuggestions(historyResults.slice(0, 5));
    } else {
      setSuggestions([]);
    }
  };

  const handleNavigate = async (navUrl: string) => {
    if (!activeTab) {
      // If no active tab, create a new one
      const { addTab } = useTabsStore.getState();
      await addTab(navUrl.trim() || "about:blank");
      return;
    }

    let finalUrl = navUrl.trim();
    if (!finalUrl) return;
    
    // Handle special URLs
    if (finalUrl.startsWith("about:") || finalUrl.startsWith("crynn://")) {
      // Handle internal URLs
    } else if (!finalUrl.startsWith("http://") && !finalUrl.startsWith("https://")) {
      if (finalUrl.includes(".") && !finalUrl.includes(" ")) {
        finalUrl = `https://${finalUrl}`;
      } else {
        const searchUrl = settings?.defaultSearch || "https://www.google.com/search?q=";
        finalUrl = `${searchUrl}${encodeURIComponent(finalUrl)}`;
      }
    }

    // Always update local state first (for immediate UI update)
    updateTab(activeTab.id, { url: finalUrl });
    addEntry(finalUrl, "");
    setSuggestions([]);
    inputRef.current?.blur();
    
    // Try to update Firefox in background (but don't block)
    try {
      await invoke("navigate_tab", {
        firefoxId: activeTab.firefoxId,
        url: finalUrl,
      });
    } catch (error) {
      console.error("Failed to navigate in Firefox:", error);
      // Continue anyway - iframe will load directly
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter") {
      handleNavigate(url);
    } else if (e.key === "Escape") {
      setSuggestions([]);
      inputRef.current?.blur();
    }
  };

  const handleBack = async () => {
    // Navigation history would need to be tracked in Firefox
    // This is a placeholder
    console.log("Back navigation");
  };

  const handleForward = async () => {
    // Navigation history would need to be tracked in Firefox
    // This is a placeholder
    console.log("Forward navigation");
  };

  const handleReload = async () => {
    if (activeTab) {
      try {
        await invoke("navigate_tab", {
          firefoxId: activeTab.firefoxId,
          url: activeTab.url,
        });
      } catch (error) {
        console.error("Failed to reload:", error);
      }
    }
  };

  const handleHome = () => {
    handleNavigate("about:blank");
  };

  return (
    <div className="h-12 bg-transparent flex items-center gap-2 px-3 flex-1">
      <div className="flex items-center gap-1 flex-shrink-0">
        <button
          onClick={handleBack}
          className="p-2 hover:bg-muted/10 active:bg-muted/20 rounded-lg transition-all duration-200 hover:scale-105 active:scale-95"
          title="Back"
        >
          <ArrowLeft className="w-4 h-4 text-fg" />
        </button>
        <button
          onClick={handleForward}
          className="p-2 hover:bg-muted/10 active:bg-muted/20 rounded-lg transition-all duration-200 hover:scale-105 active:scale-95"
          title="Forward"
        >
          <ArrowRight className="w-4 h-4 text-fg" />
        </button>
        <button
          onClick={handleReload}
          className="p-2 hover:bg-muted/10 active:bg-muted/20 rounded-lg transition-all duration-200 hover:scale-105 active:scale-95 hover:rotate-180"
          title="Reload"
        >
          <RotateCcw className="w-4 h-4 text-fg" />
        </button>
        <button
          onClick={handleHome}
          className="p-2 hover:bg-muted/10 active:bg-muted/20 rounded-lg transition-all duration-200 hover:scale-105 active:scale-95"
          title="Home"
        >
          <Home className="w-4 h-4 text-fg" />
        </button>
      </div>

      <div className="flex-1 relative">
        <input
          ref={inputRef}
          type="text"
          value={url}
          onChange={(e) => handleUrlChange(e.target.value)}
          onKeyDown={handleKeyDown}
          onFocus={() => setIsFocused(true)}
          onBlur={() => {
            setTimeout(() => setIsFocused(false), 200);
          }}
          placeholder="Search or enter address"
          className="w-full px-4 py-2.5 text-sm bg-muted/10 hover:bg-muted/15 focus:bg-bg rounded-lg border border-border focus:border-accent focus:outline-none focus:ring-2 focus:ring-accent/20 transition-all duration-200 text-fg placeholder:text-muted"
        />

        {/* Suggestions */}
        {isFocused && suggestions.length > 0 && (
          <div className="absolute top-full left-0 right-0 mt-1 bg-bg border border-border rounded shadow-lg z-50">
            {suggestions.map((suggestion) => (
              <button
                key={suggestion.id}
                onClick={() => handleNavigate(suggestion.url)}
                className="w-full text-left px-3 py-2 text-sm hover:bg-muted/10 flex items-center gap-2"
              >
                <span className="truncate">{suggestion.title || suggestion.url}</span>
              </button>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};

export default AddressBar;

