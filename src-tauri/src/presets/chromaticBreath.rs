use crate::led_driver::{LedController, Color};
use crate::effects::Effect;

pub struct ChromaticBreathEffect {
    speed: f32,
}

impl ChromaticBreathEffect {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }
}

impl Effect for ChromaticBreathEffect {
    fn update(&mut self, controller: &mut LedController, time: f32, _delta: f32) {
        // Large discrete palette (one color breathes at a time)
        const COLORS: [f32; 20] = [
            0.0,    // Red
            305.0,  // Magenta
            24.0,   // Deep Orange
            265.0,  // Indigo
            48.0,   // Amber
            225.0,  // Blue
            75.0,   // Lime
            185.0,  // Cyan
            120.0,  // Green
            
            165.0,  // Teal
            145.0,  // Spring Green
            205.0,  // Sky Blue
            95.0,   // Yellow-Green
            245.0,  // Deep Blue
            60.0,   // Yellow
            285.0,  // Purple
            36.0,   // Orange
            325.0,  // Pink
            12.0,   // Crimson
            345.0,  // Rose
        ];

        let cycle_time = 8.0 / self.speed;
        let total_time = COLORS.len() as f32 * cycle_time;

        let t = time % total_time;
        let color_index = (t / cycle_time) as usize;
        let local_t = (t % cycle_time) / cycle_time;

        // Smooth inhale → exhale
        let breath = (local_t * std::f32::consts::PI).sin();

        let hue = COLORS[color_index % COLORS.len()];
        let color = Color::from_hsv(hue, 1.0, breath);

        let _ = controller.set_all_instant(color);
    }

    fn name(&self) -> &str {
        "Chromatic Breath"
    }
}
