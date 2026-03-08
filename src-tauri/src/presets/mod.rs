use serde::{Serialize, Deserialize};
use crate::AppState;
use tauri::State;
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ParameterConfig {
    pub name: String,
    pub label: String,
    pub param_type: ParameterType,
    pub min: f32,
    pub max: f32,
    pub default: f32,
    pub step: f32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ParameterType {
    Float,
    #[serde(rename = "Color")]
    Color { r: u8, g: u8, b: u8 },
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PresetMetadata {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub parameters: Vec<ParameterConfig>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PresetConfig {
    pub name: String,
    pub parameters: HashMap<String, ParameterValue>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "value")]
pub enum ParameterValue {
    Float(f32),
    Color { r: u8, g: u8, b: u8 },
}

pub fn get_available_presets() -> Vec<PresetMetadata> {
    vec![PresetMetadata {
            name: "staticColor".to_string(),
            display_name: "Static Color".to_string(),
            description: "Set a static color from 16,581,375 gradients of color".to_string(),
            parameters: vec![
                ParameterConfig {
                    name: "color".to_string(),
                    label: "Color".to_string(),
                    param_type: ParameterType::Color { r: 255, g: 255, b: 200},
                    min: 0.1,
                    max: 3.0,
                    default: 0.5,
                    step: 0.1,
                },
            ],
        },
        PresetMetadata {
            name: "thermalStatus".to_string(),
            display_name: "CPU-Mem-GPU usage status".to_string(),
            description: "Left => CPU, Middle => Memory, Right => GPU".to_string(),
            parameters: vec![],
        },
        PresetMetadata {
            name: "off".to_string(),
            display_name: "Off".to_string(),
            description: "Turn off all lighting".to_string(),
            parameters: vec![],
        },
        PresetMetadata {
            name: "ambient".to_string(),
            display_name: "Screen Ambiance light effect.".to_string(),
            description: "Mimics ambient light based on screen content.".to_string(),
            parameters: vec![
                ParameterConfig
                {name: "speed".to_string(),
                    label: "Speed".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.1,
                    max: 5.0,
                    default: 1.0,
                    step: 0.1,},
                ParameterConfig {
                    name: "smoothing".to_string(),
                    label: "Smoothing".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.0,
                    max: 10.0,
                    default: 5.0,
                    step: 0.1,
                },
                // Sampling region controls (fractions where 0.0 = top/left, 1.0 = bottom/right)
                ParameterConfig {
                    name: "sample_left".to_string(),
                    label: "Screen sample left".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.0,
                    max: 1.0,
                    default: 0.0,
                    step: 0.01,
                },
                ParameterConfig {
                    name: "sample_width".to_string(),
                    label: "Screen sample width".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.01,
                    max: 1.0,
                    default: 1.0,
                    step: 0.01,
                },
            ],
        },
        PresetMetadata {
            name: "breathing".to_string(),
            display_name: "Color Breath".to_string(),
            description: "Fade in → fade out.".to_string(),
            parameters: vec![
                ParameterConfig {
                    name: "speed".to_string(),
                    label: "Speed".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.1,
                    max: 5.0,
                    default: 1.0,
                    step: 0.1,
                },
                ParameterConfig {
                    name: "color".to_string(),
                    label: "Color".to_string(),
                    param_type: ParameterType::Color { r: 255, g: 0, b: 0 },
                    min: 0.0,  // Not used for colors
                    max: 255.0, // Not used for colors
                    default: 0.0, // Not used for colors
                    step: 1.0, // Not used for colors
                },
            ],
        },
        PresetMetadata {
            name: "pulse".to_string(),
            display_name: "Pulse Center".to_string(),
            description: "Pulsing effect from center".to_string(),
            parameters: vec![
                ParameterConfig {
                    name: "speed".to_string(),
                    label: "Speed".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.1,
                    max: 5.0,
                    default: 1.0,
                    step: 0.1,
                },
                ParameterConfig {
                    name: "color".to_string(),
                    label: "Color".to_string(),
                    param_type: ParameterType::Color { r: 255, g: 0, b: 0 },
                    min: 0.0,  // Not used for colors
                    max: 255.0, // Not used for colors
                    default: 0.0, // Not used for colors
                    step: 1.0, // Not used for colors
                },
            ],
        },
        PresetMetadata {
            name: "horse".to_string(),
            display_name: "Horse Color".to_string(),
            description: "A sharp chaser segment racing across a solid base color — fast, focused, and minimal.".to_string(),
            parameters: vec![
                ParameterConfig {
                    name: "speed".to_string(),
                    label: "Speed".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.1,
                    max: 10.0,
                    default: 5.0,
                    step: 0.1,
                },
                ParameterConfig {
                    name: "length".to_string(),
                    label: "Length".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.1,
                    max: 10.0,
                    default: 5.0,
                    step: 0.1,
                },
                ParameterConfig {
                    name: "base_color".to_string(),
                    label: "Base Color".to_string(),
                    param_type: ParameterType::Color { r: 81, g: 169, b: 158 },
                    min: 0.0,  // Not used for colors
                    max: 255.0, // Not used for colors
                    default: 0.0, // Not used for colors
                    step: 1.0, // Not used for colors
                },
                ParameterConfig {
                    name: "horse_color".to_string(),
                    label: "Horse Color".to_string(),
                    param_type: ParameterType::Color { r: 255, g: 0, b: 0 },
                    min: 0.0,  // Not used for colors
                    max: 255.0, // Not used for colors
                    default: 0.0, // Not used for colors
                    step: 1.0, // Not used for colors
                },
            ],
        },
        PresetMetadata {
            name: "horseCycle".to_string(),
            display_name: "Horse Cycle".to_string(),
            description: "A racing chaser over a smoothly color-cycling base, blending calm ambience with high-energy motion.".to_string(),
            parameters: vec![
                ParameterConfig {
                    name: "speed".to_string(),
                    label: "Speed".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.1,
                    max: 10.0,
                    default: 5.0,
                    step: 0.1,
                },
                ParameterConfig {
                    name: "length".to_string(),
                    label: "Length".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.1,
                    max: 10.0,
                    default: 5.0,
                    step: 0.1,
                },
            ],
        },
        PresetMetadata {
            name: "rpm".to_string(),
            display_name: "Ferrari RPM".to_string(),
            description: "Ferrari-like = fast, aggressive, red-dominant, precision motion — not rainbow fluff.".to_string(),
            parameters: vec![],
        },
        PresetMetadata {
            name: "rainbowBreath".to_string(),
            display_name: "Rainbow Breath".to_string(),
            description: "Whole keyboard breathes through rainbow hues".to_string(),
            parameters: vec![
                ParameterConfig {
                    name: "speed".to_string(),
                    label: "Speed".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.1,
                    max: 10.0,
                    default: 1.0,
                    step: 0.01,
                },
            ],
        },
        PresetMetadata {
            name: "rainbowCycle".to_string(),
            display_name: "Rainbow Cycle".to_string(),
            description: "Whole keyboard cycles through hues together.".to_string(),
            parameters: vec![
                ParameterConfig {
                    name: "speed".to_string(),
                    label: "Speed".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.1,
                    max: 10.0,
                    default: 1.0,
                    step: 0.01,
                },
            ],
        },
        PresetMetadata {
            name: "rainbowWave".to_string(),
            display_name: "Rainbow Wave".to_string(),
            description: "Left → right rainbow motion.".to_string(),
            parameters: vec![
                ParameterConfig {
                    name: "speed".to_string(),
                    label: "Speed".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.1,
                    max: 10.0,
                    default: 1.0,
                    step: 0.01,
                },
            ],
        },
        PresetMetadata {
            name: "wheel".to_string(),
            display_name: "ColorWheelEffect".to_string(),
            description: "Each zone has a fixed hue offset → whole keyboard spins like a wheel".to_string(),
            parameters: vec![
                ParameterConfig {
                    name: "speed".to_string(),
                    label: "Speed".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.1,
                    max: 3.0,
                    default: 0.5,
                    step: 0.1,
                },
            ],
        },
        PresetMetadata {
            name: "sweep".to_string(),
            display_name: "Color sweep".to_string(),
            description: "Cycles through all 16.7 million colors one per frame, completing a full loop in about 3.5 days.".to_string(),
            parameters: vec![],
        },
        
        PresetMetadata {
            name: "aurora".to_string(),
            display_name: "Aurora".to_string(),
            description: "Flowing aurora effect".to_string(),
            parameters: vec![
                ParameterConfig {
                    name: "speed".to_string(),
                    label: "Speed".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.1,
                    max: 3.0,
                    default: 0.5,
                    step: 0.1,
                },
            ],
        },
        PresetMetadata {
            name: "scan".to_string(),
            display_name: "Color Scan".to_string(),
            description: "Scanning color effect".to_string(),
            parameters: vec![
                ParameterConfig {
                    name: "speed".to_string(),
                    label: "Speed".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.1,
                    max: 3.0,
                    default: 1.0,
                    step: 0.1,
                },
            ],
        },
        PresetMetadata {
            name: "sparkle".to_string(),
            display_name: "Sparkle".to_string(),
            description: "Random sparkling effect".to_string(),
            parameters: vec![
                ParameterConfig {
                    name: "density".to_string(),
                    label: "Density".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.01,
                    max: 0.5,
                    default: 0.1,
                    step: 0.01,
                },
            ],
        },
        PresetMetadata {
            name: "audio_sparkle".to_string(),
            display_name: "Audio Sparkle".to_string(),
            description: "Keyboard lights sparkle in sync with system audio.".to_string(),
            parameters: vec![
                ParameterConfig {
                    name: "sensitivity".to_string(),
                    label: "Sensitivity".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.1,
                    max: 10.0,
                    default: 1.0,
                    step: 0.1,
                },
                ParameterConfig {
                    name: "base_density".to_string(),
                    label: "Base Density".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.0,
                    max: 0.5,
                    default: 0.0,
                    step: 0.01,
                },
            ],
        },
        PresetMetadata {
            name: "audio_sparkle_rainbow".to_string(),
            display_name: "Audio Sparkle Rainbow".to_string(),
            description: "Rainbow sparkles that react to system audio.".to_string(),
            parameters: vec![
                ParameterConfig {
                    name: "sensitivity".to_string(),
                    label: "Sensitivity".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.1,
                    max: 10.0,
                    default: 1.0,
                    step: 0.1,
                },
                ParameterConfig {
                    name: "base_density".to_string(),
                    label: "Base Density".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.0,
                    max: 0.5,
                    default: 0.0,
                    step: 0.01,
                },
                ParameterConfig {
                    name: "rainbow_speed".to_string(),
                    label: "Rainbow Speed".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.1,
                    max: 5.0,
                    default: 1.0,
                    step: 0.1,
                },
            ],
        },
        PresetMetadata {
            name: "audio_sparkle_media".to_string(),
            display_name: "Audio Sparkle Media".to_string(),
            description: "Sparkles that match screen colors and react to audio.".to_string(),
            parameters: vec![
                ParameterConfig {
                    name: "sensitivity".to_string(),
                    label: "Sensitivity".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.1,
                    max: 10.0,
                    default: 1.0,
                    step: 0.1,
                },
                ParameterConfig {
                    name: "base_density".to_string(),
                    label: "Base Density".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.0,
                    max: 0.5,
                    default: 0.0,
                    step: 0.01,
                },
            ],
        },
        PresetMetadata {
            name: "audio_ripple".to_string(),
            display_name: "Audio Ripple".to_string(),
            description: "Ripples flash from the center on audio beats.".to_string(),
            parameters: vec![
                ParameterConfig {
                    name: "sensitivity".to_string(),
                    label: "Sensitivity".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.1,
                    max: 10.0,
                    default: 1.0,
                    step: 0.1,
                },
                ParameterConfig {
                    name: "speed".to_string(),
                    label: "Speed".to_string(),
                    param_type: ParameterType::Float,
                    min: 10.0,
                    max: 100.0,
                    default: 40.0,
                    step: 1.0,
                },
                ParameterConfig {
                    name: "width".to_string(),
                    label: "Width".to_string(),
                    param_type: ParameterType::Float,
                    min: 1.0,
                    max: 10.0,
                    default: 3.0,
                    step: 0.1,
                },
                ParameterConfig {
                    name: "lifetime".to_string(),
                    label: "Lifetime".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.1,
                    max: 2.0,
                    default: 0.8,
                    step: 0.1,
                },
            ],
        },
        PresetMetadata {
            name: "rainbow_ripple".to_string(),
            display_name: "Typing Rainbow Ripple".to_string(),
            description: "Rainbow waves that expand from the keys you press.".to_string(),
            parameters: vec![
                ParameterConfig {
                    name: "speed".to_string(),
                    label: "Speed".to_string(),
                    param_type: ParameterType::Float,
                    min: 10.0,
                    max: 100.0,
                    default: 40.0,
                    step: 1.0,
                },
                ParameterConfig {
                    name: "width".to_string(),
                    label: "Width".to_string(),
                    param_type: ParameterType::Float,
                    min: 1.0,
                    max: 10.0,
                    default: 3.0,
                    step: 0.1,
                },
                ParameterConfig {
                    name: "lifetime".to_string(),
                    label: "Lifetime".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.1,
                    max: 2.0,
                    default: 0.8,
                    step: 0.1,
                },
            ],
        },
        PresetMetadata {
            name: "nebula".to_string(),
            display_name: "Nebula".to_string(),
            description: "Soft, atmospheric, zero harsh transitions.".to_string(),
            parameters: vec![
                ParameterConfig {
                    name: "speed".to_string(),
                    label: "Speed".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.1,
                    max: 10.0,
                    default: 1.0,
                    step: 0.01,
                },
            ],
        },
        
        PresetMetadata {
            name: "chromaticBreath".to_string(),
            display_name: "Chromatic Breath".to_string(),
            description: "Extremely clean, perfect for idle mode.".to_string(),
            parameters: vec![
                ParameterConfig {
                    name: "speed".to_string(),
                    label: "Speed".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.1,
                    max: 10.0,
                    default: 1.0,
                    step: 0.01,
                },
            ],
        },
    ]
}

pub fn get_preset_metadata() -> Vec<PresetMetadata> {
    get_available_presets()
}

pub fn set_preset(preset_config: PresetConfig, _state: State<AppState>) -> Result<String, String> {
    // This will be implemented to actually create and set the effect
    Ok(format!("Preset '{}' loaded with {} parameters", preset_config.name, preset_config.parameters.len()))
}

pub fn adjust_preset_parameter(_preset_name: String, _param_name: String, _value: ParameterValue, _state: State<AppState>) -> Result<(), String> {
    // This will be implemented to adjust running effect parameters
    Ok(())
}

pub mod off;
pub mod pulse;
pub mod scan;
pub mod sparkle;
pub mod aurora;
pub mod keyRipple;
pub mod energyPulse;
pub mod nebula;
pub mod chromaticBreath;
pub mod fireFlow;
pub mod staticColor;
pub mod edgeGlow;
pub mod rainbowCycle;
pub mod rainbowWave;
pub mod breathing;
pub mod rainbowBreath;
pub mod wheel;
pub mod sweep;
pub mod horse;
pub mod horseCycle;
pub mod rpm;
pub mod thermalStatus;
pub mod ambient;
pub mod audio_sparkle;
pub mod audio_sparkle_rainbow;
pub mod audio_sparkle_media;
pub mod rainbow_ripple;
pub mod audio_ripple;