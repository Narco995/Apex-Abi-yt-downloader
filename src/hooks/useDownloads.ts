import { useEffect, useCallback } from 'react';
import { useAppStore } from '@/stores/useAppStore';
import { api, listenToProgress, listenToComplete } from '@/utils/tauri';
import type { DownloadProgress, Download } from '@/types';

export function useDownloads() {
  const { activeDownloads, setActiveDownloads, updateDownload, fetchDependencies, addDownload, addDownloads } = useAppStore();

  // Defined BEFORE useEffect to avoid temporal dead zone (TDZ) — TypeScript strict
  // mode flags block-scoped variables used before their lexical declaration.
  const loadActiveDownloads = useCallback(async () => {
    try {
      const downloads = await api.getActiveDownloads();
      setActiveDownloads(downloads);
    } catch (error) {
      console.error('Failed to load active downloads:', error);
    }
  }, [setActiveDownloads]);

  useEffect(() => {
    fetchDependencies();
    loadActiveDownloads();

    // Capture unlisten functions and call them on cleanup to prevent listener
    // stacking on every mount (especially in React Strict Mode).
    let unlistenProgress: (() => void) | undefined;
    let unlistenComplete: (() => void) | undefined;

    listenToProgress((progress: DownloadProgress) => {
      updateDownload(progress.id, {
        progress: progress.progress,
        downloaded_bytes: progress.downloaded_bytes,
        total_bytes: progress.total_bytes,
        speed: progress.speed,
        eta_seconds: progress.eta_seconds,
      });
    }).then((unlisten) => {
      unlistenProgress = unlisten;
    });

    listenToComplete((download: Download) => {
      updateDownload(download.id, {
        status: download.status,
        progress: download.progress,
        error: download.error,
        completed_at: download.completed_at,
      });
    }).then((unlisten) => {
      unlistenComplete = unlisten;
    });

    return () => {
      unlistenProgress?.();
      unlistenComplete?.();
    };
  }, [fetchDependencies, loadActiveDownloads, updateDownload]);

  const startDownload = useCallback(async (request: DownloadRequest) => {
    try {
      const download = await api.startDownload(request);
      addDownload(download);
      return download;
    } catch (error) {
      console.error('Failed to start download:', error);
      throw error;
    }
  }, [addDownload]);

  const cancelDownload = useCallback(async (downloadId: string) => {
    try {
      await api.cancelDownload(downloadId);
      updateDownload(downloadId, { status: 'Cancelled' });
    } catch (error) {
      console.error('Failed to cancel download:', error);
      throw error;
    }
  }, [updateDownload]);

  const batchDownload = useCallback(async (request: BatchDownloadRequest) => {
    try {
      const downloads = await api.batchDownload(request);
      addDownloads(downloads);
      return downloads;
    } catch (error) {
      console.error('Failed to start batch download:', error);
      throw error;
    }
  }, [addDownloads]);

  return {
    activeDownloads,
    startDownload,
    cancelDownload,
    batchDownload,
  };
}

// Local type aliases to satisfy TypeScript without importing from types (already imported above)
type DownloadRequest = import('@/types').DownloadRequest;
type BatchDownloadRequest = import('@/types').BatchDownloadRequest;
