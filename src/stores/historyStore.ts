import { create } from "zustand";

export interface HistoryEntry {
  id: string;
  url: string;
  title: string;
  visitCount: number;
  lastVisitTime: number;
}

interface HistoryState {
  history: HistoryEntry[];
  addEntry: (url: string, title: string) => void;
  searchHistory: (query: string) => HistoryEntry[];
  clearHistory: () => void;
  deleteEntry: (id: string) => void;
}

export const useHistoryStore = create<HistoryState>((set, get) => ({
  history: [],

  addEntry: (url: string, title: string) => {
    const existing = get().history.find((e) => e.url === url);
    if (existing) {
      set((state) => ({
        history: state.history.map((e) =>
          e.id === existing.id
            ? {
                ...e,
                title,
                visitCount: e.visitCount + 1,
                lastVisitTime: Date.now(),
              }
            : e
        ),
      }));
    } else {
      const entry: HistoryEntry = {
        id: `${Date.now()}-${Math.random()}`,
        url,
        title,
        visitCount: 1,
        lastVisitTime: Date.now(),
      };
      set((state) => ({
        history: [entry, ...state.history],
      }));
    }
  },

  searchHistory: (query: string) => {
    const lowerQuery = query.toLowerCase();
    return get().history.filter(
      (e) =>
        e.title.toLowerCase().includes(lowerQuery) ||
        e.url.toLowerCase().includes(lowerQuery)
    );
  },

  clearHistory: () => {
    set({ history: [] });
  },

  deleteEntry: (id: string) => {
    set((state) => ({
      history: state.history.filter((e) => e.id !== id),
    }));
  },
}));

