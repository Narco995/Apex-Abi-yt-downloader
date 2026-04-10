use anyhow::{anyhow, Result};
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::{mpsc, Semaphore};
use tokio_util::sync::CancellationToken;

use crate::models::{Download, DownloadProgress, DownloadRequest, DownloadStatus};

pub struct DownloadEngine {
    semaphore: Arc<Semaphore>,
    cancel_tokens: Arc<parking_lot::RwLock<std::collections::HashMap<String, CancellationToken>>>,
}

impl DownloadEngine {
    pub fn new(parallel_limit: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(parallel_limit.max(1))),
            cancel_tokens: Arc::new(parking_lot::RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Fix: accepts an optional pre-generated download_id so callers can track the download
    /// in active_downloads before the engine starts (previously IDs mismatched, breaking progress).
    pub async fn download(
        &self,
        request: DownloadRequest,
        progress_tx: mpsc::Sender<DownloadProgress>,
        download_id: Option<String>,
    ) -> Result<Download> {
        let _permit = self.semaphore.acquire().await.map_err(|e| anyhow!("Semaphore error: {}", e))?;
        
        let cancel_token = CancellationToken::new();
        // Use the caller-provided ID or generate a new one
        let download_id = download_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        
        {
            let mut tokens = self.cancel_tokens.write();
            tokens.insert(download_id.clone(), cancel_token.clone());
        }

        let result = self.execute_download(&download_id, request, progress_tx, cancel_token).await;
        
        {
            let mut tokens = self.cancel_tokens.write();
            tokens.remove(&download_id);
        }

        result
    }

    async fn execute_download(
        &self,
        download_id: &str,
        request: DownloadRequest,
        progress_tx: mpsc::Sender<DownloadProgress>,
        cancel_token: CancellationToken,
    ) -> Result<Download> {
        let output_template = PathBuf::from(&request.output_path)
            .join("%(title)s.%(ext)s")
            .to_string_lossy()
            .to_string();

        let mut cmd = Command::new("yt-dlp");
        cmd.args([
            "--newline",
            "--progress",
            "--progress-template",
            "%(progress._percent_str)s|%(progress._speed_str)s|%(progress._eta_str)s|%(progress.downloaded_bytes)s|%(progress.total_bytes)s",
            "-f",
            &self.format_selector(&request.quality, &request.format),
            "-o",
            &output_template,
            "--no-mtime",
            "--no-playlist",
            &request.url,
        ]);

        if request.format == "mp3" || request.format == "m4a" {
            cmd.args(["-x", "--audio-format", &request.format]);
        }

        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let mut child = cmd.spawn().map_err(|e| anyhow!("Failed to spawn yt-dlp: {}", e))?;
        
        let stdout = child.stdout.take().ok_or_else(|| anyhow!("Failed to capture stdout"))?;
        let stderr = child.stderr.take().ok_or_else(|| anyhow!("Failed to capture stderr"))?;

        // Fix: drain stderr in a background task to prevent pipe buffer deadlock.
        // Previously stderr was captured but never read — if yt-dlp wrote enough to fill
        // the OS pipe buffer, the child process would block forever.
        tokio::spawn(async move {
            let mut reader = BufReader::new(stderr).lines();
            tokio::pin!(reader);
            while let Ok(Some(_line)) = reader.next_line().await {
                // Could log here if needed: tracing::debug!("yt-dlp stderr: {}", _line);
            }
        });

        let download_id_owned = download_id.to_string();
        let progress_tx_clone = progress_tx.clone();
        let cancel_token_clone = cancel_token.clone();
        
        let stdout_task = tokio::spawn(async move {
            let reader = BufReader::new(stdout).lines();
            let mut last_progress = DownloadProgress {
                id: download_id_owned.clone(),
                downloaded_bytes: 0,
                total_bytes: 0,
                speed: 0,
                progress: 0.0,
                eta_seconds: 0,
            };

            tokio::pin!(reader);
            while let Some(line) = reader.next_line().await.transpose() {
                if cancel_token_clone.is_cancelled() {
                    break;
                }
                
                if let Ok(line) = line {
                    if let Some(progress) = parse_progress_line(&line, &last_progress) {
                        last_progress = progress.clone();
                        let _ = progress_tx_clone.send(progress).await;
                    }
                }
            }
        });

        let mut download = Download::new(
            request.url.clone(),
            String::new(),
            String::new(),
            request.format.clone(),
            request.quality.clone(),
            PathBuf::from(&request.output_path),
        );
        // Fix: use the pre-agreed download_id (was generating a new UUID here, mismatching callers)
        download.id = download_id.to_string();
        download.status = DownloadStatus::Downloading;

        let status = child.wait().await?;
        
        stdout_task.abort();

        if cancel_token.is_cancelled() {
            download.status = DownloadStatus::Cancelled;
            return Ok(download);
        }

        if status.success() {
            download.status = DownloadStatus::Completed;
            download.progress = 100.0;
        } else {
            download.status = DownloadStatus::Failed;
            download.error = Some("Download failed".to_string());
        }

        Ok(download)
    }

    pub fn cancel_download(&self, download_id: &str) -> bool {
        let tokens = self.cancel_tokens.read();
        if let Some(token) = tokens.get(download_id) {
            token.cancel();
            true
        } else {
            false
        }
    }

    pub fn update_parallel_limit(&self, new_limit: usize) {
        self.semaphore.add_permits(new_limit);
    }

    fn format_selector(&self, quality: &str, format: &str) -> String {
        match (quality, format) {
            ("highest", _) => "bestvideo+bestaudio/best".to_string(),
            ("high", "mp4") => "bestvideo[height<=1080][ext=mp4]+bestaudio[ext=m4a]/best[height<=1080][ext=mp4]".to_string(),
            ("high", _) => "bestvideo[height<=1080]+bestaudio/best[height<=1080]".to_string(),
            ("medium", "mp4") => "bestvideo[height<=720][ext=mp4]+bestaudio[ext=m4a]/best[height<=720][ext=mp4]".to_string(),
            ("medium", _) => "bestvideo[height<=720]+bestaudio/best[height<=720]".to_string(),
            ("low", "mp4") => "bestvideo[height<=480][ext=mp4]+bestaudio[ext=m4a]/best[height<=480][ext=mp4]".to_string(),
            ("low", _) => "bestvideo[height<=480]+bestaudio/best[height<=480]".to_string(),
            (custom, _) => custom.to_string(),
        }
    }
}

fn parse_progress_line(line: &str, last: &DownloadProgress) -> Option<DownloadProgress> {
    let parts: Vec<&str> = line.split('|').collect();
    
    if parts.len() >= 5 {
        let progress = parts[0].trim().trim_end_matches('%').parse().unwrap_or(last.progress);
        let speed = parse_speed(parts[1].trim());
        let eta = parse_eta(parts[2].trim());
        let downloaded = parts[3].trim().parse().unwrap_or(last.downloaded_bytes);
        let total = parts[4].trim().parse().unwrap_or(last.total_bytes);
        
        Some(DownloadProgress {
            id: last.id.clone(),
            progress,
            speed,
            eta_seconds: eta,
            downloaded_bytes: downloaded,
            total_bytes: total,
        })
    } else {
        None
    }
}

fn parse_speed(s: &str) -> u64 {
    let s = s.trim();
    if s.contains("KiB") {
        s.replace("KiB/s", "").trim().parse().unwrap_or(0) * 1024
    } else if s.contains("MiB") {
        (s.replace("MiB/s", "").trim().parse::<f64>().unwrap_or(0.0) * 1024.0 * 1024.0) as u64
    } else if s.contains("GiB") {
        (s.replace("GiB/s", "").trim().parse::<f64>().unwrap_or(0.0) * 1024.0 * 1024.0 * 1024.0) as u64
    } else {
        s.parse().unwrap_or(0)
    }
}

fn parse_eta(s: &str) -> u64 {
    if s.trim() == "N/A" || s.trim().is_empty() {
        return 0;
    }
    
    let parts: Vec<&str> = s.trim().split(':').collect();
    match parts.len() {
        1 => parts[0].parse().unwrap_or(0),
        2 => parts[0].parse::<u64>().unwrap_or(0) * 60 + parts[1].parse().unwrap_or(0),
        3 => {
            parts[0].parse::<u64>().unwrap_or(0) * 3600
                + parts[1].parse::<u64>().unwrap_or(0) * 60
                + parts[2].parse().unwrap_or(0)
        }
        _ => 0,
    }
}
