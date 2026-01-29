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
        let breath = ((time * self.speed).sin() + 1.0) * 0.5;
        let hue = (time * 40.0) % 360.0;

        let color = Color::from_hsv(hue, 1.0, breath);
        let _ = controller.set_all_instant(color);
    }

    fn name(&self) -> &str {
        "Chromatic Breath"
    }
}
