use crate::led_driver::{LedController, Color};
use crate::effects::Effect;

pub struct RainbowCycleEffect {
    speed: f32,
}

impl RainbowCycleEffect {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }
}

impl Effect for RainbowCycleEffect {
    fn update(&mut self, controller: &mut LedController, time: f32, _delta: f32) {
        let hue = (time * self.speed * 60.0) % 360.0;
        let color = Color::from_hsv(hue, 1.0, 1.0);

        let _ = controller.set_all_instant(color);
    }

    fn name(&self) -> &str {
        "Rainbow Cycle"
    }
}
