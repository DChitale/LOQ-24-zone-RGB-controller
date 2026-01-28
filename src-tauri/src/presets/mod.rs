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
    vec![
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