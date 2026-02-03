use crate::led_driver::{LedController, Color, NUM_ZONES};
use crate::effects::{Effect, distance_from_center};

pub struct PulseCenterEffect  {
    color: Color,
    speed: f32,
}

impl PulseCenterEffect  {
    pub fn new(color: Color, speed: f32) -> Self {
        PulseCenterEffect  { color, speed }
    }
}

impl Effect for PulseCenterEffect  {
    fn update(&mut self, controller: &mut LedController, time: f32, _delta: f32) {
        for i in 0..NUM_ZONES {
            let d = distance_from_center(i);

            // Moving wave from center outward
            let wave = ((d * 3.0 - time * self.speed).cos() * 0.5 + 0.5).powf(2.0);

            // Apply soft gradient color
            controller.set_zone(i, self.color.scale(wave));
        }

        let _ = controller.flush_buffered();
    }

    fn name(&self) -> &str {
        "Center Wave"
    }
}
