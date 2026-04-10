use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub download_path: String,
    pub parallel_downloads: usize,
    pub max_retries: u32,
    pub auto_convert: bool,
    pub preferred_format: VideoFormat,
    pub quality_preference: QualityPreference,
    pub skip_existing: bool,
    pub filename_template: String,
    pub theme: String,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VideoFormat {
    MP4,
    WEBM,
    MKV,
    MP3,
    M4A,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QualityPreference {
    Highest,
    High,
    Medium,
    Low,
    Custom(String),
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            download_path: dirs::download_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .to_string_lossy()
                .to_string(),
            parallel_downloads: 2,
            max_retries: 3,
            auto_convert: false,
            preferred_format: VideoFormat::MP4,
            quality_preference: QualityPreference::High,
            skip_existing: true,
            filename_template: "%(title)s.%(ext)s".to_string(),
            theme: "dark".to_string(),
            language: "en".to_string(),
        }
    }
}