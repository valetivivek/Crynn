import { create } from "zustand";

const generateId = () => `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;

export interface Download {
  id: string;
  url: string;
  filename: string;
  path: string;
  totalBytes?: number;
  receivedBytes: number;
  status: "pending" | "inProgress" | "paused" | "completed" | "failed";
}

interface DownloadsState {
  downloads: Download[];
  addDownload: (url: string, filename: string, path: string) => void;
  updateDownload: (id: string, updates: Partial<Download>) => void;
  removeDownload: (id: string) => void;
  clearCompleted: () => void;
}

export const useDownloadsStore = create<DownloadsState>((set) => ({
  downloads: [],

  addDownload: (url: string, filename: string, path: string) => {
    const download: Download = {
      id: generateId(),
      url,
      filename,
      path,
      totalBytes: undefined,
      receivedBytes: 0,
      status: "pending",
    };
    set((state) => ({
      downloads: [...state.downloads, download],
    }));
  },

  updateDownload: (id: string, updates: Partial<Download>) => {
    set((state) => ({
      downloads: state.downloads.map((dl) =>
        dl.id === id ? { ...dl, ...updates } : dl
      ),
    }));
  },

  removeDownload: (id: string) => {
    set((state) => ({
      downloads: state.downloads.filter((dl) => dl.id !== id),
    }));
  },

  clearCompleted: () => {
    set((state) => ({
      downloads: state.downloads.filter(
        (dl) => dl.status !== "completed" && dl.status !== "failed"
      ),
    }));
  },
}));

