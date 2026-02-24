use crate::led_driver::{LedController, Color, NUM_ZONES};
use crate::effects::Effect;
use crate::input_handler::KEY_EVENTS;

struct Ripple {
    center: f32,
    start_time: f32,
    hue_offset: f32,
}

pub struct RainbowRippleEffect {
    ripples: Vec<Ripple>,
    speed: f32,
    width: f32,
    lifetime: f32,
}

impl RainbowRippleEffect {
    pub fn new(speed: f32, width: f32, lifetime: f32) -> Self {
        RainbowRippleEffect {
            ripples: Vec::new(),
            speed,
            width,
            lifetime,
        }
    }
}

impl Effect for RainbowRippleEffect {
    fn start(&mut self) {
        self.ripples.clear();
    }

    fn update(&mut self, controller: &mut LedController, time: f32, _delta: f32) {
        // 1. Consume new key events
        if let Ok(mut events) = KEY_EVENTS.lock() {
            for zone in events.drain(..) {
                self.ripples.push(Ripple {
                    center: zone as f32,
                    start_time: time,
                    hue_offset: (time * 60.0) % 360.0,
                });
            }
        }

        // 2. Clean up old ripples
        self.ripples.retain(|r| time - r.start_time < self.lifetime);

        // 3. Render
        controller.fill(Color::black());

        for i in 0..NUM_ZONES {
            let mut final_color = Color::black();
            let mut total_intensity = 0.0f32;

            for ripple in &self.ripples {
                let age = time - ripple.start_time;
                let dist = (i as f32 - ripple.center).abs();
                let wave_pos = age * self.speed;
                
                // Water Drop Physics: Oscillating sine wave with Gaussian envelope
                // The frequency creates the "rings"
                let frequency = 1.2; 
                let diff = dist - wave_pos;
                
                // Core wave function: cos gives a peak at diff=0 (the main wavefront)
                let wave = (diff * frequency).cos();
                
                // Envelope limits the wave to a specific width around the wavefront
                let envelope = (-(diff * diff) / (self.width * self.width)).exp();
                
                // Global decay over the lifetime of the ripple
                let decay = (1.0 - age / self.lifetime).powi(2);
                
                let intensity = (wave * 0.5 + 0.5) * envelope * decay;
                
                if intensity > 0.01 {
                    let hue = (ripple.hue_offset + dist * 10.0 + age * 20.0) % 360.0;
                    let ripple_color = Color::from_hsv(hue, 0.9, 1.0).scale(intensity);
                    
                    // Additive blending
                    final_color.r = final_color.r.saturating_add(ripple_color.r);
                    final_color.g = final_color.g.saturating_add(ripple_color.g);
                    final_color.b = final_color.b.saturating_add(ripple_color.b);
                    total_intensity += intensity;
                }
            }

            if total_intensity > 0.0 {
                controller.set_zone(i, final_color);
            }
        }

        let _ = controller.flush_buffered();
    }

    fn stop(&mut self, controller: &mut LedController) {
        self.ripples.clear();
        let _ = controller.clear();
    }

    fn name(&self) -> &str {
        "Typing Rainbow Ripple"
    }
}
