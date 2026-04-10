import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { Settings, Download, VideoInfo } from '@/types';
import { api } from '@/utils/tauri';

interface AppState {
  settings: Settings | null;
  activeDownloads: Download[];
  currentVideo: VideoInfo | null;
  dependencies: { yt_dlp: boolean; ffmpeg: boolean } | null;
  isLoading: boolean;
  error: string | null;
  
  // Actions
  setSettings: (settings: Settings) => void;
  setActiveDownloads: (downloads: Download[]) => void;
  updateDownload: (downloadId: string, updates: Partial<Download>) => void;
  setCurrentVideo: (video: VideoInfo | null) => void;
  setDependencies: (deps: { yt_dlp: boolean; ffmpeg: boolean }) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  
  // API Actions
  fetchSettings: () => Promise<void>;
  updateSettings: (settings: Settings) => Promise<void>;
  fetchDependencies: () => Promise<void>;
}

export const useAppStore = create<AppState>()(
  persist(
    (set, get) => ({
      settings: null,
      activeDownloads: [],
      currentVideo: null,
      dependencies: null,
      isLoading: false,
      error: null,

      setSettings: (settings) => set({ settings }),
      setActiveDownloads: (downloads) => set({ activeDownloads: downloads }),
      updateDownload: (downloadId, updates) => set((state) => ({
        activeDownloads: state.activeDownloads.map((d) =>
          d.id === downloadId ? { ...d, ...updates } : d
        ),
      })),
      setCurrentVideo: (video) => set({ currentVideo: video }),
      setDependencies: (deps) => set({ dependencies: deps }),
      setLoading: (loading) => set({ isLoading: loading }),
      setError: (error) => set({ error }),

      fetchSettings: async () => {
        try {
          set({ isLoading: true, error: null });
          const settings = await api.getSettings();
          set({ settings, isLoading: false });
        } catch (error) {
          set({ error: (error as Error).message, isLoading: false });
        }
      },

      updateSettings: async (settings) => {
        try {
          set({ isLoading: true, error: null });
          await api.updateSettings(settings);
          set({ settings, isLoading: false });
        } catch (error) {
          set({ error: (error as Error).message, isLoading: false });
          throw error;
        }
      },

      fetchDependencies: async () => {
        try {
          const deps = await api.checkDependencies();
          set({ dependencies: deps });
        } catch (error) {
          set({ error: (error as Error).message });
        }
      },
    }),
    {
      name: 'abi-yt-downloader-storage',
      partialize: (state) => ({
        settings: state.settings,
      }),
    }
  )
);