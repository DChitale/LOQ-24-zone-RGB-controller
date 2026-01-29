use crate::led_driver::{LedController, Color, NUM_ZONES};
use crate::effects::Effect;

pub struct NebulaEffect {
    speed: f32,
}

impl NebulaEffect {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }
}

impl Effect for NebulaEffect {
    fn update(&mut self, controller: &mut LedController, time: f32, _delta: f32) {
        for i in 0..NUM_ZONES {
            let pos = i as f32 / NUM_ZONES as f32;
            let drift = (time * self.speed + pos * 3.0).sin();

            let hue = 260.0 + drift * 40.0;
            let brightness = (drift * 0.4 + 0.6).clamp(0.0, 1.0);

            controller.set_zone(
                i,
                Color::from_hsv(hue, 0.7, brightness),
            );
        }

        let _ = controller.flush_buffered();
    }

    fn name(&self) -> &str {
        "Nebula"
    }
}
