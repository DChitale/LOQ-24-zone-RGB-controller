use crate::led_driver::{LedController, Color, NUM_ZONES};
use crate::effects::Effect;

pub struct ColorWheelEffect {
    speed: f32,
}

impl ColorWheelEffect {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }
}

impl Effect for ColorWheelEffect {
    fn update(&mut self, controller: &mut LedController, time: f32, _delta: f32) {
        let base_hue = (time * self.speed * 60.0) % 360.0;

        // Narrow hue spread to look like a rotating wheel, not a rainbow
        let wheel_width = 60.0; // degrees of hue across the whole wheel

        for i in 0..NUM_ZONES {
            let offset = (i as f32 / NUM_ZONES as f32) * wheel_width;
            let hue = (base_hue + offset) % 360.0;
            controller.set_zone(i, Color::from_hsv(hue, 1.0, 1.0));
        }

        let _ = controller.flush_buffered();
    }

    fn name(&self) -> &str {
        "Color Wheel"
    }
}

