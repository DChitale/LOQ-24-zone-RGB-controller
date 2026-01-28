use crate::led_driver::{LedController, Color, NUM_ZONES};
use crate::effects::{Effect, wave};

pub struct HeatWaveEffect {
    speed: f32,
}

impl HeatWaveEffect {
    pub fn new(speed: f32) -> Self {
        HeatWaveEffect { speed }
    }
}

impl Effect for HeatWaveEffect {
    fn update(&mut self, controller: &mut LedController, time: f32, _delta: f32) {
        for i in 0..NUM_ZONES {
            let pos = i as f32 / NUM_ZONES as f32;
            let w = wave(pos, time * self.speed, 2.0);
            let intensity = ((w + 1.0) / 2.0).clamp(0.0, 1.0);

            let color = Color::from_hsv(
                20.0 + intensity * 40.0, // red → yellow
                1.0,
                intensity,
            );

            controller.set_zone(i, color);
        }

        let _ = controller.flush_buffered();
    }

    fn name(&self) -> &str {
        "Heat Wave"
    }
}
