use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(dead_code)] // Resolving/Converting/Paused reserved for future use
pub enum DownloadStatus {
    Pending,
    Resolving,
    Downloading,
    Converting,
    Completed,
    Failed,
    Cancelled,
    Paused,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgress {
    pub id: String,
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
    pub speed: u64,
    pub progress: f32,
    pub eta_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Download {
    pub id: String,
    pub url: String,
    pub title: String,
    pub thumbnail: String,
    pub format: String,
    pub quality: String,
    pub filepath: PathBuf,
    pub status: DownloadStatus,
    pub progress: f32,
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
    pub speed: u64,
    pub eta_seconds: u64,
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl Download {
    pub fn new(url: String, title: String, thumbnail: String, format: String, quality: String, filepath: PathBuf) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            url,
            title,
            thumbnail,
            format,
            quality,
            filepath,
            status: DownloadStatus::Pending,
            progress: 0.0,
            downloaded_bytes: 0,
            total_bytes: 0,
            speed: 0,
            eta_seconds: 0,
            error: None,
            created_at: Utc::now(),
            completed_at: None,
        }
    }
    
    #[allow(dead_code)] // Available for future progress tracking refactor
    pub fn update_progress(&mut self, downloaded: u64, total: u64, speed: u64) {
        self.downloaded_bytes = downloaded;
        self.total_bytes = total;
        self.speed = speed;
        if total > 0 {
            self.progress = (downloaded as f64 / total as f64 * 100.0) as f32;
        }
        if speed > 0 {
            self.eta_seconds = (total.saturating_sub(downloaded)) / speed;
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadRequest {
    pub url: String,
    pub format: String,
    pub quality: String,
    pub output_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDownloadRequest {
    pub urls: Vec<String>,
    pub format: String,
    pub quality: String,
    pub output_path: String,
}