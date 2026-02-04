use crate::led_driver::{LedController, Color, NUM_ZONES};
use crate::effects::Effect;
use rand::Rng;

pub struct EventHorizon {
    particles: Vec<f32>, // Positions
}

impl EventHorizon {
    pub fn new() -> Self {
        Self { particles: Vec::new() }
    }
}

impl Effect for EventHorizon {
    fn start(&mut self) { self.particles.clear(); }

    fn update(&mut self, controller: &mut LedController, time: f32, delta: f32) {
        controller.fill(Color::black());
        let center = NUM_ZONES as f32 / 2.0;
        let mut rng = rand::thread_rng();

        // Spawn new particles at edges
        if rng.gen_bool(0.1) {
            self.particles.push(if rng.gen_bool(0.5) { 0.0 } else { NUM_ZONES as f32 - 1.0 });
        }

        // Move and draw particles
        self.particles.retain_mut(|pos| {
            let dist = (*pos - center).abs();
            // Gravity formula: closer = faster
            let speed = (1.0 / (dist + 0.5)) * 150.0;
            
            if *pos < center { *pos += speed * delta; }
            else { *pos -= speed * delta; }

            let idx = *pos as usize;
            if idx < NUM_ZONES {
                // Color shift: Red at edges -> Blue/White at center
                let factor = 1.0 - (dist / center);
                let color = Color::new(
                    (255.0 * (1.0 - factor)) as u8,
                    (50.0 * factor) as u8,
                    (255.0 * factor) as u8
                );
                controller.set_zone(idx, color);
            }

            // Remove if they hit the "singularity"
            dist > 0.5
        });

        // The Singularity pulse
        let pulse = (time * 3.0).sin().abs() * 100.0;
        controller.set_zone(center as usize, Color::new(pulse as u8, 0, (pulse * 2.0).min(255.0) as u8));
    }

    fn name(&self) -> &str { "Event Horizon" }
}