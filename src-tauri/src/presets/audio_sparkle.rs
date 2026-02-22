use crate::led_driver::{LedController, Color, NUM_ZONES};
use crate::effects::Effect;
use crate::audio_sampler::AudioSampler;

pub struct AudioSparkleEffect {
    sampler: AudioSampler,
    sensitivity: f32,
    base_density: f32,
}

impl AudioSparkleEffect {
    pub fn new(sampler: AudioSampler, sensitivity: f32, base_density: f32) -> Self {
        AudioSparkleEffect { 
            sampler,
            sensitivity,
            base_density,
        }
    }
}

impl Effect for AudioSparkleEffect {
    fn update(&mut self, controller: &mut LedController, time: f32, _delta: f32) {
        controller.fill(Color::black());

        let intensity = self.sampler.get_intensity();
        // Scale density with audio intensity
        let density = (self.base_density + intensity * self.sensitivity).clamp(0.0, 1.0);
        
        // Brightness also reacts to intensity
        let brightness = (intensity * self.sensitivity * 5.0).clamp(0.0, 1.0);

        for i in 0..NUM_ZONES {
            // Using a simple hash for "random" sparkles that move slightly over time
            let hash = ((i as f32 * 12.9898 + time * 78.233).sin() * 43758.5453).fract();
            
            if hash > 1.0 - density {
                let color = Color::white().scale(brightness);
                controller.set_zone(i, color);
            }
        }

        let _ = controller.flush_buffered();
    }

    fn name(&self) -> &str {
        "Audio Sparkle"
    }
}
