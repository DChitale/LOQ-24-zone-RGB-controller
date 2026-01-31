use crate::led_driver::{LedController, Color, NUM_ZONES};
use crate::effects::Effect;

pub struct FerrariRpmEffect {
    phase: f32,
    speed: f32,
}

impl FerrariRpmEffect {
    pub fn new(speed: f32) -> Self {
        Self {
            phase: 0.0,
            speed,
        }
    }
}

impl Effect for FerrariRpmEffect {
    fn start(&mut self) {
        self.phase = 0.0;
    }

    fn update(&mut self, controller: &mut LedController, _time: f32, delta: f32) {
        // Advance RPM phase
        self.phase += delta * self.speed;

        // Simulate rev limiter snap
        if self.phase > 1.0 {
            self.phase = 0.0;
        }

        let active_zone = (self.phase * NUM_ZONES as f32) as usize;

        // Base Ferrari red (dark)
        let base_red = Color::new(90, 0, 0);

        for i in 0..NUM_ZONES {
            let color = if i <= active_zone {
                // Brightness ramps toward white near redline
                let intensity = i as f32 / NUM_ZONES as f32;

                if intensity > 0.85 {
                    // White-hot redline
                    Color::new(255, 240, 240)
                } else {
                    // Pure Ferrari red ramp
                    Color::new(
                        (180.0 + intensity * 75.0) as u8,
                        0,
                        0,
                    )
                }
            } else {
                base_red
            };

            controller.set_zone(i, color);
        }

        let _ = controller.flush_buffered();
    }

    fn name(&self) -> &str {
        "Ferrari RPM"
    }
}
