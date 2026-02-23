use crate::led_driver::{LedController, Color, NUM_ZONES};
use crate::effects::Effect;
use crate::audio_sampler::AudioSampler;
use crate::presets::ambient::{ScreenSampler, RgbF};

const AMBIENT_WIDTH: usize = NUM_ZONES;
const AMBIENT_HEIGHT: usize = 12;

pub struct AudioSparkleMediaEffect<S: ScreenSampler> {
    sampler_audio: AudioSampler,
    sampler_media: S,
    sensitivity: f32,
    base_density: f32,
}

impl<S: ScreenSampler> AudioSparkleMediaEffect<S> {
    pub fn new(sampler_audio: AudioSampler, sampler_media: S, sensitivity: f32, base_density: f32) -> Self {
        AudioSparkleMediaEffect { 
            sampler_audio,
            sampler_media,
            sensitivity,
            base_density,
        }
    }
}

impl<S: ScreenSampler> Effect for AudioSparkleMediaEffect<S> {
    fn update(&mut self, controller: &mut LedController, time: f32, _delta: f32) {
        controller.fill(Color::black());

        let intensity = self.sampler_audio.get_intensity();
        let density = (self.base_density + intensity * self.sensitivity).clamp(0.0, 1.0);
        let brightness = (intensity * self.sensitivity * 5.0).clamp(0.0, 1.0);

        let mut buffer = [[RgbF::black(); AMBIENT_HEIGHT]; AMBIENT_WIDTH];
        self.sampler_media.sample(&mut buffer);

        for i in 0..NUM_ZONES {
            let hash = ((i as f32 * 12.9898 + time * 78.233).sin() * 43758.5453).fract();
            
            if hash > 1.0 - density {
                // Find the pixel with maximum luminance in the column for better accuracy
                let mut best_pixel = RgbF::black();
                let mut max_luma = -1.0;
                
                for y in 0..AMBIENT_HEIGHT {
                    let pixel = buffer[i][y];
                    let luma = 0.2126 * pixel.r + 0.7152 * pixel.g + 0.0722 * pixel.b;
                    if luma > max_luma {
                        max_luma = luma;
                        best_pixel = pixel;
                    }
                }

                // Apply slight saturation boost and gamma to make sparkles more vibrant
                // Saturation boost (1.5x)
                let l = max_luma;
                let gray = RgbF { r: l, g: l, b: l };
                let saturated = RgbF {
                    r: (gray.r + (best_pixel.r - gray.r) * 1.5).clamp(0.0, 1.0),
                    g: (gray.g + (best_pixel.g - gray.g) * 1.5).clamp(0.0, 1.0),
                    b: (gray.b + (best_pixel.b - gray.b) * 1.5).clamp(0.0, 1.0),
                };
                
                // Gamma correction (0.8) for richer colors in mid-brightness
                let corrected = RgbF {
                    r: saturated.r.powf(0.8),
                    g: saturated.g.powf(0.8),
                    b: saturated.b.powf(0.8),
                };

                let color = corrected.to_color().scale(brightness);
                controller.set_zone(i, color);
            }
        }

        let _ = controller.flush_buffered();
    }

    fn name(&self) -> &str {
        "Audio Sparkle Media"
    }
}
