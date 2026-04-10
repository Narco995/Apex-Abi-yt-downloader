import { useState, useEffect } from 'react';
import { X, Save, RotateCcw, Folder } from 'lucide-react';
import { Button } from './Button';
import { Input } from './Input';
import { useAppStore } from '@/stores/useAppStore';
import { api } from '@/utils/tauri';
import { open } from '@tauri-apps/plugin-dialog';

interface SettingsModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export function SettingsModal({ isOpen, onClose }: SettingsModalProps) {
  const { settings, updateSettings, fetchSettings } = useAppStore();
  const [localSettings, setLocalSettings] = useState(settings);
  const [isSaving, setIsSaving] = useState(false);

  useEffect(() => {
    if (isOpen) {
      fetchSettings();
    }
  }, [isOpen, fetchSettings]);

  useEffect(() => {
    setLocalSettings(settings);
  }, [settings]);

  const handleSave = async () => {
    if (!localSettings) return;
    
    setIsSaving(true);
    try {
      await updateSettings(localSettings);
      onClose();
    } catch (error) {
      console.error('Failed to save settings:', error);
    } finally {
      setIsSaving(false);
    }
  };

  const handleReset = async () => {
    try {
      await api.resetSettings();
      await fetchSettings();
    } catch (error) {
      console.error('Failed to reset settings:', error);
    }
  };

  const handleBrowseFolder = async () => {
    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: localSettings?.download_path,
    });
    
    if (selected) {
      setLocalSettings(prev => prev ? { ...prev, download_path: selected as string } : prev);
    }
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm">
      <div className="bg-dark-900 border border-dark-700 rounded-2xl w-full max-w-2xl max-h-[80vh] overflow-hidden">
        <div className="flex items-center justify-between p-4 border-b border-dark-700">
          <h2 className="text-xl font-semibold">Settings</h2>
          <Button variant="ghost" size="sm" onClick={onClose}>
            <X className="w-5 h-5" />
          </Button>
        </div>
        
        <div className="p-4 overflow-y-auto max-h-[60vh]">
          {localSettings && (
            <div className="space-y-6">
              {/* Download Path */}
              <div>
                <label className="block text-sm font-medium text-gray-300 mb-2">
                  Download Location
                </label>
                <div className="flex gap-2">
                  <Input
                    value={localSettings.download_path}
                    onChange={(e) => setLocalSettings({ ...localSettings, download_path: e.target.value })}
                    className="flex-1"
                  />
                  <Button variant="secondary" onClick={handleBrowseFolder}>
                    <Folder className="w-4 h-4" />
                  </Button>
                </div>
              </div>

              {/* Parallel Downloads */}
              <div>
                <label className="block text-sm font-medium text-gray-300 mb-2">
                  Parallel Downloads: {localSettings.parallel_downloads}
                </label>
                <input
                  type="range"
                  min="1"
                  max="10"
                  value={localSettings.parallel_downloads}
                  onChange={(e) => setLocalSettings({ ...localSettings, parallel_downloads: parseInt(e.target.value) })}
                  className="w-full accent-primary-500"
                />
              </div>

              {/* Max Retries */}
              <div>
                <label className="block text-sm font-medium text-gray-300 mb-2">
                  Max Retries
                </label>
                <Input
                  type="number"
                  min="0"
                  max="10"
                  value={localSettings.max_retries}
                  onChange={(e) => setLocalSettings({ ...localSettings, max_retries: parseInt(e.target.value) })}
                />
              </div>

              {/* Format */}
              <div>
                <label className="block text-sm font-medium text-gray-300 mb-2">
                  Preferred Format
                </label>
                <select
                  value={localSettings.preferred_format}
                  onChange={(e) => setLocalSettings({ ...localSettings, preferred_format: e.target.value as any })}
                  className="w-full px-4 py-2 bg-dark-800 border border-dark-600 rounded-lg text-white"
                >
                  <option value="MP4">MP4</option>
                  <option value="WEBM">WebM</option>
                  <option value="MKV">MKV</option>
                  <option value="MP3">MP3 (Audio Only)</option>
                  <option value="M4A">M4A (Audio Only)</option>
                </select>
              </div>

              {/* Quality */}
              <div>
                <label className="block text-sm font-medium text-gray-300 mb-2">
                  Quality Preference
                </label>
                <select
                  value={localSettings.quality_preference}
                  onChange={(e) => setLocalSettings({ ...localSettings, quality_preference: e.target.value as any })}
                  className="w-full px-4 py-2 bg-dark-800 border border-dark-600 rounded-lg text-white"
                >
                  <option value="Highest">Highest</option>
                  <option value="High">High (1080p)</option>
                  <option value="Medium">Medium (720p)</option>
                  <option value="Low">Low (480p)</option>
                </select>
              </div>

              {/* Checkboxes */}
              <div className="space-y-3">
                <label className="flex items-center gap-3 cursor-pointer">
                  <input
                    type="checkbox"
                    checked={localSettings.skip_existing}
                    onChange={(e) => setLocalSettings({ ...localSettings, skip_existing: e.target.checked })}
                    className="w-5 h-5 rounded bg-dark-700 border-dark-600 text-primary-500 focus:ring-primary-500"
                  />
                  <span className="text-gray-300">Skip existing files</span>
                </label>
                
                <label className="flex items-center gap-3 cursor-pointer">
                  <input
                    type="checkbox"
                    checked={localSettings.auto_convert}
                    onChange={(e) => setLocalSettings({ ...localSettings, auto_convert: e.target.checked })}
                    className="w-5 h-5 rounded bg-dark-700 border-dark-600 text-primary-500 focus:ring-primary-500"
                  />
                  <span className="text-gray-300">Auto-convert to preferred format</span>
                </label>
              </div>

              {/* Filename Template */}
              <div>
                <label className="block text-sm font-medium text-gray-300 mb-2">
                  Filename Template
                </label>
                <Input
                  value={localSettings.filename_template}
                  onChange={(e) => setLocalSettings({ ...localSettings, filename_template: e.target.value })}
                  placeholder="%(title)s.%(ext)s"
                />
                <p className="text-xs text-gray-500 mt-1">
                  Available: %(title)s, %(id)s, %(ext)s, %(uploader)s, %(resolution)s
                </p>
              </div>
            </div>
          )}
        </div>
        
        <div className="flex justify-between p-4 border-t border-dark-700">
          <Button variant="ghost" onClick={handleReset}>
            <RotateCcw className="w-4 h-4 mr-2" />
            Reset to Defaults
          </Button>
          <div className="flex gap-2">
            <Button variant="secondary" onClick={onClose}>
              Cancel
            </Button>
            <Button onClick={handleSave} isLoading={isSaving}>
              <Save className="w-4 h-4 mr-2" />
              Save
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}