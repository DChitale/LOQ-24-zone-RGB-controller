use crate::led_driver::{LedController, Color, NUM_ZONES};
use crate::effects::Effect;

pub struct OceanWaveEffect {
    speed: f32,
}

impl OceanWaveEffect {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }
}

impl Effect for OceanWaveEffect {
    fn update(&mut self, controller: &mut LedController, time: f32, _delta: f32) {
        for i in 0..NUM_ZONES {
            let pos = i as f32 / NUM_ZONES as f32;

            let wave1 = (time * self.speed + pos * 6.0).sin();
            let wave2 = (time * self.speed * 0.6 + pos * 12.0).sin();

            let intensity = ((wave1 + wave2) * 0.25 + 0.5).clamp(0.0, 1.0);

            let color = Color::from_hsv(200.0, 0.8, intensity);
            controller.set_zone(i, color);
        }

        let _ = controller.flush_buffered();
    }

    fn name(&self) -> &str {
        "Ocean Wave"
    }
}
