pub mod static_color;
pub mod rainbow;

use crate::led_driver::LedController;

pub enum RenderMode {
    Static,   // Uses 0x05 (Single packet)
    Animated, // Uses 0x04 (Three packets)
}

pub trait LedEffect: Send {
    fn render_mode(&self) -> RenderMode;
    fn update(&mut self, controller: &mut LedController, tick: f32);
}