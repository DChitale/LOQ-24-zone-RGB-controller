use crate::led_driver::{LedController, Color, NUM_ZONES};
use crate::effects::Effect;

pub struct StaticEffect {
    color: Color,
}

impl StaticEffect{
    pub fn new(color: Color) -> Self {
        StaticEffect { color }
    }
}

impl Effect for StaticEffect {
    //const is_static:bool = true;
    fn update(&mut self, controller: &mut LedController, _time: f32, _delta: f32) {
        // for i in 0..NUM_ZONES {
        //     controller.set_zone(i, self.color);
        // }
        let _ = controller.set_all_instant(self.color);

        //let _ = controller.flush_buffered();
    }

    fn name(&self) -> &str {
        "Static"
    }

}