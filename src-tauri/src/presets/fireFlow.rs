use crate::led_driver::{LedController, Color, NUM_ZONES};
use crate::effects::Effect;

pub struct FireFlowEffect {
    speed: f32,
}

impl FireFlowEffect {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }
}

impl Effect for FireFlowEffect {
    fn update(&mut self, controller: &mut LedController, time: f32, _delta: f32) {
        for i in 0..NUM_ZONES {
            let pos = i as f32 / NUM_ZONES as f32;
            let flicker = ((time * self.speed + pos * 10.0).sin() + 1.0) * 0.5;

            let hue = 10.0 + flicker * 40.0;
            let brightness = (0.4 + flicker * 0.6).clamp(0.0, 1.0);

            controller.set_zone(
                i,
                Color::from_hsv(hue, 1.0, brightness),
            );
        }

        let _ = controller.flush_buffered();
    }

    fn name(&self) -> &str {
        "Fire Flow"
    }
}
