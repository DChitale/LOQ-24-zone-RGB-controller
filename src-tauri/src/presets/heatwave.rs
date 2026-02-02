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
        let t = time * self.speed * 0.15;

        for i in 0..NUM_ZONES {
            let pos = i as f32 / NUM_ZONES as f32;

            // Always-on heat base (fills the keyboard)
            let base = 0.6;

            // Very slow spatial drift (large-scale motion)
            let drift = ((pos * 2.0 + t).fract() - 0.5).abs();
            let drift = 1.0 - drift * 2.0;

            // Gentle variation, NOT flicker
            let variation = drift * 0.25;

            let mut intensity = (base + variation).clamp(0.0, 1.0);

            // Soft curve to avoid harsh edges
            intensity = intensity.powf(1.3);

            // Heat color: deep orange → warm yellow
            let hue = 18.0 + intensity * 22.0;
            let saturation = 1.0;
            let value = intensity;

            controller.set_zone(
                i,
                Color::from_hsv(hue, saturation, value),
            );
        }

        let _ = controller.flush_buffered();
    }

    fn name(&self) -> &str {
        "Heat Bed"
    }
}
