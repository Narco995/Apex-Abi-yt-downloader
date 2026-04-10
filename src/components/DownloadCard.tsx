import { useState } from 'react';
import { X, Play, Pause, Folder, RotateCcw } from 'lucide-react';
import type { Download } from '@/types';
import { Button } from './Button';
import { ProgressBar } from './ProgressBar';
import { formatFileSize, formatSpeed, formatEta, cn } from '@/utils/format';

interface DownloadCardProps {
  download: Download;
  onCancel?: (id: string) => void;
  onRetry?: (id: string) => void;
  onOpenFolder?: (path: string) => void;
}

export function DownloadCard({ download, onCancel, onRetry, onOpenFolder }: DownloadCardProps) {
  const statusColors: Record<string, string> = {
    Pending: 'bg-yellow-500',
    Resolving: 'bg-blue-500',
    Downloading: 'bg-primary-500',
    Converting: 'bg-purple-500',
    Completed: 'bg-green-500',
    Failed: 'bg-red-500',
    Cancelled: 'bg-gray-500',
    Paused: 'bg-orange-500',
  };

  const progressVariant = download.status === 'Failed' ? 'danger' : 
    download.status === 'Completed' ? 'success' : 'default';

  return (
    <div className="bg-dark-800 border border-dark-700 rounded-xl p-4 hover:border-dark-600 transition-all duration-200">
      <div className="flex gap-4">
        {download.thumbnail ? (
          <img 
            src={download.thumbnail} 
            alt={download.title}
            className="w-32 h-20 object-cover rounded-lg flex-shrink-0"
          />
        ) : (
          <div className="w-32 h-20 bg-dark-700 rounded-lg flex-shrink-0 flex items-center justify-center">
            <Play className="w-8 h-8 text-gray-500" />
          </div>
        )}
        
        <div className="flex-1 min-w-0">
          <div className="flex items-start justify-between gap-2">
            <div className="min-w-0">
              <h3 className="text-white font-medium truncate">{download.title || 'Unknown'}</h3>
              <p className="text-sm text-gray-400 truncate">{download.format} • {download.quality}</p>
            </div>
            
            <div className="flex items-center gap-1">
              <span className={cn(
                'px-2 py-1 text-xs font-medium rounded-full',
                statusColors[download.status] || 'bg-gray-500'
              )}>
                {download.status}
              </span>
            </div>
          </div>
          
          {download.status === 'Downloading' && (
            <div className="mt-2">
              <ProgressBar progress={download.progress} variant={progressVariant} />
              <div className="flex justify-between text-xs text-gray-500 mt-1">
                <span>{formatFileSize(download.downloaded_bytes)} / {formatFileSize(download.total_bytes)}</span>
                <span>{formatSpeed(download.speed)}</span>
                <span>ETA: {formatEta(download.eta_seconds)}</span>
              </div>
            </div>
          )}
          
          {download.error && (
            <p className="mt-2 text-sm text-red-400">{download.error}</p>
          )}
        </div>
      </div>
      
      <div className="flex justify-end gap-2 mt-3 pt-3 border-t border-dark-700">
        {download.status === 'Completed' && onOpenFolder && (
          <Button
            size="sm"
            variant="ghost"
            onClick={() => onOpenFolder(download.filepath)}
          >
            <Folder className="w-4 h-4 mr-1" />
            Open Folder
          </Button>
        )}
        
        {download.status === 'Failed' && onRetry && (
          <Button
            size="sm"
            variant="secondary"
            onClick={() => onRetry(download.id)}
          >
            <RotateCcw className="w-4 h-4 mr-1" />
            Retry
          </Button>
        )}
        
        {(download.status === 'Downloading' || download.status === 'Pending') && onCancel && (
          <Button
            size="sm"
            variant="danger"
            onClick={() => onCancel(download.id)}
          >
            <X className="w-4 h-4 mr-1" />
            Cancel
          </Button>
        )}
      </div>
    </div>
  );
}