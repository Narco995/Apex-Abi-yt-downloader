import { useEffect, useState } from 'react';
import { Settings, Download, Info, Github } from 'lucide-react';
import { Dashboard, SettingsModal, Button } from '@/components';
import { useAppStore } from '@/stores/useAppStore';
import { Toaster } from 'react-hot-toast';
// Fix: removed unused `api` import

function App() {
  const [showSettings, setShowSettings] = useState(false);
  const { fetchSettings, fetchDependencies, dependencies } = useAppStore();

  useEffect(() => {
    fetchSettings();
    fetchDependencies();
  }, [fetchSettings, fetchDependencies]);

  const missingDeps = dependencies && (!dependencies.yt_dlp || !dependencies.ffmpeg);

  return (
    <div className="min-h-screen bg-dark-950">
      <Toaster
        position="bottom-right"
        toastOptions={{
          style: {
            background: '#1e293b',
            color: '#fff',
            border: '1px solid #334155',
          },
        }}
      />
      
      {/* Top Navigation */}
      <nav className="fixed top-0 left-0 right-0 z-40 bg-dark-900/90 backdrop-blur-sm border-b border-dark-800">
        <div className="max-w-6xl mx-auto px-4 py-3 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <Download className="w-6 h-6 text-primary-500" />
            <span className="text-lg font-bold">Abi YT Downloader</span>
          </div>
          
          <div className="flex items-center gap-2">
            {missingDeps && (
              <Button
                variant="danger"
                size="sm"
                onClick={() => {
                  alert('Please install yt-dlp and ffmpeg to use this application.\n\nInstall instructions:\n- yt-dlp: https://github.com/yt-dlp/yt-dlp\n- ffmpeg: https://ffmpeg.org/download.html');
                }}
              >
                <Info className="w-4 h-4 mr-1" />
                Missing Dependencies
              </Button>
            )}
            <Button
              variant="ghost"
              size="sm"
              onClick={() => setShowSettings(true)}
            >
              <Settings className="w-4 h-4" />
            </Button>
            <Button
              variant="ghost"
              size="sm"
              onClick={() => window.open('https://github.com/Narco995/Apex-Abi-yt-downloader', '_blank')}
            >
              <Github className="w-4 h-4" />
            </Button>
          </div>
        </div>
      </nav>
      
      {/* Main Content */}
      <main className="pt-16">
        <Dashboard />
      </main>
      
      {/* Settings Modal */}
      <SettingsModal
        isOpen={showSettings}
        onClose={() => setShowSettings(false)}
      />
    </div>
  );
}

export default App;
