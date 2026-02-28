use crate::led_driver::{LedController, Color, NUM_ZONES};
use crate::effects::Effect;
use crate::audio_sampler::AudioSampler;

struct Ripple {
    center: f32,
    start_time: f32,
    hue_offset: f32,
}

pub struct AudioRippleEffect {
    ripples: Vec<Ripple>,
    speed: f32,
    width: f32,
    lifetime: f32,
    sampler: AudioSampler,
    last_intensity: f32,
    last_ripple_time: f32,
    sensitivity: f32,
}

impl AudioRippleEffect {
    pub fn new(sampler: AudioSampler, sensitivity: f32, speed: f32, width: f32, lifetime: f32) -> Self {
        AudioRippleEffect {
            ripples: Vec::new(),
            speed,
            width,
            lifetime,
            sampler,
            last_intensity: 0.0,
            last_ripple_time: -10.0,
            sensitivity,
        }
    }
}

impl Effect for AudioRippleEffect {
    fn start(&mut self) {
        self.ripples.clear();
        self.last_intensity = self.sampler.get_intensity();
    }

    fn update(&mut self, controller: &mut LedController, time: f32, _delta: f32) {
        let raw_intensity = self.sampler.get_intensity();
        let intensity = raw_intensity * self.sensitivity;
        
        // Basic beat detection: sudden spike in intensity
        // Trigger a ripple if the sound gets loud suddenly and hasn't rippled recently (to avoid spam)
        if intensity > 0.1 && (intensity - self.last_intensity) > 0.05 && (time - self.last_ripple_time) > 0.3 {
            self.ripples.push(Ripple {
                center: (NUM_ZONES as f32) / 2.0,
                start_time: time,
                hue_offset: (time * 60.0) % 360.0,
            });
            self.last_ripple_time = time;
        }
        self.last_intensity = intensity;

        // Clean up old ripples
        self.ripples.retain(|r| time - r.start_time < self.lifetime);

        controller.fill(Color::black());

        for i in 0..NUM_ZONES {
            let mut final_color = Color::black();
            let mut total_intensity = 0.0f32;

            for ripple in &self.ripples {
                let age = time - ripple.start_time;
                let dist = (i as f32 - ripple.center).abs();
                let wave_pos = age * self.speed;
                
                let frequency = 1.2; 
                let diff = dist - wave_pos;
                
                let wave = (diff * frequency).cos();
                let envelope = (-(diff * diff) / (self.width * self.width)).exp();
                let decay = (1.0 - age / self.lifetime).powi(2);
                
                let ripple_intensity = (wave * 0.5 + 0.5) * envelope * decay;
                
                if ripple_intensity > 0.01 {
                    let hue = (ripple.hue_offset + dist * 10.0 + age * 20.0) % 360.0;
                    let ripple_color = Color::from_hsv(hue, 1.0, 1.0).scale(ripple_intensity);
                    
                    final_color.r = final_color.r.saturating_add(ripple_color.r);
                    final_color.g = final_color.g.saturating_add(ripple_color.g);
                    final_color.b = final_color.b.saturating_add(ripple_color.b);
                    total_intensity += ripple_intensity;
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
        "Audio Ripple"
    }
}
