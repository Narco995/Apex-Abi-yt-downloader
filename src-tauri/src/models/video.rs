use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    pub id: String,
    pub url: String,
    pub title: String,
    pub thumbnail: String,
    pub duration: u32,
    pub channel: String,
    pub view_count: Option<u64>,
    pub upload_date: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoQuality {
    pub itag: u32,
    pub format_id: String,
    pub format_note: String,
    pub extension: String,
    pub resolution: String,
    pub fps: Option<u32>,
    pub vcodec: String,
    pub acodec: String,
    pub filesize: Option<u64>,
    pub is_video_only: bool,
    pub is_audio_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoInfo {
    pub video: Video,
    pub qualities: Vec<VideoQuality>,
    pub best_quality: Option<VideoQuality>,
    pub audio_only_qualities: Vec<VideoQuality>,
    pub video_qualities: Vec<VideoQuality>,
}

impl VideoInfo {
    pub fn new(video: Video, qualities: Vec<VideoQuality>) -> Self {
        let audio_only: Vec<_> = qualities.iter().filter(|q| q.is_audio_only).cloned().collect();
        let video_qualities: Vec<_> = qualities.iter().filter(|q| !q.is_audio_only).cloned().collect();
        let best = video_qualities.first().cloned();
        
        Self {
            video,
            qualities,
            best_quality: best,
            audio_only_qualities: audio_only,
            video_qualities,
        }
    }
}