use crate::led_driver::{LedController, Color, NUM_ZONES};
use crate::effects::Effect;

pub struct HorseEffect {
    position: f32,
    speed: f32,
    length: f32,
    base_color: Color,
    horse_color: Color,
}

impl HorseEffect {
    pub fn new(
        speed: f32,
        length: f32,
        base_color: Color,
        horse_color: Color,
    ) -> Self {
        Self {
            position: 0.0,
            speed,
            length,
            base_color,
            horse_color,
        }
    }
}

impl Effect for HorseEffect {
    fn update(&mut self, controller: &mut LedController, _time: f32, delta: f32) {
        // advance smoothly
        self.position = (self.position + delta * self.speed) % NUM_ZONES as f32;

        for i in 0..NUM_ZONES {
            // distance from moving horse
            let dist = ((i as f32 - self.position + NUM_ZONES as f32)
                % NUM_ZONES as f32)
                .min(
                    (self.position - i as f32 + NUM_ZONES as f32)
                        % NUM_ZONES as f32,
                );

            // soft falloff
            let intensity = (1.0 - dist / self.length).clamp(0.0, 1.0);

            let color = self.base_color.lerp(&self.horse_color, intensity);
            controller.set_zone(i, color);
        }

        let _ = controller.flush_buffered();
    }

    fn name(&self) -> &str {
        "Horse"
    }
}
