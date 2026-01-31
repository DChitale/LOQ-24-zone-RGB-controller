use crate::led_driver::{LedController, Color, NUM_ZONES};
use crate::effects::Effect;

pub struct SmoothHorseCycleEffect {
    position: f32,
    speed: f32,
    length: f32,
    base_hue: f32,
    horse_hue: f32,
}

impl SmoothHorseCycleEffect {
    pub fn new(speed: f32, length: f32) -> Self {
        Self {
            position: 0.0,
            speed,
            length,
            base_hue: 0.0,
            horse_hue: 180.0,
        }
    }
}

impl Effect for SmoothHorseCycleEffect {
    fn update(&mut self, controller: &mut LedController, _time: f32, delta: f32) {
        // Smooth position advance
        self.position = (self.position + delta * self.speed) % NUM_ZONES as f32;

        // Smooth hue cycling
        self.base_hue = (self.base_hue + delta * 8.0) % 360.0;
        self.horse_hue = (self.horse_hue + delta * 50.0) % 360.0;

        let base_color = Color::from_hsv(self.base_hue, 0.25, 0.50);
        let horse_color = Color::from_hsv(self.horse_hue, 1.0, 1.0);

        for i in 0..NUM_ZONES {
            let zone_pos = i as f32;

            // Circular distance
            let dist = ((zone_pos - self.position + NUM_ZONES as f32)
                % NUM_ZONES as f32)
                .min(
                    (self.position - zone_pos + NUM_ZONES as f32)
                        % NUM_ZONES as f32,
                );

            // Soft falloff
            let intensity = (1.0 - dist / self.length).clamp(0.0, 1.0);

            let color = base_color.lerp(&horse_color, intensity);
            controller.set_zone(i, color);
        }

        let _ = controller.flush_buffered();
    }

    fn name(&self) -> &str {
        "Smooth Horse Cycle"
    }
}
