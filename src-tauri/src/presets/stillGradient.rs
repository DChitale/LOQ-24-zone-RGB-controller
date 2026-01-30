use crate::led_driver::{LedController, Color, NUM_ZONES};
use crate::effects::Effect;

pub struct StillGradientEffect {
    color_a: Color,
    color_b: Color,
    middle: f32,
}

impl StillGradientEffect {
    pub fn new(color_a: Color, color_b: Color, middle: f32) -> Self {
        Self { color_a, color_b, middle }
    }
}

impl Effect for StillGradientEffect {
    fn update(&mut self, controller: &mut LedController, _time: f32, _delta: f32) {
        let middle = self.middle as usize;
        for i in 0..NUM_ZONES {
            let t = if i < middle {
                // Left side: interpolate from 0 to 1 as we approach middle from left
                i as f32 / middle as f32 * 0.5
            } else {
                // Right side: interpolate from 0.5 to 1 as we go from middle to right
                0.5 + (i - middle) as f32 / (NUM_ZONES - middle) as f32 * 0.5
            };
            let color = self.color_a.lerp(&self.color_b, t);
            controller.set_zone(i, color);
        }

        let _ = controller.flush_buffered();
    }

    fn name(&self) -> &str {
        "Still Gradient"
    }
}
