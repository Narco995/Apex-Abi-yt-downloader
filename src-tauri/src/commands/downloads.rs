use std::sync::Arc;
use tauri::{AppHandle, Emitter, State};
use tokio::sync::mpsc;
use parking_lot::RwLock;

use crate::engine::{VideoResolver, DownloadEngine, is_valid_youtube_url};
use crate::models::{Download, DownloadRequest, DownloadProgress, VideoInfo, BatchDownloadRequest};
use crate::services::{SettingsService, HistoryService};

pub struct DownloadManager {
    pub engine: Arc<DownloadEngine>,
    pub resolver: Arc<VideoResolver>,
    pub active_downloads: Arc<RwLock<Vec<Download>>>,
}

impl DownloadManager {
    pub fn new(parallel_limit: usize) -> Self {
        Self {
            engine: Arc::new(DownloadEngine::new(parallel_limit)),
            resolver: Arc::new(VideoResolver::new()),
            active_downloads: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

#[tauri::command]
pub async fn resolve_video(
    url: String,
    manager: State<'_, DownloadManager>,
) -> Result<VideoInfo, String> {
    if !is_valid_youtube_url(&url) {
        return Err("Invalid YouTube URL".to_string());
    }
    
    manager.resolver.resolve(&url).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn start_download(
    request: DownloadRequest,
    manager: State<'_, DownloadManager>,
    settings: State<'_, SettingsService>,
    history: State<'_, HistoryService>,
    app: AppHandle,
) -> Result<Download, String> {
    let settings = settings.get();
    let mut request = request.clone();
    
    if request.output_path.is_empty() {
        request.output_path = settings.download_path;
    }

    // Fix: generate the download ID upfront so active_downloads entry and engine share the same ID.
    // Previously, Download::new() in active_downloads and the engine each generated separate UUIDs,
    // making progress events untrackable (they'd never find the right entry in active_downloads).
    let download_id = uuid::Uuid::new_v4().to_string();
    
    let (progress_tx, mut progress_rx) = mpsc::channel::<DownloadProgress>(100);
    
    // Add to active downloads with pre-generated ID
    {
        let mut downloads = manager.active_downloads.write();
        let mut new_download = Download::new(
            request.url.clone(),
            String::new(),
            String::new(),
            request.format.clone(),
            request.quality.clone(),
            std::path::PathBuf::from(&request.output_path),
        );
        new_download.id = download_id.clone();
        downloads.push(new_download);
    }
    
    // Spawn progress reporter
    let app_clone = app.clone();
    let downloads_clone = manager.active_downloads.clone();
    tokio::spawn(async move {
        while let Some(progress) = progress_rx.recv().await {
            let _ = app_clone.emit("download-progress", &progress);
            
            // Update active download — now correctly finds the entry since IDs match
            let mut downloads = downloads_clone.write();
            if let Some(d) = downloads.iter_mut().find(|d| d.id == progress.id) {
                d.progress = progress.progress;
                d.downloaded_bytes = progress.downloaded_bytes;
                d.total_bytes = progress.total_bytes;
                d.speed = progress.speed;
                d.eta_seconds = progress.eta_seconds;
            }
        }
    });
    
    // Pass the pre-generated ID to the engine
    let result = manager.engine.download(request, progress_tx, Some(download_id)).await;
    
    match result {
        Ok(download) => {
            let _ = app.emit("download-complete", &download);
            
            if download.status == crate::models::DownloadStatus::Completed {
                let _ = history.add(&download);
            }
            
            // Remove from active downloads
            {
                let mut downloads = manager.active_downloads.write();
                downloads.retain(|d| d.id != download.id);
            }
            
            Ok(download)
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn cancel_download(
    download_id: String,
    manager: State<'_, DownloadManager>,
) -> Result<bool, String> {
    Ok(manager.engine.cancel_download(&download_id))
}

#[tauri::command]
pub fn get_active_downloads(
    manager: State<'_, DownloadManager>,
) -> Result<Vec<Download>, String> {
    Ok(manager.active_downloads.read().clone())
}

#[tauri::command]
pub async fn batch_download(
    request: BatchDownloadRequest,
    manager: State<'_, DownloadManager>,
    settings: State<'_, SettingsService>,
    history: State<'_, HistoryService>,
    app: AppHandle,
) -> Result<Vec<Download>, String> {
    let base_settings = settings.get();

    // Fix: was a sequential for-loop with .await — all downloads blocked each other.
    // Spawn a task per URL; the semaphore in DownloadEngine gates actual parallelism.
    let tasks: Vec<_> = request.urls.into_iter().map(|url| {
        let output_path = if request.output_path.is_empty() {
            base_settings.download_path.clone()
        } else {
            request.output_path.clone()
        };
        let single_request = DownloadRequest {
            url,
            format: request.format.clone(),
            quality: request.quality.clone(),
            output_path,
        };
        let engine = manager.engine.clone();
        let downloads_clone = manager.active_downloads.clone();
        let app_clone = app.clone();

        tokio::spawn(async move {
            let download_id = uuid::Uuid::new_v4().to_string();
            let (progress_tx, mut progress_rx) = mpsc::channel::<DownloadProgress>(100);

            let app_prog = app_clone.clone();
            let dl_clone = downloads_clone.clone();
            tokio::spawn(async move {
                while let Some(progress) = progress_rx.recv().await {
                    let _ = app_prog.emit("download-progress", &progress);
                    let mut downloads = dl_clone.write();
                    if let Some(d) = downloads.iter_mut().find(|d| d.id == progress.id) {
                        d.progress = progress.progress;
                        d.downloaded_bytes = progress.downloaded_bytes;
                        d.total_bytes = progress.total_bytes;
                        d.speed = progress.speed;
                        d.eta_seconds = progress.eta_seconds;
                    }
                }
            });

            match engine.download(single_request, progress_tx, Some(download_id)).await {
                Ok(download) => {
                    let _ = app_clone.emit("download-complete", &download);
                    Ok(download)
                }
                Err(e) => {
                    tracing::error!("Batch download error: {}", e);
                    Err(e.to_string())
                }
            }
        })
    }).collect();

    // Collect all results, then record history (HistoryService not Clone so can't move into tasks)
    let results: Vec<Download> = futures_util::future::join_all(tasks)
        .await
        .into_iter()
        .filter_map(|join_result| join_result.ok())
        .filter_map(|download_result| download_result.ok())
        .collect();

    for download in &results {
        if download.status == crate::models::DownloadStatus::Completed {
            let _ = history.add(download);
        }
    }

    Ok(results)
}
