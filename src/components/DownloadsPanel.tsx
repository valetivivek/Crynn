import React, { useState } from "react";
import { Download, X, FolderOpen, Trash2 } from "lucide-react";
import { useDownloadsStore } from "../stores/downloadsStore";

const DownloadsPanel: React.FC = () => {
  const { downloads, removeDownload, clearCompleted } = useDownloadsStore();
  const [isOpen, setIsOpen] = useState(false);

  const activeDownloads = downloads.filter(
    (dl) => dl.status === "pending" || dl.status === "inProgress"
  );

  const handleOpenFolder = async (path: string) => {
    try {
      // Open folder using Tauri shell API
      const { open } = await import("@tauri-apps/api/shell");
      await open(path);
    } catch (error) {
      console.error("Failed to open folder:", error);
    }
  };

  if (!isOpen && activeDownloads.length === 0) {
    return null;
  }

  return (
    <div className="border-t border-border bg-bg">
      <div className="flex items-center justify-between px-3 py-2">
        <div className="flex items-center gap-2">
          <Download className="w-4 h-4 text-muted" />
          <span className="text-sm font-medium">Downloads</span>
          {downloads.length > 0 && (
            <span className="text-xs text-muted">({downloads.length})</span>
          )}
        </div>
        <div className="flex items-center gap-2">
          {downloads.length > 0 && (
            <button
              onClick={clearCompleted}
              className="text-xs text-muted hover:text-fg"
            >
              Clear completed
            </button>
          )}
          <button
            onClick={() => setIsOpen(!isOpen)}
            className="p-1 hover:bg-muted/10 rounded"
          >
            <X className="w-4 h-4" />
          </button>
        </div>
      </div>

      {isOpen && downloads.length > 0 && (
        <div className="max-h-48 overflow-y-auto">
          {downloads.map((download) => (
            <div
              key={download.id}
              className="flex items-center gap-3 px-3 py-2 hover:bg-muted/5 border-b border-border last:border-b-0"
            >
              <div className="flex-1 min-w-0">
                <div className="text-sm truncate">{download.filename}</div>
                {download.status === "inProgress" && (
                  <div className="mt-1">
                    <div className="h-1 bg-muted/20 rounded-full overflow-hidden">
                      <div
                        className="h-full bg-accent transition-all"
                        style={{
                          width: `${
                            download.totalBytes
                              ? Math.round((download.receivedBytes / download.totalBytes) * 100)
                              : 0
                          }%`,
                        }}
                      />
                    </div>
                    <div className="text-xs text-muted mt-1">
                      {download.receivedBytes} / {download.totalBytes || "?"} bytes (
                      {download.totalBytes
                        ? Math.round((download.receivedBytes / download.totalBytes) * 100)
                        : 0}%)
                    </div>
                  </div>
                )}
                {download.status === "completed" && (
                  <div className="text-xs text-muted">Completed</div>
                )}
                {download.status === "failed" && (
                  <div className="text-xs text-red-500">Failed</div>
                )}
              </div>
              <div className="flex items-center gap-1">
                {download.status === "completed" && (
                  <button
                    onClick={() => handleOpenFolder(download.path)}
                    className="p-1.5 hover:bg-muted/10 rounded"
                    title="Open in folder"
                  >
                    <FolderOpen className="w-4 h-4" />
                  </button>
                )}
                <button
                  onClick={() => removeDownload(download.id)}
                  className="p-1.5 hover:bg-muted/10 rounded"
                  title="Remove"
                >
                  <Trash2 className="w-4 h-4" />
                </button>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default DownloadsPanel;

