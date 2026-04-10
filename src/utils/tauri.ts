import { invoke } from '@tauri-apps/api/core';
import type {
  Settings,
  VideoInfo,
  Download,
  DownloadRequest,
  BatchDownloadRequest,
  DownloadProgress,
  DependenciesStatus,
} from '@/types';

export const api = {
  // Settings
  async getSettings(): Promise<Settings> {
    return invoke('get_settings');
  },

  async updateSettings(settings: Settings): Promise<void> {
    return invoke('update_settings', { newSettings: settings });
  },

  async resetSettings(): Promise<void> {
    return invoke('reset_settings');
  },

  // Downloads
  async resolveVideo(url: string): Promise<VideoInfo> {
    return invoke('resolve_video', { url });
  },

  async startDownload(request: DownloadRequest): Promise<Download> {
    return invoke('start_download', { request });
  },

  async cancelDownload(downloadId: string): Promise<boolean> {
    return invoke('cancel_download', { downloadId });
  },

  async getActiveDownloads(): Promise<Download[]> {
    return invoke('get_active_downloads');
  },

  async batchDownload(request: BatchDownloadRequest): Promise<Download[]> {
    return invoke('batch_download', { request });
  },

  // System
  async checkDependencies(): Promise<DependenciesStatus> {
    return invoke('check_dependencies');
  },

  async openFolder(path: string): Promise<void> {
    return invoke('open_folder', { path });
  },

  async getVersion(): Promise<string> {
    return invoke('get_version');
  },

  async getDownloadFolder(): Promise<string> {
    return invoke('get_download_folder');
  },
};

// Fix: return Promise<unlisten fn> so callers can clean up on unmount
export async function listenToProgress(
  callback: (progress: DownloadProgress) => void
): Promise<() => void> {
  const { listen } = await import('@tauri-apps/api/event');
  return listen<DownloadProgress>('download-progress', (event) => {
    callback(event.payload);
  });
}

export async function listenToComplete(
  callback: (download: Download) => void
): Promise<() => void> {
  const { listen } = await import('@tauri-apps/api/event');
  return listen<Download>('download-complete', (event) => {
    callback(event.payload);
  });
}
