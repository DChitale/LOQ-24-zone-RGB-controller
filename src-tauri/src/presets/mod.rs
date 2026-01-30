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
            name: "heatwave".to_string(),
            display_name: "Heat Wave".to_string(),
            description: "Warm flowing wave".to_string(),
            parameters: vec![
                ParameterConfig {
                    name: "speed".to_string(),
                    label: "Speed".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.1,
                    max: 4.0,
                    default: 1.0,
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
            name: "ocean".to_string(),
            display_name: "Ocean Wave".to_string(),
            description: "Looks like rolling water across the keyboard.".to_string(),
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
            name: "energyPulse".to_string(),
            display_name: "Energy Pulse".to_string(),
            description: "Feels fast and punchy without being noisy.".to_string(),
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
            name: "fireFlow".to_string(),
            display_name: "Fire Flow".to_string(),
            description: "Looks alive. Surprisingly cheap to compute.".to_string(),
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
        PresetMetadata {
            name: "silk".to_string(),
            display_name: "Silk".to_string(),
            description: "Feels like light breathing through fabric, Slow hue drift + heavy smoothing.".to_string(),
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
            name: "egdeGlow".to_string(),
            display_name: "Liquid Edge Glow".to_string(),
            description: "Looks like light leaking from under glass, Center is calm, edges gently alive".to_string(),
            parameters: vec![
                ParameterConfig {
                    name: "none".to_string(),
                    label: "No adjustments available".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.0,
                    max: 0.0,
                    default: 0.0,
                    step: 0.0,
                },
            ],
        },
        PresetMetadata {
            name: "thermalStatus".to_string(),
            display_name: "Thermal Situation".to_string(),
            description: "Left => CPU, Right => GPU".to_string(),
            parameters: vec![
                ParameterConfig {
                    name: "none".to_string(),
                    label: "No adjustments available".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.0,
                    max: 0.0,
                    default: 0.0,
                    step: 0.0,
                },
            ],
        },
        PresetMetadata {
            name: "stillGradient".to_string(),
            display_name: "Still Gradient".to_string(),
            description: "Just perfectly distributed light, This is peak minimalism".to_string(),
            parameters: vec![
                ParameterConfig {
                    name: "color_a".to_string(),
                    label: "Color A".to_string(),
                    param_type: ParameterType::Color {r: 89, g: 108, b: 128},
                    min: 0.0,
                    max: 0.0,
                    default: 0.0,
                    step: 0.0,
                },
                ParameterConfig {
                    name: "color_b".to_string(),
                    label: "Color B".to_string(),
                    param_type: ParameterType::Color { r: 88, g: 75, b: 115 },
                    min: 0.0,  // Not used for colors
                    max: 255.0, // Not used for colors
                    default: 0.0, // Not used for colors
                    step: 1.0, // Not used for colors
                },
                ParameterConfig {
                    name: "middle".to_string(),
                    label: "Set the gradient mix".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.0,  // Not used for colors
                    max: 24.0, // Not used for colors
                    default: 12.0, // Not used for colors
                    step: 1.0, // Not used for colors
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

pub mod pulse;
pub mod scan;
pub mod heatwave;
pub mod sparkle;
pub mod aurora;
pub mod keyRipple;
pub mod ocean;
pub mod energyPulse;
pub mod nebula;
pub mod chromaticBreath;
pub mod fireFlow;
pub mod silk;
pub mod staticColor;
pub mod edgeGlow;
pub mod stillGradient;
//pub mod thermalStatus;