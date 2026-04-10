use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Converter;

impl Converter {
    pub fn new() -> Self {
        Self
    }

    pub fn convert(&self, input: &Path, output_format: &str, output_path: Option<&Path>) -> Result<PathBuf> {
        let output = output_path
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| {
                input.with_extension(output_format)
            });

        let status = Command::new("ffmpeg")
            .args([
                "-i", &input.to_string_lossy(),
                "-y",
                "-vn",
                "-acodec", Self::get_audio_codec(output_format),
                &output.to_string_lossy(),
            ])
            .status()
            .map_err(|e| anyhow!("FFmpeg error: {}. Make sure FFmpeg is installed.", e))?;

        if status.success() {
            Ok(output)
        } else {
            Err(anyhow!("Conversion failed"))
        }
    }

    fn get_audio_codec(format: &str) -> &'static str {
        match format {
            "mp3" => "libmp3lame",
            "m4a" => "aac",
            "opus" => "libopus",
            "wav" => "pcm_s16le",
            "flac" => "flac",
            _ => "copy",
        }
    }

    pub fn is_ffmpeg_available(&self) -> bool {
        Command::new("ffmpeg")
            .arg("-version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    pub fn merge_video_audio(&self, video: &Path, audio: &Path, output: &Path) -> Result<()> {
        let status = Command::new("ffmpeg")
            .args([
                "-i", &video.to_string_lossy(),
                "-i", &audio.to_string_lossy(),
                "-c", "copy",
                "-y",
                &output.to_string_lossy(),
            ])
            .status()
            .map_err(|e| anyhow!("FFmpeg merge error: {}", e))?;

        if status.success() {
            Ok(())
        } else {
            Err(anyhow!("Failed to merge video and audio"))
        }
    }
}

impl Default for Converter {
    fn default() -> Self {
        Self::new()
    }
}