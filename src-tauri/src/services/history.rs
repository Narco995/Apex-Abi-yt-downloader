use anyhow::Result;
use chrono::{DateTime, Utc};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use parking_lot::RwLock;

use crate::models::Download;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadHistory {
    pub downloads: Vec<HistoryEntry>,
    pub total_downloads: u64,
    pub total_bytes_downloaded: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: String,
    pub url: String,
    pub title: String,
    pub filepath: String,
    pub format: String,
    pub quality: String,
    pub downloaded_at: DateTime<Utc>,
    pub file_size: u64,
}

impl Default for DownloadHistory {
    fn default() -> Self {
        Self {
            downloads: Vec::new(),
            total_downloads: 0,
            total_bytes_downloaded: 0,
        }
    }
}

pub struct HistoryService {
    history: Arc<RwLock<DownloadHistory>>,
    history_path: PathBuf,
}

impl HistoryService {
    pub fn new() -> Result<Self> {
        let project_dirs = ProjectDirs::from("com", "AbiYT", "AbiYTDownloader")
            .ok_or_else(|| anyhow::anyhow!("Failed to get project directories"))?;
        
        let data_dir = project_dirs.data_dir();
        fs::create_dir_all(data_dir)?;
        
        let history_path = data_dir.join("history.json");
        
        let history = if history_path.exists() {
            let content = fs::read_to_string(&history_path)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            DownloadHistory::default()
        };
        
        Ok(Self {
            history: Arc::new(RwLock::new(history)),
            history_path,
        })
    }

    pub fn add(&self, download: &Download) -> Result<()> {
        {
            let mut history = self.history.write();
            
            let entry = HistoryEntry {
                id: download.id.clone(),
                url: download.url.clone(),
                title: download.title.clone(),
                filepath: download.filepath.to_string_lossy().to_string(),
                format: download.format.clone(),
                quality: download.quality.clone(),
                downloaded_at: Utc::now(),
                file_size: download.downloaded_bytes,
            };
            
            history.downloads.insert(0, entry);
            history.total_downloads += 1;
            history.total_bytes_downloaded += download.downloaded_bytes;
            
            if history.downloads.len() > 1000 {
                history.downloads.truncate(1000);
            }
        }
        self.save()
    }

    pub fn get(&self) -> DownloadHistory {
        self.history.read().clone()
    }

    pub fn clear(&self) -> Result<()> {
        {
            let mut history = self.history.write();
            history.downloads.clear();
            history.total_downloads = 0;
            history.total_bytes_downloaded = 0;
        }
        self.save()
    }

    pub fn remove(&self, id: &str) -> Result<()> {
        {
            let mut history = self.history.write();
            if let Some(pos) = history.downloads.iter().position(|e| e.id == id) {
                history.downloads.remove(pos);
            }
        }
        self.save()
    }

    fn save(&self) -> Result<()> {
        let history = self.history.read().clone();
        let content = serde_json::to_string_pretty(&history)?;
        fs::write(&self.history_path, content)?;
        Ok(())
    }
}

impl Default for HistoryService {
    fn default() -> Self {
        Self::new().expect("Failed to initialize history service")
    }
}