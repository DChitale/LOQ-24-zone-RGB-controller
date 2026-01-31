use crate::led_driver::{LedController, Color};
use crate::effects::Effect;

pub struct ColorBreathEffect {
    color: Color,
    speed: f32,
}

impl ColorBreathEffect {
    pub fn new(color: Color, speed: f32) -> Self {
        Self { color, speed }
    }
}

impl Effect for ColorBreathEffect {
    fn update(&mut self, controller: &mut LedController, time: f32, _delta: f32) {
        let brightness =
            ((time * self.speed * std::f32::consts::PI * 2.0).sin() + 1.0) / 2.0;

        let _ = controller.set_all_instant(self.color.scale(brightness));
    }

    fn name(&self) -> &str {
        "Color Breath"
    }
}
