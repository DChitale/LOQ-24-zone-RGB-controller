use crate::led_driver::{LedController, Color, NUM_ZONES};
use crate::effects::Effect;

pub struct OffEffect;

impl OffEffect {
    pub fn new() -> Self {
        Self {}
    }
}

impl Effect for OffEffect {
    fn update(&mut self, controller: &mut LedController, _time: f32, delta: f32) {
        let _ = controller.set_all_instant(Color::new(0,0,0));
    }

    fn name(&self) -> &str {
        "Off"
    }
}