# Abi YT Downloader

<p align="center">
  <img src="https://img.shields.io/badge/version-1.0.0-blue.svg" alt="Version">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="License">
  <img src="https://img.shields.io/badge/Rust-1.70+-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/Tauri-v2-blue.svg" alt="Tauri">
</p>

A high-performance, secure, and lightweight YouTube video downloader built with **Tauri v2**, **Rust**, and **React 19**. Download videos in multiple formats and qualities with ease.

## ✨ Features

- 🚀 **High Performance** - Built with Rust backend for blazing-fast downloads
- 🔒 **Secure** - Sandboxed WebView, least-privilege permissions
- 🎨 **Modern UI** - Beautiful dark theme with smooth animations
- 📦 **Lightweight** - Small bundle size, minimal resource usage
- 🎬 **Multiple Formats** - MP4, WebM, MKV, MP3, M4A
- 📊 **Quality Selection** - Highest, High, Medium, Low options
- ⬇️ **Parallel Downloads** - Configurable concurrent download limit
- 💾 **Download History** - Track all your downloads
- 🔔 **Real-time Progress** - Live progress updates and ETA
- 🎯 **Batch Downloads** - Download multiple videos at once

## 📋 Prerequisites

Before using Abi YT Downloader, ensure you have the following installed:

1. **yt-dlp** - YouTube download engine
   ```bash
   # Windows (using pip)
   pip install yt-dlp
   
   # Windows (using winget)
   winget install yt-dlp.yt-dlp
   
   # Windows (using scoop)
   scoop install yt-dlp
   ```

2. **FFmpeg** - For audio conversion and video merging
   ```bash
   # Windows (using winget)
   winget install Gyan.FFmpeg
   
   # Windows (using scoop)
   scoop install ffmpeg
   ```

## 🚀 Installation

### Download Pre-built Executable

Download the latest release from the [Releases](https://github.com/Narco995/Apex-Abi-yt-downloader/releases) page.

### Build from Source

1. **Clone the repository**
   ```bash
   git clone https://github.com/Narco995/Apex-Abi-yt-downloader.git
   cd Apex-Abi-yt-downloader
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Development mode**
   ```bash
   npm run tauri dev
   ```

4. **Build for production**
   ```bash
   npm run tauri build
   ```

The built executable will be in `src-tauri/target/release/bundle/nsis/`.

## 🛠️ Tech Stack

- **Frontend**
  - React 19
  - TypeScript 5.8
  - TailwindCSS 3.4
  - Zustand 5 (State Management)
  - Lucide React (Icons)

- **Backend**
  - Rust (1.70+)
  - Tauri v2
  - Tokio (Async Runtime)
  - Reqwest (HTTP Client)

## 📁 Project Structure

```
abi-yt-downloader/
├── src/                    # React frontend
│   ├── components/         # UI components
│   ├── hooks/              # Custom React hooks
│   ├── stores/             # Zustand stores
│   ├── types/              # TypeScript types
│   └── utils/              # Utility functions
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── commands/       # Tauri IPC commands
│   │   ├── engine/         # Download engine
│   │   ├── models/         # Data models
│   │   ├── services/       # Services (settings, history)
│   │   └── utils/          # Utility functions
│   └── tauri.conf.json     # Tauri configuration
└── .github/                # GitHub Actions workflows
```

## 🔧 Configuration

### Settings

- **Download Location** - Where files are saved
- **Parallel Downloads** - Number of concurrent downloads (1-10)
- **Max Retries** - Retry attempts for failed downloads
- **Preferred Format** - Default video/audio format
- **Quality Preference** - Default quality level
- **Skip Existing** - Skip already downloaded files
- **Filename Template** - Custom filename pattern

### Filename Template Variables

| Variable | Description |
|----------|-------------|
| `%(title)s` | Video title |
| `%(id)s` | YouTube video ID |
| `%(ext)s` | File extension |
| `%(uploader)s` | Channel name |
| `%(resolution)s` | Video resolution |

## 🔐 Security

- **Sandboxed WebView** - UI runs in isolated environment
- **Least Privilege** - Minimal required permissions
- **No Tracking** - All data stays local
- **Secure IPC** - Type-safe communication between frontend and backend

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## ⚠️ Disclaimer

This tool is for personal use only. Please respect YouTube's Terms of Service and copyright laws. The developers are not responsible for any misuse of this software.

## 🙏 Acknowledgments

- [yt-dlp](https://github.com/yt-dlp/yt-dlp) - The powerful YouTube download engine
- [Tauri](https://tauri.app/) - Build smaller, faster, and more secure desktop apps
- [React](https://react.dev/) - The library for web and native user interfaces
- [TailwindCSS](https://tailwindcss.com/) - A utility-first CSS framework

---

<p align="center">
  Made with ❤️ by Abi YT Team
</p>