use crate::led_driver::{LedController, Color, NUM_ZONES};
use crate::effects::Effect;
use crate::audio_sampler::AudioSampler;

pub struct AudioSparkleRainbowEffect {
    sampler: AudioSampler,
    sensitivity: f32,
    base_density: f32,
    rainbow_speed: f32,
}

impl AudioSparkleRainbowEffect {
    pub fn new(sampler: AudioSampler, sensitivity: f32, base_density: f32, rainbow_speed: f32) -> Self {
        AudioSparkleRainbowEffect { 
            sampler,
            sensitivity,
            base_density,
            rainbow_speed,
        }
    }
}

impl Effect for AudioSparkleRainbowEffect {
    fn update(&mut self, controller: &mut LedController, time: f32, _delta: f32) {
        controller.fill(Color::black());

        let intensity = self.sampler.get_intensity();
        let density = (self.base_density + intensity * self.sensitivity).clamp(0.0, 1.0);
        let brightness = (intensity * self.sensitivity * 5.0).clamp(0.0, 1.0);

        for i in 0..NUM_ZONES {
            let hash = ((i as f32 * 12.9898 + time * 78.233).sin() * 43758.5453).fract();
            
            if hash > 1.0 - density {
                let hue = ((i as f32 / NUM_ZONES as f32) * 360.0 + time * self.rainbow_speed * 180.0) % 360.0;
                let color = Color::from_hsv(hue, 1.0, 1.0).scale(brightness);
                controller.set_zone(i, color);
            }
        }

        let _ = controller.flush_buffered();
    }

    fn name(&self) -> &str {
        "Audio Sparkle Rainbow"
    }
}
