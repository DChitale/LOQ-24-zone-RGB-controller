use std::os::raw::c_char;
use std::ffi::{CStr, CString};
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::collections::HashMap;
use std::time::Duration;
use once_cell::sync::Lazy;

mod audio_sampler;
mod input_handler;
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
    horse::HorseEffect,
    horseCycle::SmoothHorseCycleEffect,
    off::OffEffect,
    pulse::PulseCenterEffect,
    rainbowBreath::RainbowBreathEffect,
    rainbowCycle::RainbowCycleEffect,
    rainbowWave::RainbowWaveEffect,
    rpm::FerrariRpmEffect,
    scan::ColorScanEffect,
    sparkle::SparkleEffect,
    sweep::RgbSweepEffect,
    wheel::ColorWheelEffect,
    thermalStatus::ThermalStatusEffect,
    ambient::AmbientEffect,
    audio_sparkle::AudioSparkleEffect,
    audio_sparkle_rainbow::AudioSparkleRainbowEffect,
    audio_sparkle_media::AudioSparkleMediaEffect,
    audio_ripple::AudioRippleEffect,
    rainbow_ripple::RainbowRippleEffect,
    ParameterValue,
    PresetConfig,
};

const NUM_ZONES: usize = 24;

pub struct AppState {
    pub controller: Mutex<LedController>,
    pub ui_frame: Arc<Mutex<Vec<Color>>>,
    pub should_run_effect: Arc<AtomicBool>,
    pub current_effect: Mutex<Option<Box<dyn Effect>>>,
    pub current_preset_params: Mutex<HashMap<String, ParameterValue>>,
    pub preset_cycle_index: std::sync::atomic::AtomicUsize,
}

static APP_STATE: Lazy<Mutex<Option<Arc<AppState>>>> = Lazy::new(|| Mutex::new(None));

pub type FrameCallback = unsafe extern "C" fn(*const Color, i32);
static FRAME_CALLBACK: Lazy<Mutex<Option<FrameCallback>>> = Lazy::new(|| Mutex::new(None));

// ===================================================================
// FFI HELPERS
// ===================================================================

unsafe fn c_str_to_str<'a>(ptr: *const c_char) -> Result<&'a str, String> {
    if ptr.is_null() {
        return Err("Null pointer passed for string".to_string());
    }
    CStr::from_ptr(ptr)
        .to_str()
        .map_err(|e| format!("Invalid UTF-8 string: {}", e))
}

fn string_to_c_char(s: String) -> *mut c_char {
    let c_str = CString::new(s).unwrap_or_else(|_| CString::new("Error: CString conversion failed").unwrap());
    c_str.into_raw()
}

// ===================================================================
// CORE API EXPORTS
// ===================================================================

#[no_mangle]
pub extern "C" fn rgb_init() -> i32 {
    let mut state_lock = APP_STATE.lock().unwrap();
    if state_lock.is_none() {
        let ui_frame = Arc::new(Mutex::new(vec![Color::black(); NUM_ZONES]));
        let mut controller = LedController::new(ui_frame.clone());
        
        let settings = settings::load_settings().unwrap_or_default();
        let _ = lighting::set_light_control_owner(true);
        let _ = controller.connect();
        controller.set_brightness(settings.brightness_level);
        let _ = controller.flush_buffered();

        let state = Arc::new(AppState {
            controller: Mutex::new(controller),
            ui_frame: ui_frame.clone(),
            should_run_effect: Arc::new(AtomicBool::new(true)),
            current_effect: Mutex::new(None),
            current_preset_params: Mutex::new(HashMap::new()),
            preset_cycle_index: std::sync::atomic::AtomicUsize::new(0),
        });
        
        *state_lock = Some(state.clone());

        // Start key listener
        input_handler::start_key_listener();

        // Run startup fixes if configured
        if settings.auto_fix_on_startup {
            if !installer::is_startup_task_installed() {
                let _ = installer::create_startup_task(settings.startup_delay_seconds);
            }
            if settings.fix_on_app_launch {
                let _ = lighting::set_windows_lighting_on_top();
            }
        }

        // Start effect loop in background thread
        let loop_state = state.clone();
        std::thread::spawn(move || {
            let start_time = std::time::Instant::now();
            let mut last_update = std::time::Instant::now();
            let mut consecutive_hid_failures: u32 = 0;
            const MAX_CONSECUTIVE_FAILURES: u32 = 5;
            const SLEEP_DELTA_THRESHOLD: f32 = 2.0; // seconds — anything above this means we just resumed

            loop {
                // If AppState was destroyed, exit loop
                if APP_STATE.lock().unwrap().is_none() {
                    break;
                }

                if !loop_state.should_run_effect.load(Ordering::Relaxed) {
                    std::thread::sleep(Duration::from_millis(16));
                    continue;
                }

                let now = std::time::Instant::now();
                let raw_delta = (now - last_update).as_secs_f32();
                last_update = now;

                // Sleep/resume detection: if the gap is huge, the system just woke up.
                // Clamp delta to one normal frame so effects don't compute with a multi-hour delta.
                let delta = if raw_delta > SLEEP_DELTA_THRESHOLD {
                    // Reconnect the HID device — the old handle is almost certainly stale
                    {
                        let mut controller = loop_state.controller.lock().unwrap();
                        let _ = controller.reconnect();
                    }
                    consecutive_hid_failures = 0;
                    0.04 // one normal frame at 25fps
                } else {
                    raw_delta
                };

                let time = (now - start_time).as_secs_f32();

                // Update current effect
                {
                    let mut current_effect = loop_state.current_effect.lock().unwrap();
                    if let Some(ref mut effect) = *current_effect {
                        let mut controller = loop_state.controller.lock().unwrap();
                        effect.update(&mut controller, time, delta);
                    }
                }

                // Copy buffer and trigger callback
                let frame = {
                    let controller = loop_state.controller.lock().unwrap();
                    let current_frame = controller.get_buffer_vec();
                    *loop_state.ui_frame.lock().unwrap() = current_frame.clone();
                    current_frame
                };

                if let Some(callback) = *FRAME_CALLBACK.lock().unwrap() {
                    unsafe {
                        callback(frame.as_ptr(), frame.len() as i32);
                    }
                }

                // Passive reconnection: if HID writes are silently failing (effects use `let _ = ...`),
                // detect via is_connected() check after the update and attempt recovery.
                {
                    let controller = loop_state.controller.lock().unwrap();
                    if !controller.is_connected() {
                        consecutive_hid_failures += 1;
                    } else {
                        consecutive_hid_failures = 0;
                    }
                }

                if consecutive_hid_failures >= MAX_CONSECUTIVE_FAILURES {
                    let mut controller = loop_state.controller.lock().unwrap();
                    let _ = controller.reconnect();
                    consecutive_hid_failures = 0;
                }

                std::thread::sleep(Duration::from_millis(40));
            }
        });

        1 // Success
    } else {
        0 // Already initialized
    }
}

#[no_mangle]
pub extern "C" fn rgb_shutdown() {
    *FRAME_CALLBACK.lock().unwrap() = None;

    let mut state_lock = APP_STATE.lock().unwrap();
    if let Some(state) = state_lock.take() {
        // Stop active effect
        let mut current_effect = state.current_effect.lock().unwrap();
        if let Some(mut effect) = current_effect.take() {
            let mut controller = state.controller.lock().unwrap();
            effect.stop(&mut controller);
        }
        
        // Turn off LEDs
        let mut controller = state.controller.lock().unwrap();
        controller.fill(Color::black());
        let _ = controller.flush_buffered();
        let _ = lighting::set_light_control_owner(false);
        controller.disconnect();
    }
}

#[no_mangle]
pub extern "C" fn rgb_start_frame_callback(callback: Option<FrameCallback>) {
    *FRAME_CALLBACK.lock().unwrap() = callback;
}

#[no_mangle]
pub extern "C" fn rgb_get_brightness() -> f32 {
    let state_lock = APP_STATE.lock().unwrap();
    if let Some(ref state) = *state_lock {
        state.controller.lock().unwrap().brightness()
    } else {
        1.0
    }
}

#[no_mangle]
pub extern "C" fn rgb_set_brightness(brightness: f32) -> i32 {
    let state_lock = APP_STATE.lock().unwrap();
    let state = match &*state_lock {
        Some(s) => s,
        None => return 0,
    };

    let b = brightness.clamp(0.0, 1.0);

    // Persist brightness
    let mut s = match settings::load_settings() {
        Ok(s) => s,
        Err(_) => return 0,
    };
    s.brightness_level = b;
    if settings::save_settings(&s).is_err() {
        return 0;
    }

    let mut controller = state.controller.lock().unwrap();
    controller.set_brightness(b);

    // Update UI frame immediately
    let scaled_frame: Vec<Color> = controller
        .get_buffer_vec()
        .iter()
        .map(|c| c.perceptual_scale(b))
        .collect();

    *state.ui_frame.lock().unwrap() = scaled_frame.clone();

    if !controller.is_connected() {
        let _ = controller.connect();
    }

    if controller.is_connected() {
        let _ = controller.flush_buffered();
    }

    1
}

#[no_mangle]
pub extern "C" fn rgb_get_frame(buffer: *mut Color, len: i32) -> i32 {
    if buffer.is_null() || len < NUM_ZONES as i32 {
        return 0;
    }

    let state_lock = APP_STATE.lock().unwrap();
    if let Some(ref state) = *state_lock {
        let frame = state.ui_frame.lock().unwrap();
        unsafe {
            std::ptr::copy_nonoverlapping(frame.as_ptr(), buffer, NUM_ZONES);
        }
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn rgb_get_preset_metadata() -> *mut c_char {
    let metadata = presets::get_preset_metadata();
    let json = serde_json::to_string(&metadata).unwrap_or_else(|_| "[]".to_string());
    string_to_c_char(json)
}

#[no_mangle]
pub extern "C" fn rgb_set_preset(preset_name: *const c_char, params_json: *const c_char) -> *mut c_char {
    let name_str = match unsafe { c_str_to_str(preset_name) } {
        Ok(s) => s.to_string(),
        Err(e) => return string_to_c_char(format!("Error: {}", e)),
    };

    let params_str = match unsafe { c_str_to_str(params_json) } {
        Ok(s) => s,
        Err(e) => return string_to_c_char(format!("Error: {}", e)),
    };

    let parameters: HashMap<String, ParameterValue> = match serde_json::from_str(params_str) {
        Ok(p) => p,
        Err(e) => return string_to_c_char(format!("Error deserializing parameters: {}", e)),
    };

    // Save parameter tweaks
    if let Ok(mut settings) = settings::load_settings() {
        if !parameters.is_empty() {
            settings.preset_tweaks.insert(name_str.clone(), parameters.clone());
            let _ = settings::save_settings(&settings);
        }
    }

    let state_lock = APP_STATE.lock().unwrap();
    if let Some(ref state) = *state_lock {
        // Sync the cycle index to match the manually selected preset if it's in the cycle list
        if let Ok(settings) = settings::load_settings() {
            if let Some(pos) = settings.preset_cycle_effects.iter().position(|x| x.eq_ignore_ascii_case(&name_str)) {
                state.preset_cycle_index.store(pos, std::sync::atomic::Ordering::SeqCst);
            }
        }

        match apply_preset(name_str, parameters, state) {
            Ok(msg) => string_to_c_char(msg),
            Err(e) => string_to_c_char(format!("Error: {}", e)),
        }
    } else {
        string_to_c_char("Error: AppState not initialized".to_string())
    }
}

#[no_mangle]
pub extern "C" fn rgb_adjust_parameter(preset_name: *const c_char, param_name: *const c_char, value_json: *const c_char) -> *mut c_char {
    let name_str = match unsafe { c_str_to_str(preset_name) } {
        Ok(s) => s.to_string(),
        Err(e) => return string_to_c_char(format!("Error: {}", e)),
    };

    let param_str = match unsafe { c_str_to_str(param_name) } {
        Ok(s) => s.to_string(),
        Err(e) => return string_to_c_char(format!("Error: {}", e)),
    };

    let val_str = match unsafe { c_str_to_str(value_json) } {
        Ok(s) => s,
        Err(e) => return string_to_c_char(format!("Error: {}", e)),
    };

    let value: ParameterValue = match serde_json::from_str(val_str) {
        Ok(v) => v,
        Err(e) => return string_to_c_char(format!("Error deserializing parameter value: {}", e)),
    };

    let state_lock = APP_STATE.lock().unwrap();
    if let Some(ref state) = *state_lock {
        // Update current preset params
        {
            let mut params = state.current_preset_params.lock().unwrap();
            params.insert(param_str.clone(), value.clone());
        }

        // Update settings tweaks
        if let Ok(mut settings) = settings::load_settings() {
            let tweaks = settings.preset_tweaks.entry(name_str.clone()).or_insert_with(HashMap::new);
            tweaks.insert(param_str, value);
            let _ = settings::save_settings(&settings);
        }

        // Reapply preset
        let current_params = state.current_preset_params.lock().unwrap().clone();
        match apply_preset(name_str, current_params, state) {
            Ok(msg) => string_to_c_char(msg),
            Err(e) => string_to_c_char(format!("Error: {}", e)),
        }
    } else {
        string_to_c_char("Error: AppState not initialized".to_string())
    }
}

#[no_mangle]
pub extern "C" fn rgb_connect_keyboard(vid: u16, pid: u16) -> i32 {
    let state_lock = APP_STATE.lock().unwrap();
    if let Some(ref state) = *state_lock {
        let mut controller = state.controller.lock().unwrap();
        controller.disconnect();
        let _ = lighting::set_light_control_owner(true);
        match controller.connect_device(vid, pid) {
            Ok(_) => 1,
            Err(_) => 0,
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn rgb_set_light_control_owner(enable: i32) -> i32 {
    match lighting::set_light_control_owner(enable != 0) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "C" fn rgb_enable_dynamic_lighting() -> i32 {
    if lighting::enable_windows_lighting().is_ok() { 1 } else { 0 }
}

#[no_mangle]
pub extern "C" fn rgb_disable_dynamic_lighting() -> i32 {
    if lighting::disable_windows_lighting().is_ok() { 1 } else { 0 }
}

#[no_mangle]
pub extern "C" fn rgb_is_dynamic_lighting_enabled() -> i32 {
    if lighting::is_windows_lighting_enabled() { 1 } else { 0 }
}

#[no_mangle]
pub extern "C" fn rgb_set_lighting_priority() -> *mut c_char {
    match lighting::set_windows_lighting_on_top() {
        Ok(_) => string_to_c_char("Windows Dynamic Lighting Controller set to top priority.".to_string()),
        Err(e) => string_to_c_char(format!("Error: {}", e)),
    }
}

#[no_mangle]
pub extern "C" fn rgb_check_startup_installed() -> i32 {
    if installer::is_startup_task_installed() { 1 } else { 0 }
}

#[no_mangle]
pub extern "C" fn rgb_install_startup_task(delay_seconds: u32) -> *mut c_char {
    match installer::create_startup_task(delay_seconds) {
        Ok(_) => string_to_c_char("Startup task installed successfully.".to_string()),
        Err(e) => string_to_c_char(format!("Error: {}", e)),
    }
}

#[no_mangle]
pub extern "C" fn rgb_uninstall_startup_task() -> *mut c_char {
    match installer::remove_startup_task() {
        Ok(_) => string_to_c_char("Startup task uninstalled successfully.".to_string()),
        Err(e) => string_to_c_char(format!("Error: {}", e)),
    }
}

#[no_mangle]
pub extern "C" fn rgb_get_settings() -> *mut c_char {
    match settings::load_settings() {
        Ok(s) => {
            let json = serde_json::to_string(&s).unwrap_or_else(|_| "{}".to_string());
            string_to_c_char(json)
        }
        Err(e) => string_to_c_char(format!("Error: {}", e)),
    }
}

#[no_mangle]
pub extern "C" fn rgb_save_settings(settings_json: *const c_char) -> *mut c_char {
    let json_str = match unsafe { c_str_to_str(settings_json) } {
        Ok(s) => s,
        Err(e) => return string_to_c_char(format!("Error: {}", e)),
    };

    let settings_obj: settings::AppSettings = match serde_json::from_str(json_str) {
        Ok(s) => s,
        Err(e) => return string_to_c_char(format!("Error deserializing settings: {}", e)),
    };

    match settings::save_settings(&settings_obj) {
        Ok(_) => string_to_c_char("Settings saved successfully.".to_string()),
        Err(e) => string_to_c_char(format!("Error: {}", e)),
    }
}

#[no_mangle]
pub extern "C" fn rgb_cycle_preset() -> *mut c_char {
    let state_lock = APP_STATE.lock().unwrap();
    let state = match &*state_lock {
        Some(s) => s,
        None => return string_to_c_char("Error: AppState not initialized".to_string()),
    };

    let settings = match settings::load_settings() {
        Ok(s) => s,
        Err(e) => return string_to_c_char(format!("Error loading settings: {}", e)),
    };
    
    let effects = settings.preset_cycle_effects;
    if effects.is_empty() {
        return string_to_c_char("".to_string());
    }

    let current_index = state.preset_cycle_index.load(Ordering::SeqCst);
    let next_index = (current_index + 1) % effects.len();
    state.preset_cycle_index.store(next_index, Ordering::SeqCst);

    let next_effect = &effects[next_index];
    
    let parameters = settings
        .preset_tweaks
        .get(next_effect)
        .cloned()
        .unwrap_or_else(HashMap::new);
    
    match apply_preset(next_effect.clone(), parameters, state) {
        Ok(_) => string_to_c_char(next_effect.clone()),
        Err(e) => string_to_c_char(format!("Error: {}", e)),
    }
}

#[no_mangle]
pub extern "C" fn rgb_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

// ===================================================================
// EFFECT DISPATCH (PORTED FROM MAIN.RS)
// ===================================================================

fn apply_preset(
    preset_name: String,
    parameters: HashMap<String, ParameterValue>,
    state: &AppState,
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

    // Create new effect based on preset name (case-insensitive match)
    let preset_name_lc = preset_config.name.to_lowercase();
    let new_effect: Box<dyn Effect> = match preset_name_lc.as_str() {
        "staticcolor" => {
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
        "rainbowcycle" => {
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
        "rainbowwave" => {
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
        "ambient" => {
            #[cfg(not(target_os = "windows"))]
            return Err("Ambient effect is only supported on Windows".to_string());
            #[cfg(target_os = "windows")]
            {
                let smoothing: f32 = preset_config
                    .parameters
                    .get("smoothing")
                    .and_then(|v| match v {
                        ParameterValue::Float(f) => Some(*f),
                        _ => None,
                    })
                    .unwrap_or(1.0);

                let mut sampler = crate::presets::ambient::DxgiScreenSampler::new()
                    .map_err(|e| e.to_string())?;

                let preset_left = preset_config
                    .parameters
                    .get("sample_left")
                    .and_then(|v| match v { ParameterValue::Float(f) => Some(*f), _ => None });
                let preset_width = preset_config
                    .parameters
                    .get("sample_width")
                    .and_then(|v| match v { ParameterValue::Float(f) => Some(*f), _ => None });

                if let (Some(l), Some(w)) = (preset_left, preset_width) {
                    sampler.set_sample_horizontal_region(l, w);
                } else if let Ok(s) = crate::settings::load_settings() {
                    sampler.set_sample_horizontal_region(
                        s.ambient_sample_left_fraction,
                        s.ambient_sample_width_fraction,
                    );
                }

                Box::new(AmbientEffect::new(sampler, smoothing))
            }
        },
        "sweep" => Box::new(RgbSweepEffect::new()),
        "rainbowbreath" => {
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
        "thermalstatus" => {
            Box::new(ThermalStatusEffect::new())
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
        "horsecycle" => {
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
            Box::new(FerrariRpmEffect::new())
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
        "audio_sparkle" => {
            let sensitivity: f32 = preset_config
                .parameters
                .get("sensitivity")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(1.0);
            
            let base_density: f32 = preset_config
                .parameters
                .get("base_density")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(0.05);

            let sampler = crate::audio_sampler::AudioSampler::new()
                .map_err(|e| e.to_string())?;

            Box::new(AudioSparkleEffect::new(sampler, sensitivity, base_density))
        }
        "audio_sparkle_rainbow" => {
            let sensitivity: f32 = preset_config
                .parameters
                .get("sensitivity")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(1.0);
            
            let base_density: f32 = preset_config
                .parameters
                .get("base_density")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(0.0);

            let rainbow_speed: f32 = preset_config
                .parameters
                .get("rainbow_speed")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(1.0);

            let sampler = crate::audio_sampler::AudioSampler::new()
                .map_err(|e| e.to_string())?;

            Box::new(AudioSparkleRainbowEffect::new(sampler, sensitivity, base_density, rainbow_speed))
        }
        "audio_sparkle_media" => {
            let sensitivity: f32 = preset_config
                .parameters
                .get("sensitivity")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(1.0);
            
            let base_density: f32 = preset_config
                .parameters
                .get("base_density")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(0.0);

            let sampler_audio = crate::audio_sampler::AudioSampler::new()
                .map_err(|e| e.to_string())?;
            
            let mut sampler_media = crate::presets::ambient::DxgiScreenSampler::new()
                .map_err(|e| e.to_string())?;
            
            sampler_media.set_sample_top_fraction(0.15);
            sampler_media.set_sample_horizontal_region(0.0, 1.0);

            Box::new(AudioSparkleMediaEffect::new(sampler_audio, sampler_media, sensitivity, base_density))
        }
        "audio_ripple" => {
            let sensitivity: f32 = preset_config
                .parameters
                .get("sensitivity")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(1.0);
            let speed: f32 = preset_config
                .parameters
                .get("speed")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(40.0);
            let width: f32 = preset_config
                .parameters
                .get("width")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(3.0);
            let lifetime: f32 = preset_config
                .parameters
                .get("lifetime")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(0.8);

            let sampler = crate::audio_sampler::AudioSampler::new()
                .map_err(|e| e.to_string())?;

            Box::new(AudioRippleEffect::new(sampler, sensitivity, speed, width, lifetime))
        }
        "rainbow_ripple" => {
            let speed: f32 = preset_config
                .parameters
                .get("speed")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(40.0);
            
            let width: f32 = preset_config
                .parameters
                .get("width")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(3.0);

            let lifetime: f32 = preset_config
                .parameters
                .get("lifetime")
                .and_then(|v| match v {
                    ParameterValue::Float(f) => Some(*f),
                    _ => None,
                })
                .unwrap_or(0.8);

            Box::new(RainbowRippleEffect::new(speed, width, lifetime))
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
        "chromaticbreath" => {
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
