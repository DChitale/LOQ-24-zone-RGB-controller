use crate::led_driver::{LedController, Color, NUM_ZONES};
use crate::effects::Effect;

enum Phase {
    Compress,
    Lock,
    Implode,
    Explode,
}

pub struct ColorScanEffect   {
    phase: Phase,
    progress: f32,
    speed: f32,
}

impl ColorScanEffect   {
    pub fn new(speed: f32) -> Self {
        Self {
            phase: Phase::Compress,
            progress: 0.0,
            speed,
        }
    }
}
impl Effect for ColorScanEffect   {
    // ... (new, start, and struct fields remain the same)

    fn update(&mut self, controller: &mut LedController, _time: f32, delta: f32) {
        let center = NUM_ZONES as f32 / 2.0;
        
        // Advance progress
        self.progress += delta * self.speed;

        // --- Easing Logic ---
        // Ease In (Accelerating): t * t * t
        // Ease Out (Decelerating): 1 - (1 - t)^3
        let eased_progress = match self.phase {
            Phase::Compress => self.progress.powi(3),             // Smashes into center
            Phase::Implode  => 1.0 - (1.0 - self.progress).powi(3), // Snaps shut
            Phase::Explode  => 1.0 - (1.0 - self.progress).powi(4), // Fast initial burst
            _ => self.progress,
        };

        // --- Phase Management ---
        match self.phase {
            Phase::Compress if self.progress >= 1.0 => {
                self.phase = Phase::Lock;
                self.progress = 0.0;
            }
            Phase::Lock if self.progress >= 0.2 => { // Shorter lock for better flow
                self.phase = Phase::Implode;
                self.progress = 0.0;
            }
            Phase::Implode if self.progress >= 1.0 => {
                self.phase = Phase::Explode;
                self.progress = 0.0;
            }
            Phase::Explode if self.progress >= 1.0 => {
                self.phase = Phase::Compress;
                self.progress = 0.0;
            }
            _ => {}
        }

        // --- Rendering with Spatial Smoothing ---
        for z in 0..NUM_ZONES {
            let dist = (z as f32 - center).abs();
            
            let color = match self.phase {
                Phase::Compress => {
                    let threshold = center * (1.0 - eased_progress);
                    // Use a smooth edge instead of a hard cutoff
                    let intensity = ((dist - threshold).max(0.0) / 2.0).min(1.0);
                    Color::new(
                        (intensity * 100.0) as u8, 
                        (intensity * 180.0) as u8, 
                        (intensity * 255.0) as u8
                    )
                }

                Phase::Lock => {
                    let pulse = (self.progress * 10.0).sin().abs();
                    if dist <= 1.5 { 
                        Color::new(255, 255, 255) 
                    } else {
                        Color::new((pulse * 50.0) as u8, 0, 0) // Low red throb
                    }
                }

                Phase::Implode => {
                    let radius = (1.0 - eased_progress) * center;
                    if dist <= radius {
                        let fade = 1.0 - eased_progress;
                        Color::new((fade * 255.0) as u8, (fade * 255.0) as u8, (fade * 255.0) as u8)
                    } else {
                        Color::black()
                    }
                }

                Phase::Explode => {
                    let radius = eased_progress * center;
                    let thickness = 3.0;
                    // Create a soft gradient for the shockwave
                    let edge = 1.0 - ((dist - radius).abs() / thickness).min(1.0);
                    let power = (1.0 - eased_progress); // Fade out as it expands
                    
                    Color::new(
                        (edge * power * 255.0) as u8,
                        (edge * power * 100.0) as u8,
                        (edge * power * 50.0) as u8
                    )
                }
            };

            controller.set_zone(z, color);
        }

        let _ = controller.flush_buffered();
    }
}