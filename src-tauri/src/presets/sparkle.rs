use crate::led_driver::{LedController, Color, NUM_ZONES};
use crate::effects::Effect;

pub struct SparkleEffect {
    density: f32,
}

impl SparkleEffect {
    pub fn new(density: f32) -> Self {
        SparkleEffect { density }
    }
}

impl Effect for SparkleEffect {
    fn update(&mut self, controller: &mut LedController, time: f32, _delta: f32) {
        controller.fill(Color::black());

        for i in 0..NUM_ZONES {
            let hash = ((i as f32 * 12.9898 + time * 78.233).sin() * 43758.5453).fract();
            if hash > 1.0 - self.density {
                controller.set_zone(i, Color::white());
            }
        }

        let _ = controller.flush_buffered();
    }

    fn name(&self) -> &str {
        "Sparkle"
    }
}
