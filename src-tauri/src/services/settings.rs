use anyhow::Result;
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use parking_lot::RwLock;
use crate::models::Settings;

pub struct SettingsService {
    settings: Arc<RwLock<Settings>>,
    config_path: PathBuf,
}

impl SettingsService {
    pub fn new() -> Result<Self> {
        let project_dirs = ProjectDirs::from("com", "AbiYT", "AbiYTDownloader")
            .ok_or_else(|| anyhow::anyhow!("Failed to get project directories"))?;
        
        let config_dir = project_dirs.config_dir();
        fs::create_dir_all(config_dir)?;
        
        let config_path = config_dir.join("settings.json");
        
        let settings = if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Settings::default()
        };
        
        Ok(Self {
            settings: Arc::new(RwLock::new(settings)),
            config_path,
        })
    }

    pub fn get(&self) -> Settings {
        self.settings.read().clone()
    }

    pub fn update<F>(&self, f: F) -> Result<()>
    where
        F: FnOnce(&mut Settings),
    {
        {
            let mut settings = self.settings.write();
            f(&mut settings);
        }
        self.save()
    }

    pub fn set(&self, settings: Settings) -> Result<()> {
        {
            let mut current = self.settings.write();
            *current = settings;
        }
        self.save()
    }

    fn save(&self) -> Result<()> {
        let settings = self.settings.read().clone();
        let content = serde_json::to_string_pretty(&settings)?;
        fs::write(&self.config_path, content)?;
        Ok(())
    }

    pub fn reset(&self) -> Result<()> {
        self.set(Settings::default())
    }
}

impl Default for SettingsService {
    fn default() -> Self {
        Self::new().expect("Failed to initialize settings service")
    }
}