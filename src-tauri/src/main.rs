#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod led_driver;
mod effects;

use crate::led_driver::{LedController, Color};
use crate::effects::{LedEffect, RenderMode};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;
use tauri::{Manager, SystemTray, SystemTrayMenu, CustomMenuItem, SystemTrayEvent, State};

pub struct AppState {
    pub controller: Mutex<LedController>,
    pub active_effect: Mutex<Option<Box<dyn LedEffect>>>,
}

#[tauri::command]
fn apply_effect(effect_type: String, r: u8, g: u8, b: u8, state: tauri::State<'_, Arc<AppState>>) -> Result<(), String> {
    let mut effect_lock = state.active_effect.lock().unwrap();
    
    match effect_type.as_str() {
        "rainbow" => {
            *effect_lock = Some(Box::new(effects::rainbow::RainbowEffect { speed: 2.0 }));
        },
        "static" => {
            *effect_lock = Some(Box::new(effects::static_color::StaticColor { 
                color: Color::new(r, g, b) 
            }));
        },
        "off" => {
            *effect_lock = None;
            let mut controller = state.controller.lock().unwrap();
            let _ = controller.clear();
        },
        _ => return Err("Unknown effect".into()),
    }
    Ok(())
}


fn main() {
    let app_state = Arc::new(AppState {
        controller: Mutex::new(LedController::new()),
        active_effect: Mutex::new(None),
    });

    // --- BACKGROUND ENGINE ---
    let engine_state = app_state.clone();
    thread::spawn(move || {
        let mut tick: f32 = 0.0;
        loop {
            let start = Instant::now();
            {
                let mut controller = engine_state.controller.lock().unwrap();
                let mut effect_opt = engine_state.active_effect.lock().unwrap();

                if controller.is_connected() {
                    if let Some(effect) = effect_opt.as_mut() {
                        effect.update(&mut *controller, tick);
                        match effect.render_mode() {
                            RenderMode::Static => {
                                let c = controller.get_zone(0);
                                let _ = controller.set_range(0, 23, c);
                            }
                            RenderMode::Animated => { let _ = controller.flush_buffered(); }
                        }
                    }
                } else { let _ = controller.connect(); }
            }
            tick += 0.1;
            thread::sleep(Duration::from_millis(16).saturating_sub(start.elapsed()));
        }
    });

    // --- TRAY & TAURI SETUP ---
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("quit", "Exit"));

    tauri::Builder::default()
        .manage(app_state)
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => if id == "quit" { std::process::exit(0); },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            apply_effect
        ]) // Add effect-switching commands here
        .run(tauri::generate_context!())
        .expect("error");
}