// src-tauri/src/settings.rs
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// Enable automatic fix on system startup
    pub auto_fix_on_startup: bool,
    
    /// Delay in seconds before running the fix after login
    pub startup_delay_seconds: u32,
    
    /// Fix lighting priority when app launches
    pub fix_on_app_launch: bool,

    #[serde(default = "default_brightness_level")]
    pub brightness_level: f32,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            auto_fix_on_startup: false,  // Disabled by default - user must opt-in
            startup_delay_seconds: 60,
            fix_on_app_launch: true,
            brightness_level: 1.0,
        }
    }
}

/// Default value for `brightness_level` used by serde's `default` attribute
fn default_brightness_level() -> f32 { 1.0 }

/// Get the path to the settings file
fn get_settings_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let appdata = std::env::var("APPDATA")?;
    let app_dir = PathBuf::from(appdata).join("LightingControl");
    
    // Create directory if it doesn't exist
    fs::create_dir_all(&app_dir)?;
    
    Ok(app_dir.join("settings.json"))
}

/// Load settings from file
pub fn load_settings() -> Result<AppSettings, Box<dyn std::error::Error>> {
    let settings_path = get_settings_path()?;
    
    if !settings_path.exists() {
        // Return default settings if file doesn't exist
        return Ok(AppSettings::default());
    }
    
    let contents = fs::read_to_string(settings_path)?;
    let settings: AppSettings = serde_json::from_str(&contents)?;
    
    Ok(settings)
}

/// Save settings to file
pub fn save_settings(settings: &AppSettings) -> Result<(), Box<dyn std::error::Error>> {
    let settings_path = get_settings_path()?;
    let json = serde_json::to_string_pretty(settings)?;
    
    fs::write(settings_path, json)?;
    
    Ok(())
}