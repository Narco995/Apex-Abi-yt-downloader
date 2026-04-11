import { useState, useCallback } from 'react';
import { Search, Download, CheckCircle, XCircle } from 'lucide-react';
import { Button } from './Button';
import { Input } from './Input';
import { DownloadCard } from './DownloadCard';
import { useDownloads } from '@/hooks/useDownloads';
import { useAppStore } from '@/stores/useAppStore';
import { api } from '@/utils/tauri';
import { cn } from '@/utils/format';
import type { VideoInfo } from '@/types';

export function Dashboard() {
  const [url, setUrl] = useState('');
  const [selectedFormat, setSelectedFormat] = useState('mp4');
  const [selectedQuality, setSelectedQuality] = useState('high');
  const [isResolving, setIsResolving] = useState(false);
  const [currentVideo, setCurrentVideo] = useState<VideoInfo | null>(null);
  
  const { activeDownloads, startDownload, cancelDownload } = useDownloads();
  const { settings, dependencies } = useAppStore();

  const handleResolve = useCallback(async () => {
    if (!url.trim()) return;
    
    setIsResolving(true);
    try {
      const videoInfo = await api.resolveVideo(url);
      setCurrentVideo(videoInfo);
    } catch (error) {
      console.error('Failed to resolve video:', error);
    } finally {
      setIsResolving(false);
    }
  }, [url]);

  const handleDownload = useCallback(async () => {
    if (!currentVideo || !settings) return;
    
    try {
      await startDownload({
        url: currentVideo.video.url,
        format: selectedFormat,
        quality: selectedQuality,
        output_path: settings.download_path,
      });
      setCurrentVideo(null);
      setUrl('');
    } catch (error) {
      console.error('Failed to start download:', error);
    }
  }, [currentVideo, settings, selectedFormat, selectedQuality, startDownload]);

  const handleCancel = useCallback(async (id: string) => {
    try {
      await cancelDownload(id);
    } catch (error) {
      console.error('Failed to cancel download:', error);
    }
  }, [cancelDownload]);

  const handleOpenFolder = useCallback(async (filepath: string) => {
    // Fix: lastIndexOf('/') breaks on Windows backslash paths.
    // Take the max of both separators to handle cross-platform paths.
    const lastSep = Math.max(filepath.lastIndexOf('/'), filepath.lastIndexOf('\\'));
    const folder = lastSep > -1 ? filepath.substring(0, lastSep) : filepath;
    await api.openFolder(folder);
  }, []);

  const formats = ['mp4', 'webm', 'mkv', 'mp3', 'm4a'];
  const qualities = ['highest', 'high', 'medium', 'low'];

  return (
    <div className="min-h-screen bg-dark-950 text-white p-6">
      <div className="max-w-6xl mx-auto">
        {/* Header */}
        <header className="mb-8">
          <h1 className="text-3xl font-bold bg-gradient-to-r from-primary-400 to-primary-600 bg-clip-text text-transparent">
            Abi YT Downloader
          </h1>
          <p className="text-gray-400 mt-2">Download YouTube videos with ease</p>
        </header>

        {/* Dependencies Status */}
        {dependencies && (
          <div className="mb-6 flex gap-4">
            <div className={cn(
              'flex items-center gap-2 px-3 py-1.5 rounded-full text-sm',
              dependencies.yt_dlp ? 'bg-green-900/50 text-green-400' : 'bg-red-900/50 text-red-400'
            )}>
              {dependencies.yt_dlp ? <CheckCircle className="w-4 h-4" /> : <XCircle className="w-4 h-4" />}
              yt-dlp
            </div>
            <div className={cn(
              'flex items-center gap-2 px-3 py-1.5 rounded-full text-sm',
              dependencies.ffmpeg ? 'bg-green-900/50 text-green-400' : 'bg-red-900/50 text-red-400'
            )}>
              {dependencies.ffmpeg ? <CheckCircle className="w-4 h-4" /> : <XCircle className="w-4 h-4" />}
              ffmpeg
            </div>
          </div>
        )}

        {/* URL Input Section */}
        <div className="bg-dark-900 border border-dark-800 rounded-2xl p-6 mb-6">
          <div className="flex gap-4">
            <div className="flex-1">
              <Input
                placeholder="Paste YouTube URL here..."
                value={url}
                onChange={(e) => setUrl(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && handleResolve()}
              />
            </div>
            <Button
              onClick={handleResolve}
              isLoading={isResolving}
              disabled={!url.trim()}
            >
              <Search className="w-4 h-4 mr-2" />
              Resolve
            </Button>
          </div>

          {/* Video Preview */}
          {currentVideo && (
            <div className="mt-6 p-4 bg-dark-800 rounded-xl">
              <div className="flex gap-4">
                <img
                  src={currentVideo.video.thumbnail}
                  alt={currentVideo.video.title}
                  className="w-48 h-28 object-cover rounded-lg"
                />
                <div className="flex-1">
                  <h3 className="text-lg font-semibold">{currentVideo.video.title}</h3>
                  <p className="text-gray-400 text-sm mt-1">{currentVideo.video.channel}</p>
                  <p className="text-gray-500 text-xs mt-1">
                    Duration: {Math.floor(currentVideo.video.duration / 60)}:{(currentVideo.video.duration % 60).toString().padStart(2, '0')}
                  </p>
                </div>
              </div>

              {/* Format & Quality Selection */}
              <div className="flex gap-4 mt-4">
                <div className="flex-1">
                  <label className="block text-sm text-gray-400 mb-1">Format</label>
                  <select
                    value={selectedFormat}
                    onChange={(e) => setSelectedFormat(e.target.value)}
                    className="w-full px-4 py-2 bg-dark-700 border border-dark-600 rounded-lg text-white"
                  >
                    {formats.map((f) => (
                      <option key={f} value={f}>{f.toUpperCase()}</option>
                    ))}
                  </select>
                </div>
                <div className="flex-1">
                  <label className="block text-sm text-gray-400 mb-1">Quality</label>
                  <select
                    value={selectedQuality}
                    onChange={(e) => setSelectedQuality(e.target.value)}
                    className="w-full px-4 py-2 bg-dark-700 border border-dark-600 rounded-lg text-white"
                  >
                    {qualities.map((q) => (
                      <option key={q} value={q}>{q.charAt(0).toUpperCase() + q.slice(1)}</option>
                    ))}
                  </select>
                </div>
              </div>

              <div className="flex justify-end mt-4">
                <Button onClick={handleDownload}>
                  <Download className="w-4 h-4 mr-2" />
                  Download
                </Button>
              </div>
            </div>
          )}
        </div>

        {/* Active Downloads */}
        <div className="bg-dark-900 border border-dark-800 rounded-2xl p-6">
          <h2 className="text-xl font-semibold mb-4 flex items-center gap-2">
            <Download className="w-5 h-5" />
            Downloads ({activeDownloads.length})
          </h2>

          {activeDownloads.length === 0 ? (
            <div className="text-center py-12 text-gray-500">
              <Download className="w-12 h-12 mx-auto mb-3 opacity-50" />
              <p>No active downloads</p>
              <p className="text-sm mt-1">Paste a YouTube URL above to get started</p>
            </div>
          ) : (
            <div className="space-y-4">
              {activeDownloads.map((download) => (
                <DownloadCard
                  key={download.id}
                  download={download}
                  onCancel={handleCancel}
                  onOpenFolder={handleOpenFolder}
                />
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
