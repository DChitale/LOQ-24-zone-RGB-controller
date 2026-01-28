use crate::led_driver::{LedController, Color, NUM_ZONES};
use crate::effects::{Effect, smoothstep};

pub struct AuroraEffect {
    speed: f32,
}

impl AuroraEffect {
    pub fn new(speed: f32) -> Self {
        AuroraEffect { speed }
    }
}

impl Effect for AuroraEffect {
    fn update(&mut self, controller: &mut LedController, time: f32, _delta: f32) {
        for i in 0..NUM_ZONES {
            let pos = i as f32 / NUM_ZONES as f32;
            let flow = (pos + time * self.speed).fract();

            let brightness = smoothstep(0.0, 0.5, (flow - 0.5).abs());
            let hue = 160.0 + flow * 80.0;

            controller.set_zone(
                i,
                Color::from_hsv(hue, 0.8, brightness),
            );
        }

        let _ = controller.flush_buffered();
    }

    fn name(&self) -> &str {
        "Aurora"
    }
}
