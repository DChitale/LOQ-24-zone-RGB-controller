use crate::led_driver::{LedController, Color, NUM_ZONES};
use crate::effects::Effect;

pub struct KeyReactiveRippleEffect {
    intensities: [f32; NUM_ZONES],
    decay_speed: f32,
}

impl KeyReactiveRippleEffect {
    pub fn new(decay_speed: f32) -> Self {
        KeyReactiveRippleEffect {
            intensities: [0.0; NUM_ZONES],
            decay_speed,
        }
    }
}

impl Effect for KeyReactiveRippleEffect {
    fn start(&mut self) {
        self.intensities = [0.0; NUM_ZONES];
    }

    fn update(&mut self, controller: &mut LedController, _time: f32, delta: f32) {
        let key_states = controller.get_key_states();

        for i in 0..NUM_ZONES {
            // If key is pressed, instantly light it up
            if key_states[i] {
                self.intensities[i] = 1.0;
            } else {
                // Otherwise decay smoothly
                self.intensities[i] =
                    (self.intensities[i] - delta * self.decay_speed).max(0.0);
            }

            let color = Color::white().scale(self.intensities[i]);
            controller.set_zone(i, color);
        }

        let _ = controller.flush_buffered();
    }

    fn stop(&mut self, controller: &mut LedController) {
        self.intensities = [0.0; NUM_ZONES];
        let _ = controller.clear();
    }

    fn name(&self) -> &str {
        "Key Reactive Ripple"
    }
}
