use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

// Our app config stores user info that should be remembered between sessions
#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub file_path: String,
}

// The config used on first session
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            file_path: dirs::document_dir().unwrap().to_str().unwrap().to_owned(),
        }
    }
}

impl AppConfig {

    // gets the config path for each platform
    fn get_config_path() -> PathBuf {
        let config_dir = if cfg!(target_os = "windows") {
            dirs::config_dir().unwrap_or_else(|| PathBuf::from("."))
        } else if cfg!(target_os = "macos") {
            dirs::config_dir().unwrap_or_else(|| PathBuf::from("."))
        } else {
            dirs::config_dir().unwrap_or_else(|| PathBuf::from("."))
        };

        config_dir.join("remdaw")
    }

    // Loads app configuration from file
    pub fn load() -> Self {
        let config_path = Self::get_config_path().join("config.json");

        if let Ok(content) = fs::read_to_string(&config_path) {
            if let Ok(config) = serde_json::from_str(&content) {
                return config;
            }
        }

        Self::default()
    }

    // saves config file
    pub fn save(&self) {
        if self.is_valid() {
            let config_dir = Self::get_config_path();
            if fs::create_dir_all(&config_dir).is_ok() {
                let config_path = config_dir.join("config.json");
                if let Ok(json) = serde_json::to_string_pretty(&self) {
                    let _ = fs::write(config_path, json);
                }
            }
        }
    }

    // checks if save is valid
    pub fn is_valid(&self) -> bool {
        !self.file_path.is_empty() &&
            PathBuf::from(&self.file_path).exists()
    }
}