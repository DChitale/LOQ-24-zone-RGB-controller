use crate::led_driver::{LedController, Color};
use crate::effects::Effect;

pub struct RgbSweepEffect {
    value: u32,
}

impl RgbSweepEffect {
    pub fn new() -> Self {
        Self { value: 0 }
    }
}

impl Effect for RgbSweepEffect {
    fn start(&mut self) {
        self.value = 0;
    }

    fn update(&mut self, controller: &mut LedController, _time: f32, _delta: f32) {
        // Extract RGB from 24-bit counter
        let r = ((self.value >> 16) & 0xFF) as u8;
        let g = ((self.value >> 8) & 0xFF) as u8;
        let b = (self.value & 0xFF) as u8;

        let color = Color::new(r, g, b);

        // Apply same color to all zones
        let _ = controller.set_all_instant(color);

        // Advance to next color (wrap at 24-bit max)
        self.value = (self.value + 1) & 0x00FF_FFFF;
    }

    fn name(&self) -> &str {
        "RGB 16M Sweep"
    }
}
