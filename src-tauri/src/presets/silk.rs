use crate::led_driver::{LedController, Color, NUM_ZONES};
use crate::effects::Effect;

pub struct SilkAmbientEffect {
    base_hue: f32,
    speed: f32,
}

impl SilkAmbientEffect {
    pub fn new(speed: f32) -> Self {
        Self {
            base_hue: 0.0,
            speed,
        }
    }
}

impl Effect for SilkAmbientEffect {
    fn update(&mut self, controller: &mut LedController, _time: f32, delta: f32) {
        self.base_hue = (self.base_hue + delta * self.speed * 10.0) % 360.0;

        for i in 0..NUM_ZONES {
            let offset = i as f32 / NUM_ZONES as f32 * 20.0;
            let hue = (self.base_hue + offset) % 360.0;

            let color = Color::from_hsv(hue, 0.35, 0.6);
            controller.set_zone(i, color);
        }

        let _ = controller.flush_buffered();
    }

    fn name(&self) -> &str {
        "Silk Ambient"
    }
}
