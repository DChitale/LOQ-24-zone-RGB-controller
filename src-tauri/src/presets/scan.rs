use crate::led_driver::{LedController, Color, NUM_ZONES};
use crate::effects::Effect;

pub struct ColorScanEffect {
    speed: f32,
}

impl ColorScanEffect {
    pub fn new(speed: f32) -> Self {
        ColorScanEffect { speed }
    }
}

impl Effect for ColorScanEffect {
    fn update(&mut self, controller: &mut LedController, time: f32, _delta: f32) {
        controller.fill(Color::black());

        let pos = ((time * self.speed * 10.0) as usize) % (NUM_ZONES * 2);
        let index = if pos < NUM_ZONES {
            pos
        } else {
            NUM_ZONES * 2 - pos - 1
        };

        let hue = (time * 120.0) % 360.0;
        let color = Color::from_hsv(hue, 1.0, 1.0);

        controller.set_zone(index, color);
        let _ = controller.flush_buffered();
    }

    fn name(&self) -> &str {
        "Color Scan"
    }
}
