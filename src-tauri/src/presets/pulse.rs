use crate::led_driver::{LedController, Color, NUM_ZONES};
use crate::effects::{Effect, distance_from_center};

pub struct PulseCenterEffect {
    color: Color,
    speed: f32,
}

impl PulseCenterEffect {
    pub fn new(color: Color, speed: f32) -> Self {
        PulseCenterEffect { color, speed }
    }
}

impl Effect for PulseCenterEffect {
    fn update(&mut self, controller: &mut LedController, time: f32, _delta: f32) {
        let pulse = (time * self.speed).fract();

        for i in 0..NUM_ZONES {
            let d = distance_from_center(i);
            let brightness = (1.0 - (pulse - d).abs() * 4.0).clamp(0.0, 1.0);
            controller.set_zone(i, self.color.scale(brightness));
        }

        let _ = controller.flush_buffered();
    }

    fn name(&self) -> &str {
        "Pulse Center"
    }
}
