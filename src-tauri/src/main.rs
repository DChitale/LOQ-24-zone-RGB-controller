#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod effect_runner;
mod effects;
mod installer;
mod led_driver;
mod lighting;
mod presets;
mod settings;

use crate::effects::Effect;
use crate::led_driver::{Color, LedController};
use crate::presets::{
    aurora::AuroraEffect,
    breathing::ColorBreathEffect,
    chromaticBreath::ChromaticBreathEffect,
    edgeGlow::LiquidEdgeEffect,
    energyPulse::EnergyPulseEffect,
    fireFlow::FireFlowEffect,
    heatwave::HeatWaveEffect,
    horse::HorseEffect,
    horseCycle::SmoothHorseCycleEffect,
    nebula::NebulaEffect,
    ocean::OceanWaveEffect,
    off::OffEffect,
    pulse::PulseCenterEffect,
    rainbowBreath::RainbowBreathEffect,
    rainbowCycle::RainbowCycleEffect,
    rainbowWave::RainbowWaveEffect,
    rpm::FerrariRpmEffect,
    scan::ColorScanEffect,
    silk::SilkAmbientEffect,
    sparkle::SparkleEffect,
    staticColor::StaticEffect,
    stillGradient::StillGradientEffect,
    sweep::RgbSweepEffect,
    wheel::ColorWheelEffect,
    ParameterValue,
    //thermalStatus::ThermalStatusEffect,
    PresetConfig,
};
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use tauri::{CustomMenuItem, Manager, State, SystemTray, SystemTrayEvent, SystemTrayMenu};

const NUM_ZONES: usize = 24;

pub struct AppState {
    pub controller: Mutex<LedController>,
    pub ui_frame: Arc<Mutex<Vec<Color>>>,
    pub should_run_effect: Arc<AtomicBool>,
    pub current_effect: Mutex<Option<Box<dyn Effect>>>,
    pub current_preset_params: Mutex<HashMap<String, ParameterValue>>,
}

#[tauri::command]
fn set_lighting_priority() -> Result<String, String> {
    lighting::set_windows_lighting_on_top()
        .map(|_| "Windows Dynamic Lighting Controller set to top priority.".to_string())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn check_startup_installed() -> bool {
    installer::is_startup_task_installed()
}

#[tauri::command]
fn install_startup_task(delay_seconds: u32) -> Result<String, String> {
    installer::create_startup_task(delay_seconds)
        .map(|_| "Startup task installed successfully.".to_string())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn uninstall_startup_task() -> Result<String, String> {
    installer::remove_startup_task()
        .map(|_| "Startup task unistalled successfully.".to_string())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_settings() -> Result<settings::AppSettings, String> {
    settings::load_settings().map_err(|e| e.to_string())
}

#[tauri::command]
fn save_settings(settings: settings::AppSettings) -> Result<String, String> {
    settings::save_settings(&settings)
        .map(|_| "Settings saved successfully.".to_string())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn set_brightness(brightness: f32, state: State<AppState>) -> Result<String, String> {
    let b = brightness.clamp(0.0, 1.0);

    // persist
    let mut s = settings::load_settings().map_err(|e| e.to_string())?;
    s.brightness_level = b;
    settings::save_settings(&s).map_err(|e| e.to_string())?;

    // apply to controller state and update UI immediately (do NOT fail the command if device is disconnected)
    let mut applied_to_device = false;
    {
        let mut controller = state.controller.lock().unwrap();
        controller.set_brightness(b);

        // Update the frontend-visible frame from the logical buffer so UI reflects the change even if HID is absent
        let scaled_frame: Vec<led_driver::Color> = controller
            .get_buffer_vec()
            .iter()
            .map(|c| c.perceptual_scale(b))
            .collect();

        *state.ui_frame.lock().unwrap() = scaled_frame.clone();

        // best-effort: try to send to device; if not connected, attempt to connect once
        if !controller.is_connected() {
            let _ = controller.connect();
        }

        if controller.is_connected() {
            if controller.flush_buffered().is_ok() {
                applied_to_device = true;
            }
        }
    }

    // UI listeners will observe the updated `ui_frame` on the next universal-loop tick.
    if applied_to_device {
        Ok("Brightness updated and sent to device".to_string())
    } else {
        Ok("Brightness updated (device not connected; UI preview applied)".to_string())
    }
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
fn set_preset(
    preset_name: String,
    parameters: std::collections::HashMap<String, ParameterValue>,
    state: State<AppState>,
) -> Result<String, String> {
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
        "staticColor" => {
            let color = preset_config
                .parameters
                .get("color")
                .and_then(|v| match v {
                    ParameterValue::Color { r, g, b } => Some(Color::new(*r, *g, *b)),
                    _ => None,
                })
                .unwrap_or(Color::new(255, 255, 200));
            Box::new(crate::presets::staticColor::StaticEffect::new(color))
        }
        "off" => Box::new(OffEffect::new()),
        "rainbowCycle" => {
            let speed = preset_config
                .parameters
                .get("speed")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(1.0);
            Box::new(RainbowCycleEffect::new(speed))
        }
        "rainbowWave" => {
            let speed = preset_config
                .parameters
                .get("speed")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(1.0);
            Box::new(RainbowWaveEffect::new(speed))
        }
        "sweep" => Box::new(RgbSweepEffect::new()),
        "rainbowBreath" => {
            let speed = preset_config
                .parameters
                .get("speed")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(1.0);
            Box::new(RainbowBreathEffect::new(speed))
        }
        "breathing" => {
            let speed = preset_config
                .parameters
                .get("speed")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(1.0);
            let color = preset_config
                .parameters
                .get("color")
                .and_then(|v| match v {
                    ParameterValue::Color { r, g, b } => Some(Color::new(*r, *g, *b)),
                    _ => None,
                })
                .unwrap_or(Color::new(255, 0, 0));
            Box::new(ColorBreathEffect::new(color, speed))
        }
        "horse" => {
            let speed = preset_config
                .parameters
                .get("speed")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(1.0);
            let length = preset_config
                .parameters
                .get("length")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(3.0);
            let base_color = preset_config
                .parameters
                .get("base_color")
                .and_then(|v| match v {
                    ParameterValue::Color { r, g, b } => Some(Color::new(*r, *g, *b)),
                    _ => None,
                })
                .unwrap_or(Color::new(20, 20, 25));

            let horse_color = preset_config
                .parameters
                .get("horse_color")
                .and_then(|v| match v {
                    ParameterValue::Color { r, g, b } => Some(Color::new(*r, *g, *b)),
                    _ => None,
                })
                .unwrap_or(Color::new(120, 140, 180));

            Box::new(HorseEffect::new(speed, length, base_color, horse_color))
        }
        "horseCycle" => {
            let speed = preset_config
                .parameters
                .get("speed")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(1.0);
            let length = preset_config
                .parameters
                .get("length")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(3.0);

            Box::new(SmoothHorseCycleEffect::new(speed, length))
        }
        "rpm" => {
            let speed = preset_config
                .parameters
                .get("speed")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(3.0);

            Box::new(FerrariRpmEffect::new(speed))
        }
        "pulse" => {
            let speed = preset_config
                .parameters
                .get("speed")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(1.0);
            let color = preset_config
                .parameters
                .get("color")
                .and_then(|v| match v {
                    ParameterValue::Color { r, g, b } => Some(Color::new(*r, *g, *b)),
                    _ => None,
                })
                .unwrap_or(Color::new(255, 0, 0));
            Box::new(PulseCenterEffect::new(color, speed))
        }
        "wheel" => {
            let speed = preset_config
                .parameters
                .get("speed")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(0.5);
            Box::new(ColorWheelEffect::new(speed))
        }
        "aurora" => {
            let speed = preset_config
                .parameters
                .get("speed")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(0.5);
            Box::new(AuroraEffect::new(speed))
        }
        "heatwave" => {
            let speed = preset_config
                .parameters
                .get("speed")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(1.0);
            Box::new(HeatWaveEffect::new(speed))
        }
        "scan" => {
            let speed = preset_config
                .parameters
                .get("speed")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(1.0);
            Box::new(ColorScanEffect::new(speed))
        }
        "sparkle" => {
            let density = preset_config
                .parameters
                .get("density")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(0.1);
            Box::new(SparkleEffect::new(density))
        }
        "ocean" => {
            let speed = preset_config
                .parameters
                .get("speed")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(1.0);
            Box::new(crate::presets::ocean::OceanWaveEffect::new(speed))
        }
        "energyPulse" => {
            let speed = preset_config
                .parameters
                .get("speed")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(1.0);
            Box::new(crate::presets::energyPulse::EnergyPulseEffect::new(speed))
        }
        "nebula" => {
            let speed = preset_config
                .parameters
                .get("speed")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(1.0);
            Box::new(crate::presets::nebula::NebulaEffect::new(speed))
        }
        "chromaticBreath" => {
            let speed = preset_config
                .parameters
                .get("speed")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(1.0);
            Box::new(crate::presets::chromaticBreath::ChromaticBreathEffect::new(
                speed,
            ))
        }
        "fireFlow" => {
            let speed = preset_config
                .parameters
                .get("speed")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(1.0);
            Box::new(crate::presets::fireFlow::FireFlowEffect::new(speed))
        }
        "silk" => {
            let speed = preset_config
                .parameters
                .get("speed")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(1.0);
            Box::new(crate::presets::silk::SilkAmbientEffect::new(speed))
        }
        "edgeGlow" => Box::new(crate::presets::edgeGlow::LiquidEdgeEffect::new()),
        "stillGradient" => {
            let color_a = preset_config
                .parameters
                .get("color_a")
                .and_then(|v| match v {
                    ParameterValue::Color { r, g, b } => Some(Color::new(*r, *g, *b)),
                    _ => None,
                })
                .unwrap_or(Color::new(89, 108, 128));
            let color_b = preset_config
                .parameters
                .get("color_b")
                .and_then(|v| match v {
                    ParameterValue::Color { r, g, b } => Some(Color::new(*r, *g, *b)),
                    _ => None,
                })
                .unwrap_or(Color::new(88, 75, 118));
            let middle = preset_config
                .parameters
                .get("middle")
                .and_then(|v| match v {
                    ParameterValue::Float(i) => Some(*i),
                    _ => None,
                })
                .unwrap_or(12.0);
            Box::new(crate::presets::stillGradient::StillGradientEffect::new(
                color_a, color_b, middle,
            ))
        }
        // "thermalStatus" => {
        //     Box::new(crate::presets::thermalStatus::ThermalStatusEffect::new())
        // }
        _ => return Err(format!("Unknown preset: {}", preset_config.name)),
    };

    // Start the new effect
    {
        let mut current_effect = state.current_effect.lock().unwrap();
        *current_effect = Some(new_effect);
    }

    Ok(format!(
        "Preset '{}' loaded successfully",
        preset_config.name
    ))
}

#[tauri::command]
fn adjust_preset_parameter(
    preset_name: String,
    param_name: String,
    value: ParameterValue,
    state: State<AppState>,
) -> Result<(), String> {
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
                    // if(effect.is_static){
                    //     continue;
                    // }else{
                    effect.update(&mut controller, time, delta);
                    //}
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
            // 32ms set for better performance
            tokio::time::sleep(Duration::from_millis(40)).await;
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
            set_lighting_priority,
            check_startup_installed,
            install_startup_task,
            uninstall_startup_task,
            get_settings,
            save_settings,
            set_brightness,
            get_frame,
            //test_set_red,
            get_preset_metadata,
            set_preset,
            adjust_preset_parameter
        ])
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|_app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                if id == "quit" {
                    std::process::exit(0);
                }
            }
            _ => {}
        })
        .setup(|app| {
            let settings = settings::load_settings().unwrap_or_default();

            if settings.auto_fix_on_startup {
                if !installer::is_startup_task_installed() {
                    println!(
                        "Installing startup task with delay of {} seconds",
                        settings.startup_delay_seconds
                    );
                    if let Err(e) = installer::create_startup_task(settings.startup_delay_seconds) {
                        eprintln!("Failed to install startup task: {}", e);
                    } else {
                        println!("Startup task installed successfully");
                    }
                }
                if settings.fix_on_app_launch {
                    if let Err(e) = lighting::set_windows_lighting_on_top() {
                        eprintln!("Failed to set Windows Dynamic Lighting on top: {}", e);
                    } else {
                        println!("Windows Dynamic Lighting Set to top priority on app launch");
                    }
                }
            }

            let handle = app.handle();
            let state = handle.state::<AppState>();

            // Try to connect the HID device first
            {
                let mut controller = state.controller.lock().unwrap();
                let _ = controller.connect();

                // apply persisted brightness immediately so UI/device match on launch
                controller.set_brightness(settings.brightness_level);
                let _ = controller.flush_buffered();
            }

            // START THE UNIVERSAL EFFECT LOOP (works with any effect)
            run_universal_effect_loop(handle.clone());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error");
}
