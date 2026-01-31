use crate::led_driver::{LedController, Color, NUM_ZONES};
use crate::effects::Effect;

pub struct RainbowWaveEffect {
    speed: f32,
}

impl RainbowWaveEffect {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }
}

impl Effect for RainbowWaveEffect {
    fn update(&mut self, controller: &mut LedController, time: f32, _delta: f32) {
        for i in 0..NUM_ZONES {
            let hue = ((i as f32 / NUM_ZONES as f32) * 360.0
                + time * self.speed * 60.0)
                % 360.0;

            controller.set_zone(i, Color::from_hsv(hue, 1.0, 1.0));
        }

        let _ = controller.flush_buffered();
    }

    fn name(&self) -> &str {
        "Rainbow Wave"
    }
}
