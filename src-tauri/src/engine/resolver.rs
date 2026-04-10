use anyhow::{anyhow, Result};
use regex::Regex;
use serde_json::Value;
use std::process::Command;

use crate::models::{Video, VideoInfo, VideoQuality};

pub struct VideoResolver;

impl VideoResolver {
    pub fn new() -> Self {
        Self
    }

    pub async fn resolve(&self, url: &str) -> Result<VideoInfo> {
        let output = Command::new("yt-dlp")
            .args([
                "--dump-json",
                "--no-download",
                "--no-warnings",
                "--flat-playlist",
                url,
            ])
            .output()
            .map_err(|e| anyhow!("Failed to execute yt-dlp: {}. Make sure yt-dlp is installed.", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("yt-dlp error: {}", stderr));
        }

        let json: Value = serde_json::from_slice(&output.stdout)?;
        
        let video = self.parse_video_info(&json, url)?;
        let qualities = self.parse_qualities(&json)?;
        
        Ok(VideoInfo::new(video, qualities))
    }

    pub async fn resolve_batch(&self, urls: &[String]) -> Vec<Result<VideoInfo>> {
        futures_util::future::join_all(
            urls.iter()
                .map(|url| self.resolve(url))
                .collect::<Vec<_>>()
        )
        .await
        }

    fn parse_video_info(&self, json: &Value, url: &str) -> Result<Video> {
        Ok(Video {
            id: json["id"].as_str().unwrap_or("unknown").to_string(),
            url: url.to_string(),
            title: json["title"].as_str().unwrap_or("Unknown Title").to_string(),
            thumbnail: json["thumbnail"]
                .as_str()
                .or(json["thumbnails"][0]["url"].as_str())
                .unwrap_or("")
                .to_string(),
            duration: json["duration"].as_u64().unwrap_or(0) as u32,
            channel: json["uploader"]
                .or(json["channel"])
                .as_str()
                .unwrap_or("Unknown")
                .to_string(),
            view_count: json["view_count"].as_u64(),
            upload_date: json["upload_date"].as_str().map(|s| s.to_string()),
            description: json["description"].as_str().map(|s| s.to_string()),
        })
    }

    fn parse_qualities(&self, json: &Value) -> Result<Vec<VideoQuality>> {
        let mut qualities = Vec::new();
        
        if let Some(formats) = json.get("formats").and_then(|f| f.as_array()) {
            for format in formats {
                let quality = VideoQuality {
                    itag: format["format_id"].as_str()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(0),
                    format_id: format["format_id"].as_str().unwrap_or("unknown").to_string(),
                    format_note: format["format_note"].as_str().unwrap_or("").to_string(),
                    extension: format["ext"].as_str().unwrap_or("mp4").to_string(),
                    resolution: format["resolution"].as_str().unwrap_or("unknown").to_string(),
                    fps: format["fps"].as_u64().map(|f| f as u32),
                    vcodec: format["vcodec"].as_str().unwrap_or("none").to_string(),
                    acodec: format["acodec"].as_str().unwrap_or("none").to_string(),
                    filesize: format["filesize"]
                        .or(format["filesize_approx"])
                        .and_then(|v| v.as_u64()),
                    is_video_only: format["vcodec"].as_str().map(|s| s != "none").unwrap_or(false) 
                        && format["acodec"].as_str().map(|s| s == "none").unwrap_or(true),
                    is_audio_only: format["acodec"].as_str().map(|s| s != "none").unwrap_or(false)
                        && format["vcodec"].as_str().map(|s| s == "none").unwrap_or(true),
                };
                
                if quality.format_id != "unknown" {
                    qualities.push(quality);
                }
            }
        }
        
        qualities.sort_by(|a, b| {
            let a_res = self.parse_resolution(&a.resolution);
            let b_res = self.parse_resolution(&b.resolution);
            b_res.cmp(&a_res)
        });
        
        Ok(qualities)
    }

    fn parse_resolution(&self, resolution: &str) -> u32 {
        resolution
            .split('x')
            .last()
            .and_then(|s| s.trim().parse().ok())
            .unwrap_or(0)
    }
}

impl Default for VideoResolver {
    fn default() -> Self {
        Self::new()
    }
}

pub fn is_valid_youtube_url(url: &str) -> bool {
    let patterns = [
        r"(?:https?://)?(?:www\.)?youtube\.com/watch\?v=[\w-]+",
        r"(?:https?://)?(?:www\.)?youtu\.be/[\w-]+",
        r"(?:https?://)?(?:www\.)?youtube\.com/shorts/[\w-]+",
        r"(?:https?://)?(?:www\.)?youtube\.com/playlist\?list=[\w-]+",
        r"(?:https?://)?(?:m\.)?youtube\.com/watch\?v=[\w-]+",
    ];
    
    patterns.iter().any(|pattern| {
        Regex::new(pattern)
            .map(|re| re.is_match(url))
            .unwrap_or(false)
    })
}