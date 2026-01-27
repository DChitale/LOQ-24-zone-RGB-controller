use std::env;
use std::error::Error;
use winreg::enums::*;
use winreg::RegKey;
use tauri::{AppHandle, Manager};
use std::time::Duration;

pub fn register_startup() -> Result<(), Box<dyn Error>> {
    // Open HKCU
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    // Windows "Run" key (user startup)
    let path = r"Software\Microsoft\Windows\CurrentVersion\Run";
    let (key, _) = hkcu.create_subkey(path)?;

    // Get current executable path
    let exe_path = env::current_exe()?;
    let exe_path = exe_path
        .to_str()
        .ok_or("Executable path is not valid UTF-8")?;

    // Quote path to handle spaces correctly
    let quoted_path = format!("\"{}\"", exe_path);

    // Register startup entry
    key.set_value("MyLightingController", &quoted_path)?;

    Ok(())
}



