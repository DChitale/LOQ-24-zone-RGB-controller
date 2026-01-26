use crate::led_driver::{LedController, Color, NUM_ZONES};
use super::{LedEffect, RenderMode};

pub struct RainbowEffect { pub speed: f32 }

impl LedEffect for RainbowEffect {
    fn render_mode(&self) -> RenderMode { RenderMode::Animated }
    fn update(&mut self, controller: &mut LedController, tick: f32) {
        for i in 0..NUM_ZONES {
            let hue = (tick * self.speed + (i as f32 * 15.0)) % 360.0;
            controller.set_zone(i, Color::from_hsv(hue, 1.0, 1.0));
        }
    }
}