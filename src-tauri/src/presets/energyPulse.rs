use crate::led_driver::{LedController, Color, NUM_ZONES};
use crate::effects::Effect;

pub struct EnergyPulseEffect {
    speed: f32,
}

impl EnergyPulseEffect {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }
}

impl Effect for EnergyPulseEffect {
    fn update(&mut self, controller: &mut LedController, time: f32, _delta: f32) {
        let center = (time * self.speed).fract();

        for i in 0..NUM_ZONES {
            let pos = i as f32 / NUM_ZONES as f32;
            let dist = (pos - center).abs();

            let intensity = (1.0 - dist * 5.0).clamp(0.0, 1.0);
            let color = Color::from_hsv(30.0, 1.0, intensity);

            controller.set_zone(i, color);
        }

        let _ = controller.flush_buffered();
    }

    fn name(&self) -> &str {
        "Energy Pulse"
    }
}
