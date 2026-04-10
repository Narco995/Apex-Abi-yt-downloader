import { useEffect, useCallback } from 'react';
import { useAppStore } from '@/stores/useAppStore';
import { api, listenToProgress, listenToComplete } from '@/utils/tauri';
import type { DownloadProgress, Download } from '@/types';

export function useDownloads() {
  const { activeDownloads, setActiveDownloads, updateDownload, fetchDependencies } = useAppStore();

  useEffect(() => {
    fetchDependencies();
    loadActiveDownloads();
    setupEventListeners();
  }, []);

  const loadActiveDownloads = async () => {
    try {
      const downloads = await api.getActiveDownloads();
      setActiveDownloads(downloads);
    } catch (error) {
      console.error('Failed to load active downloads:', error);
    }
  };

  const setupEventListeners = () => {
    listenToProgress((progress: DownloadProgress) => {
      updateDownload(progress.id, {
        progress: progress.progress,
        downloaded_bytes: progress.downloaded_bytes,
        total_bytes: progress.total_bytes,
        speed: progress.speed,
        eta_seconds: progress.eta_seconds,
      });
    });

    listenToComplete((download: Download) => {
      updateDownload(download.id, {
        status: download.status,
        progress: download.progress,
        error: download.error,
        completed_at: download.completed_at,
      });
    });
  };

  const startDownload = useCallback(async (request: any) => {
    try {
      const download = await api.startDownload(request);
      setActiveDownloads([...activeDownloads, download]);
      return download;
    } catch (error) {
      console.error('Failed to start download:', error);
      throw error;
    }
  }, [activeDownloads, setActiveDownloads]);

  const cancelDownload = useCallback(async (downloadId: string) => {
    try {
      await api.cancelDownload(downloadId);
      updateDownload(downloadId, { status: 'Cancelled' });
    } catch (error) {
      console.error('Failed to cancel download:', error);
      throw error;
    }
  }, [updateDownload]);

  const batchDownload = useCallback(async (request: any) => {
    try {
      const downloads = await api.batchDownload(request);
      setActiveDownloads([...activeDownloads, ...downloads]);
      return downloads;
    } catch (error) {
      console.error('Failed to start batch download:', error);
      throw error;
    }
  }, [activeDownloads, setActiveDownloads]);

  return {
    activeDownloads,
    startDownload,
    cancelDownload,
    batchDownload,
  };
}