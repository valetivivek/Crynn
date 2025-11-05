import { create } from "zustand";

const generateId = () => `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;

export interface Bookmark {
  id: string;
  title: string;
  url: string;
  parentId?: string;
  dateAdded: number;
}

export interface BookmarkFolder {
  id: string;
  title: string;
  parentId?: string;
  children: (Bookmark | BookmarkFolder)[];
}

interface BookmarksState {
  bookmarks: Bookmark[];
  folders: BookmarkFolder[];
  bookmarksBarVisible: boolean;
  addBookmark: (title: string, url: string, parentId?: string) => void;
  updateBookmark: (id: string, updates: Partial<Bookmark>) => void;
  deleteBookmark: (id: string) => void;
  toggleBookmarksBar: () => void;
  exportBookmarks: () => string;
  importBookmarks: (data: string) => void;
}

export const useBookmarksStore = create<BookmarksState>((set, get) => ({
  bookmarks: [],
  folders: [],
  bookmarksBarVisible: false,

  addBookmark: (title: string, url: string, parentId?: string) => {
    const bookmark: Bookmark = {
      id: generateId(),
      title,
      url,
      parentId,
      dateAdded: Date.now(),
    };
    set((state) => ({
      bookmarks: [...state.bookmarks, bookmark],
    }));
  },

  updateBookmark: (id: string, updates: Partial<Bookmark>) => {
    set((state) => ({
      bookmarks: state.bookmarks.map((b) =>
        b.id === id ? { ...b, ...updates } : b
      ),
    }));
  },

  deleteBookmark: (id: string) => {
    set((state) => ({
      bookmarks: state.bookmarks.filter((b) => b.id !== id),
    }));
  },

  toggleBookmarksBar: () => {
    set((state) => ({
      bookmarksBarVisible: !state.bookmarksBarVisible,
    }));
  },

  exportBookmarks: () => {
    const { bookmarks, folders } = get();
    return JSON.stringify({ bookmarks, folders }, null, 2);
  },

  importBookmarks: (data: string) => {
    try {
      const parsed = JSON.parse(data);
      if (parsed.bookmarks && parsed.folders) {
        set({ bookmarks: parsed.bookmarks, folders: parsed.folders });
      }
    } catch (error) {
      console.error("Failed to import bookmarks:", error);
    }
  },
}));

