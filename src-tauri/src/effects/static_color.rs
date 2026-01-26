use crate::led_driver::{LedController, Color};
use super::{LedEffect, RenderMode};

pub struct StaticColor { pub color: Color }

impl LedEffect for StaticColor {
    fn render_mode(&self) -> RenderMode { RenderMode::Static }
    fn update(&mut self, controller: &mut LedController, _tick: f32) {
        // Just set the first zone; the engine reads this for the 0x05 command
        controller.set_zone(0, self.color);
    }
}