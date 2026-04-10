use tauri::State;
use crate::models::Settings;
use crate::services::SettingsService;

#[tauri::command]
pub fn get_settings(settings: State<'_, SettingsService>) -> Result<Settings, String> {
    Ok(settings.get())
}

#[tauri::command]
pub fn update_settings(
    settings: State<'_, SettingsService>,
    new_settings: Settings,
) -> Result<(), String> {
    settings.set(new_settings).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn reset_settings(settings: State<'_, SettingsService>) -> Result<(), String> {
    settings.reset().map_err(|e| e.to_string())
}