use crate::led_driver::{LedController, Color, NUM_ZONES};
use crate::effects::{Effect, distance_from_center};

pub struct LiquidEdgeEffect {
    hue: f32,
}

impl LiquidEdgeEffect {
    pub fn new() -> Self {
        Self { hue: 200.0 }
    }
}

impl Effect for LiquidEdgeEffect {
    fn update(&mut self, controller: &mut LedController, time: f32, _delta: f32) {
        self.hue = 200.0 + (time * 8.0).sin() * 15.0;

        for i in 0..NUM_ZONES {
            let dist = distance_from_center(i);
            let brightness = (dist * 0.6).clamp(0.1, 0.6);

            let color = Color::from_hsv(self.hue, 0.4, brightness);
            controller.set_zone(i, color);
        }

        let _ = controller.flush_buffered();
    }

    fn name(&self) -> &str {
        "Liquid Edge"
    }
}
