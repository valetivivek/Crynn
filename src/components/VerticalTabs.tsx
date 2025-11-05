import React, { useState } from "react";
import { X, Plus, Pin, Volume2, VolumeX } from "lucide-react";
import { useTabsStore, Tab } from "../stores/tabsStore";

const VerticalTabs: React.FC = () => {
  const {
    tabs,
    activeTabId,
    setActiveTab,
    closeTab,
    pinTab,
    unpinTab,
    muteTab,
    unmuteTab,
    addTab,
    duplicateTab,
  } = useTabsStore();

  const [contextMenu, setContextMenu] = useState<{
    x: number;
    y: number;
    tabId: string;
  } | null>(null);

  // Separate pinned and unpinned tabs
  const pinnedTabs = tabs.filter((tab) => tab.pinned);
  const unpinnedTabs = tabs.filter((tab) => !tab.pinned);

  const handleContextMenu = (
    e: React.MouseEvent,
    tabId: string
  ) => {
    e.preventDefault();
    setContextMenu({ x: e.clientX, y: e.clientY, tabId });
  };

  const handleTabClick = (tabId: string) => {
    setActiveTab(tabId);
  };

  const renderTab = (tab: Tab) => {
    const isActive = tab.id === activeTabId;
    return (
      <div
        key={tab.id}
        className={`
          flex items-center gap-2 px-3 py-2.5 cursor-pointer
          hover:bg-muted/10 active:bg-muted/15 transition-all duration-200
          hover:translate-x-0.5
          ${isActive ? "bg-accent/10 border-l-[3px] border-accent shadow-sm" : "border-l-[3px] border-transparent"}
        `}
        onClick={() => handleTabClick(tab.id)}
        onContextMenu={(e) => handleContextMenu(e, tab.id)}
      >
        {tab.pinned && (
          <Pin className="w-3 h-3 text-muted flex-shrink-0" />
        )}
        {tab.audible && !tab.muted && (
          <Volume2 className="w-4 h-4 text-accent flex-shrink-0" />
        )}
        {tab.muted && (
          <VolumeX className="w-4 h-4 text-muted flex-shrink-0" />
        )}
        <img
          src={`https://www.google.com/s2/favicons?domain=${
            (() => {
              try {
                return new URL(tab.url).hostname;
              } catch {
                return "example.com";
              }
            })()
          }&sz=16`}
          alt=""
          className="w-4 h-4 flex-shrink-0"
          onError={(e) => {
            (e.target as HTMLImageElement).style.display = "none";
          }}
        />
        <span className="flex-1 truncate text-sm text-fg font-medium">{tab.title || "New Tab"}</span>
        <button
          onClick={async (e) => {
            e.stopPropagation();
            e.preventDefault();
            try {
              await closeTab(tab.id);
            } catch (error) {
              console.error("Error closing tab:", error);
            }
          }}
          className="opacity-0 group-hover:opacity-100 hover:bg-red-500/20 active:bg-red-500/30 rounded p-1.5 transition-all duration-200 hover:scale-110 active:scale-95"
        >
          <X className="w-3.5 h-3.5 text-muted hover:text-red-500 transition-colors" />
        </button>
      </div>
    );
  };

  return (
    <div className="w-[280px] bg-bg/95 backdrop-blur-sm border-r border-border flex flex-col h-full transition-all duration-200 overflow-hidden">
      {/* New Tab Button - Moved to Top */}
      <div className="p-2 border-b border-border">
        <button
          onClick={async (e) => {
            e.preventDefault();
            e.stopPropagation();
            console.log("New Tab button clicked");
            try {
              await addTab();
            } catch (error) {
              console.error("Error adding tab:", error);
            }
          }}
          className="group w-full flex items-center justify-center gap-2 px-3 py-2.5 text-sm font-medium bg-accent/10 hover:bg-accent/20 active:bg-accent/30 rounded-lg transition-all duration-200 text-fg hover:scale-[1.02] active:scale-[0.98] shadow-sm hover:shadow-md"
          type="button"
        >
          <Plus className="w-4 h-4 transition-transform duration-200 group-hover:rotate-90 group-active:rotate-180" />
          <span>New Tab</span>
        </button>
      </div>


      {/* Tabs */}
      <div className="flex-1 overflow-y-auto scrollbar-thin scrollbar-thumb-muted/20 scrollbar-track-transparent">
        {pinnedTabs.length > 0 && (
          <div className="animate-fadeIn">
            {pinnedTabs.map((tab) => renderTab(tab))}
            <div className="h-px bg-border/50 my-1 mx-2" />
          </div>
        )}
        {unpinnedTabs.map((tab) => (
          <div key={tab.id} className="group animate-slideIn">
            {renderTab(tab)}
          </div>
        ))}
      </div>

      {/* Context Menu */}
      {contextMenu && (
        <>
          <div
            className="fixed inset-0 z-10"
            onClick={() => setContextMenu(null)}
          />
          <div
            className="fixed z-20 bg-bg border border-border rounded shadow-lg py-1 min-w-[200px]"
            style={{ left: contextMenu.x, top: contextMenu.y }}
          >
            <button
              className="w-full text-left px-4 py-2 text-sm hover:bg-muted/10 text-fg"
              onClick={() => {
                duplicateTab(contextMenu.tabId);
                setContextMenu(null);
              }}
            >
              Duplicate
            </button>
            <button
              className="w-full text-left px-4 py-2 text-sm hover:bg-muted/10 text-fg flex items-center gap-2"
              onClick={() => {
                const tab = tabs.find((t) => t.id === contextMenu.tabId);
                if (tab?.pinned) {
                  unpinTab(contextMenu.tabId);
                } else {
                  pinTab(contextMenu.tabId);
                }
                setContextMenu(null);
              }}
            >
              <Pin className="w-4 h-4" />
              {tabs.find((t) => t.id === contextMenu.tabId)?.pinned
                ? "Unpin Tab"
                : "Pin Tab"}
            </button>
            <button
              className="w-full text-left px-4 py-2 text-sm hover:bg-muted/10 text-fg"
              onClick={() => {
                const tab = tabs.find((t) => t.id === contextMenu.tabId);
                if (tab?.muted) {
                  unmuteTab(contextMenu.tabId);
                } else {
                  muteTab(contextMenu.tabId);
                }
                setContextMenu(null);
              }}
            >
              {tabs.find((t) => t.id === contextMenu.tabId)?.muted
                ? "Unmute"
                : "Mute"}
            </button>
            <div className="h-px bg-border my-1" />
            <button
              className="w-full text-left px-4 py-2 text-sm hover:bg-muted/10 text-red-500"
              onClick={() => {
                closeTab(contextMenu.tabId);
                setContextMenu(null);
              }}
            >
              Close
            </button>
          </div>
        </>
      )}
    </div>
  );
};

export default VerticalTabs;

