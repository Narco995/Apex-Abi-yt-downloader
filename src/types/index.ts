export interface Settings {
  download_path: string;
  parallel_downloads: number;
  max_retries: number;
  auto_convert: boolean;
  preferred_format: VideoFormat;
  quality_preference: QualityPreference;
  skip_existing: boolean;
  filename_template: string;
  theme: string;
  language: string;
}

export type VideoFormat = 'MP4' | 'WEBM' | 'MKV' | 'MP3' | 'M4A';

export type QualityPreference = 'Highest' | 'High' | 'Medium' | 'Low' | `Custom:${string}`;

export interface Video {
  id: string;
  url: string;
  title: string;
  thumbnail: string;
  duration: number;
  channel: string;
  view_count?: number;
  upload_date?: string;
  description?: string;
}

export interface VideoQuality {
  itag: number;
  format_id: string;
  format_note: string;
  extension: string;
  resolution: string;
  fps?: number;
  vcodec: string;
  acodec: string;
  filesize?: number;
  is_video_only: boolean;
  is_audio_only: boolean;
}

export interface VideoInfo {
  video: Video;
  qualities: VideoQuality[];
  best_quality?: VideoQuality;
  audio_only_qualities: VideoQuality[];
  video_qualities: VideoQuality[];
}

export interface Download {
  id: string;
  url: string;
  title: string;
  thumbnail: string;
  format: string;
  quality: string;
  filepath: string;
  status: DownloadStatus;
  progress: number;
  downloaded_bytes: number;
  total_bytes: number;
  speed: number;
  eta_seconds: number;
  error?: string;
  created_at: string;
  completed_at?: string;
}

export type DownloadStatus = 
  | 'Pending'
  | 'Resolving'
  | 'Downloading'
  | 'Converting'
  | 'Completed'
  | 'Failed'
  | 'Cancelled'
  | 'Paused';

export interface DownloadProgress {
  id: string;
  downloaded_bytes: number;
  total_bytes: number;
  speed: number;
  progress: number;
  eta_seconds: number;
}

export interface DownloadRequest {
  url: string;
  format: string;
  quality: string;
  output_path: string;
}

export interface BatchDownloadRequest {
  urls: string[];
  format: string;
  quality: string;
  output_path: string;
}

export interface HistoryEntry {
  id: string;
  url: string;
  title: string;
  filepath: string;
  format: string;
  quality: string;
  downloaded_at: string;
  file_size: number;
}

export interface DownloadHistory {
  downloads: HistoryEntry[];
  total_downloads: number;
  total_bytes_downloaded: number;
}

export interface DependenciesStatus {
  yt_dlp: boolean;
  ffmpeg: boolean;
}