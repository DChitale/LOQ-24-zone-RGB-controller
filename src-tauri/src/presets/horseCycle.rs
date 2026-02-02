use crate::led_driver::{LedController, Color, NUM_ZONES};
use crate::effects::Effect;

pub struct SmoothHorseCycleEffect {
    position: f32,
    speed: f32,
    length: f32,
    direction: f32,
    horse_hue: f32,
}

impl SmoothHorseCycleEffect {
    pub fn new(speed: f32, length: f32) -> Self {
        Self {
            position: 0.0,
            speed,
            length,
            direction: 1.0,
            horse_hue: 30.0, // start warm, arbitrary
        }
    }
}

impl Effect for SmoothHorseCycleEffect {
    fn update(&mut self, controller: &mut LedController, _time: f32, delta: f32) {
        // Move
        self.position += self.direction * delta * self.speed;

        // Bounce at edges
        let max_pos = NUM_ZONES as f32 - 1.0;
        let mut bounced = false;

        if self.position <= 0.0 {
            self.position = 0.0;
            self.direction = 1.0;
            bounced = true;
        } else if self.position >= max_pos {
            self.position = max_pos;
            self.direction = -1.0;
            bounced = true;
        }

        // Change color ONLY on bounce
        if bounced {
            self.horse_hue = (self.horse_hue + 60.0) % 360.0;
        }

        let base_hue = (self.horse_hue + 180.0) % 360.0;

        let base_color = Color::from_hsv(base_hue, 0.25, 0.45);
        let horse_color = Color::from_hsv(self.horse_hue, 1.0, 1.0);

        for i in 0..NUM_ZONES {
            let dist = (i as f32 - self.position).abs();
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
