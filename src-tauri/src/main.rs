#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod led_driver;
mod effects;
mod effect_runner;
mod presets;

use crate::led_driver::{LedController, Color};
use crate::effects::Effect;
use crate::presets::{
    pulse::PulseCenterEffect,
    aurora::AuroraEffect,
    heatwave::HeatWaveEffect,
    scan::ColorScanEffect,
    sparkle::SparkleEffect,
    PresetConfig, ParameterValue
};
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::collections::HashMap;
use tauri::{Manager, SystemTray, SystemTrayMenu, CustomMenuItem, SystemTrayEvent, State};

const NUM_ZONES: usize = 24;

pub struct AppState {
    pub controller: Mutex<LedController>,
    pub ui_frame: Arc<Mutex<Vec<Color>>>,
    pub should_run_effect: Arc<AtomicBool>,
    pub current_effect: Mutex<Option<Box<dyn Effect>>>,
    pub current_preset_params: Mutex<HashMap<String, ParameterValue>>,
}

#[tauri::command]
fn get_frame(state: State<AppState>) -> Vec<Color> {
    state.ui_frame.lock().unwrap().clone()
}

#[tauri::command]
fn test_set_red(state: State<AppState>) -> Result<(), String> {
    let mut controller = state.controller.lock().unwrap();
    
    // Ensure connection before writing
    if !controller.is_connected() {
        controller.connect()?;
    }

    controller.set_range(0, 23, Color::new(125, 125, 125))?;
    Ok(())
}

#[tauri::command]
fn get_preset_metadata() -> Vec<crate::presets::PresetMetadata> {
    crate::presets::get_available_presets()
}

#[tauri::command]
fn set_preset(preset_name: String, parameters: std::collections::HashMap<String, ParameterValue>, state: State<AppState>) -> Result<String, String> {
    let preset_config = PresetConfig {
        name: preset_name,
        parameters,
    };
    // Stop current effect
    {
        let mut current_effect = state.current_effect.lock().unwrap();
        if let Some(mut effect) = current_effect.take() {
            let mut controller = state.controller.lock().unwrap();
            effect.stop(&mut controller);
        }
    }

    // Store parameters
    {
        let mut params = state.current_preset_params.lock().unwrap();
        params.clear();
        for (key, value) in &preset_config.parameters {
            params.insert(key.clone(), value.clone());
        }
    }

    // Create new effect based on preset name
    let new_effect: Box<dyn Effect> = match preset_config.name.as_str() {
        "pulse" => {
            let speed = preset_config.parameters.get("speed")
                .and_then(|v| match v { ParameterValue::Float(f) => Some(*f), _ => None })
                .unwrap_or(1.0);
            let color = preset_config.parameters.get("color")
                .and_then(|v| match v { ParameterValue::Color { r, g, b } => Some(Color::new(*r, *g, *b)), _ => None })
                .unwrap_or(Color::new(255, 0, 0));
            Box::new(PulseCenterEffect::new(color, speed))
        },
        "aurora" => {
            let speed = preset_config.parameters.get("speed")
                .and_then(|v| match v { ParameterValue::Float(f) => Some(*f), _ => None })
                .unwrap_or(0.5);
            Box::new(AuroraEffect::new(speed))
        },
        "heatwave" => {
            let speed = preset_config.parameters.get("speed")
                .and_then(|v| match v { ParameterValue::Float(f) => Some(*f), _ => None })
                .unwrap_or(1.0);
            Box::new(HeatWaveEffect::new(speed))
        },
        "scan" => {
            let speed = preset_config.parameters.get("speed")
                .and_then(|v| match v { ParameterValue::Float(f) => Some(*f), _ => None })
                .unwrap_or(1.0);
            Box::new(ColorScanEffect::new(speed))
        },
        "sparkle" => {
            let density = preset_config.parameters.get("density")
                .and_then(|v| match v { ParameterValue::Float(f) => Some(*f), _ => None })
                .unwrap_or(0.1);
            Box::new(SparkleEffect::new(density))
        },
        _ => return Err(format!("Unknown preset: {}", preset_config.name)),
    };

    // Start the new effect
    {
        let mut current_effect = state.current_effect.lock().unwrap();
        *current_effect = Some(new_effect);
    }

    Ok(format!("Preset '{}' loaded successfully", preset_config.name))
}

#[tauri::command]
fn adjust_preset_parameter(preset_name: String, param_name: String, value: ParameterValue, state: State<AppState>) -> Result<(), String> {
    // Update stored parameters
    {
        let mut params = state.current_preset_params.lock().unwrap();
        params.insert(param_name.clone(), value.clone());
    }

    // Recreate the effect with updated parameters
    let preset_config = PresetConfig {
        name: preset_name.clone(),
        parameters: state.current_preset_params.lock().unwrap().clone(),
    };

    // Call set_preset but ignore the result since we want to return ()
    let _ = set_preset(preset_name, preset_config.parameters, state);
    Ok(())
}

use std::time::Duration;

/// Universal effect loop - emits all effect updates to the frontend
fn run_universal_effect_loop(app_handle: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        let start_time = std::time::Instant::now();
        let mut last_update = std::time::Instant::now();

        loop {
            let state = app_handle.state::<AppState>();

            if !state.should_run_effect.load(Ordering::Relaxed) {
                tokio::time::sleep(Duration::from_millis(16)).await;
                continue;
            }

            // Update current effect if one exists
            {
                let mut current_effect = state.current_effect.lock().unwrap();
                if let Some(ref mut effect) = *current_effect {
                    let now = std::time::Instant::now();
                    let time = (now - start_time).as_secs_f32();
                    let delta = (now - last_update).as_secs_f32();
                    last_update = now;

                    let mut controller = state.controller.lock().unwrap();
                    effect.update(&mut controller, time, delta);
                }
            }

            // Copy current frame buffer from controller to the frontend UI frame
            {
                let controller = state.controller.lock().unwrap();
                let current_frame = controller.get_buffer_vec();
                *state.ui_frame.lock().unwrap() = current_frame;
            }

            // Emit the frame data to the frontend
            {
                let frame = state.ui_frame.lock().unwrap().clone();
                let _ = app_handle.emit_all("new-colors", frame);
            }

            // Yield to OS at target FPS (60 FPS = 16.67ms per frame)
            tokio::time::sleep(Duration::from_millis(16)).await;
        }
    });
}


fn main() {
    let ui_frame = Arc::new(Mutex::new(vec![Color::black(); NUM_ZONES]));
    let controller = LedController::new(ui_frame.clone());
    
    let app_state = AppState {
        controller: Mutex::new(controller),
        ui_frame: ui_frame.clone(),
        should_run_effect: Arc::new(AtomicBool::new(true)),
        current_effect: Mutex::new(None),
        current_preset_params: Mutex::new(HashMap::new()),
    };

    let tray_menu = SystemTrayMenu::new().add_item(CustomMenuItem::new("quit", "Exit"));

    tauri::Builder::default()
        .manage(app_state)
        // CRITICAL FIX: Only one invoke_handler call with all commands
        .invoke_handler(tauri::generate_handler![
            get_frame,
            test_set_red,
            get_preset_metadata,
            set_preset,
            adjust_preset_parameter
        ])
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|_app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => if id == "quit" { std::process::exit(0); },
            _ => {}
        })
        .setup(|app| {
            let handle = app.handle();
            let state = handle.state::<AppState>();
            
            // Try to connect the HID device first
            {
                let mut controller = state.controller.lock().unwrap();
                let _ = controller.connect();
            }

            // START THE UNIVERSAL EFFECT LOOP (works with any effect)
            run_universal_effect_loop(handle.clone());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error");
}