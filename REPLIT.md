# Apex Abi YT Downloader — Replit Build Guide

This is a **Tauri 2 desktop app** (React + TypeScript frontend, Rust backend).  
Replit runs Linux, so you can compile a native Linux binary here.

---

## 1. Import the Repo

1. In Replit → **Create Repl** → **Import from GitHub**
2. Paste: `https://github.com/Narco995/Apex-Abi-yt-downloader`
3. Language: **Bash** (we'll run everything manually)

---

## 2. Install System Dependencies

Open the **Shell** tab and run:

```bash
# Update package lists
sudo apt-get update -y

# Tauri Linux system dependencies
sudo apt-get install -y \
  build-essential \
  curl \
  wget \
  libssl-dev \
  libgtk-3-dev \
  libwebkit2gtk-4.1-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  patchelf \
  libsoup-3.0-dev \
  libjavascriptcoregtk-4.1-dev
```

---

## 3. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
rustup update stable
```

---

## 4. Install Node.js (v20+) and pnpm/npm

```bash
# Install Node via nvm
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.0/install.sh | bash
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && . "$NVM_DIR/nvm.sh"
nvm install 20
nvm use 20
node --version   # should print v20.x.x
```

---

## 5. Install Frontend Dependencies

```bash
npm install
```

---

## 6. Install Runtime Dependencies (yt-dlp + ffmpeg)

The app needs these on PATH at runtime:

```bash
# yt-dlp
sudo curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp -o /usr/local/bin/yt-dlp
sudo chmod +x /usr/local/bin/yt-dlp
yt-dlp --version   # verify

# ffmpeg
sudo apt-get install -y ffmpeg
ffmpeg -version    # verify
```

---

## 7. Build the App

### Development build (faster, includes devtools):
```bash
npm run tauri dev
```
> Note: `tauri dev` opens a window — on headless Replit this will error on window creation,
> but the Rust + frontend compilation will succeed and you can inspect errors.

### Production build (optimized binary):
```bash
npm run tauri build
```

Output binary will be at:
```
src-tauri/target/release/abi-yt-downloader
```

Installer packages (.deb, .AppImage) will be at:
```
src-tauri/target/release/bundle/
```

---

## 8. Run the Binary

```bash
./src-tauri/target/release/abi-yt-downloader
```

---

## 9. Headless / Server Mode (no GUI)

Tauri is a desktop app — it needs a display.  
To test on headless Replit without a monitor:

```bash
# Install virtual display
sudo apt-get install -y xvfb

# Start virtual display
Xvfb :99 -screen 0 1024x768x24 &
export DISPLAY=:99

# Run the app
./src-tauri/target/release/abi-yt-downloader
```

---

## Environment Variables

| Variable | Purpose | Default |
|----------|---------|--------|
| `RUST_LOG` | Log level (`debug`, `info`, `warn`, `error`) | `info` |
| `DISPLAY` | X display for GUI (headless: `:99`) | `:0` |

---

## Troubleshooting

| Error | Fix |
|-------|-----|
| `webkit2gtk not found` | Run step 2 again |
| `yt-dlp: command not found` | Run step 6 |
| `error: linker cc not found` | `sudo apt-get install build-essential` |
| `cannot open display` | Use Xvfb (step 9) |
| `thread panicked at generate_context` | Run from repo root, not src-tauri/ |
