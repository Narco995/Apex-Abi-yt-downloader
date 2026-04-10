#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod engine;
mod models;
mod services;
mod utils;

use commands::{downloads::DownloadManager, settings::*, system::*};
use commands::downloads::*;
use services::{SettingsService, HistoryService};
use tauri::Manager;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let settings = SettingsService::new().expect("Failed to initialize settings service");
    let history = HistoryService::new().expect("Failed to initialize history service");
    let parallel_limit = settings.get().parallel_downloads;
    let download_manager = DownloadManager::new(parallel_limit);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init()) // Fix: was missing, required for window.open() & open_folder
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(settings)
        .manage(history)
        .manage(download_manager)
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Settings
            get_settings,
            update_settings,
            reset_settings,
            // Downloads
            resolve_video,
            start_download,
            cancel_download,
            get_active_downloads,
            batch_download,
            // System
            check_dependencies,
            open_folder,
            get_version,
            get_download_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
