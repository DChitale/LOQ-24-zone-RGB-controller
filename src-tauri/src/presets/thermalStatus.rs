use crate::led_driver::{LedController, Color};
use crate::effects::Effect;

use sysinfo::System;
use nvml_wrapper::Nvml;
use nvml_wrapper::enum_wrappers::device::TemperatureSensor;

pub struct StrictSystemMonitorEffect {
    // ---- system ----
    sys: System,
    total_memory: u64,
    cores: usize,

    // ---- nvml ----
    nvml: Nvml,

    // ---- cached values ----
    cpu_usage: f32,
    mem_usage: f32,
    gpu_usage: f32,
}

impl StrictSystemMonitorEffect {
    pub fn new() -> Self {
        let mut sys = System::new();
        sys.refresh_all();

        let total_memory = sys.total_memory();
        let cores = sys.cpus().len();

        let nvml = Nvml::init()
            .expect("Failed to initialize NVML. Make sure NVIDIA drivers are installed.");

        Self {
            sys,
            total_memory,
            cores,
            nvml,
            cpu_usage: 0.0,
            mem_usage: 0.0,
            gpu_usage: 0.0,
        }
    }

    fn read_metrics(&mut self) {
        // === EXACT SYSTEM LOGIC ===
        self.sys.refresh_all();

        let mut total_usage = 0.0;
        for cpu in self.sys.cpus() {
            total_usage += cpu.cpu_usage() as f32;
        }
        let usage = total_usage / self.cores as f32;
        self.cpu_usage = (usage / 100.0).clamp(0.0, 1.0);

        self.mem_usage =
            (self.sys.used_memory() as f32 / self.total_memory as f32)
                .clamp(0.0, 1.0);

        // === GPU USAGE (NOT TEMP) ===
        if let Ok(device) = self.nvml.device_by_index(0) {
            if let Ok(util) = device.utilization_rates() {
                self.gpu_usage = (util.gpu as f32 / 100.0).clamp(0.0, 1.0);
            }
        }
    }
}

impl Effect for StrictSystemMonitorEffect {
    fn start(&mut self) {
        self.read_metrics();
    }

    fn update(
        &mut self,
        controller: &mut LedController,
        _time: f32,
        _delta: f32,
    ) {
        self.read_metrics();

        // CPU | MEM | GPU (8 zones each, solid blocks)
        draw_block(controller, 0,  self.cpu_usage);
        draw_block(controller, 8,  self.mem_usage);
        draw_block(controller, 16, self.gpu_usage);

        let _ = controller.flush_buffered();
    }

    fn name(&self) -> &str {
        "Strict System Monitor"
    }
}

// =======================================================
// VISUALS
// =======================================================

fn draw_block(
    controller: &mut LedController,
    start: usize,
    value: f32,
) {
    let color = value_to_color(value);

    for i in 0..8 {
        controller.set_zone(start + i, color);
    }
}

fn value_to_color(v: f32) -> Color {
    let v = v.clamp(0.0, 1.0);

    let blue   = Color::from_hsv(220.0, 1.0, 1.0);
    let green  = Color::from_hsv(120.0, 1.0, 1.0);
    let yellow = Color::from_hsv(60.0,  1.0, 1.0);
    let red    = Color::from_hsv(0.0,   1.0, 1.0);

    if v < 0.33 {
        blue.lerp(&green, v / 0.33)
    } else if v < 0.66 {
        green.lerp(&yellow, (v - 0.33) / 0.33)
    } else {
        yellow.lerp(&red, (v - 0.66) / 0.34)
    }
}

// Public alias expected by the rest of the codebase
pub type ThermalStatusEffect = StrictSystemMonitorEffect;
