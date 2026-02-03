use crate::led_driver::{LedController, Color, NUM_ZONES};
use crate::effects::Effect;
use rand::Rng;

pub struct FerrariRpmEffect {
    pos: f32,
    speed: f32,
    direction: f32,

    rpm: f32,
    redline: f32,

    shift_flash: i32,
    backfire: i32,
}

impl FerrariRpmEffect {
    pub fn new() -> Self {
        Self {
            pos: -6.0,
            speed: 0.0,
            direction: 1.0,

            rpm: 0.2,
            redline: 1.0,

            shift_flash: 0,
            backfire: 0,
        }
    }
}

impl Effect for FerrariRpmEffect {
    fn start(&mut self) {
        self.pos = -6.0;
        self.speed = 0.0;
        self.direction = 1.0;
        self.rpm = 0.2;
        self.shift_flash = 0;
        self.backfire = 0;
    }

    fn update(&mut self, controller: &mut LedController, time: f32, delta: f32) {
        let mut rng = rand::thread_rng();

        // === Soft throttle rhythm ===
        let mut throttle = (time * 2.0).sin(); // slower oscillation
        throttle = (throttle + 1.0) * 0.5;
        throttle = throttle.powf(1.2); // softer curve

        // === RPM smoothing ===
        let target_rpm = 0.2 + throttle * 0.5;
        self.rpm += (target_rpm - self.rpm) * 0.05; // smoother interpolation
        self.rpm = self.rpm.clamp(0.2, self.redline + 0.1);

        // === Redline limiter, less harsh ===
        if self.rpm >= self.redline {
            self.rpm -= 0.1; // softer RPM drop
            self.shift_flash = 2; // shorter flash
            self.backfire = rng.gen_range(1..=3); // smaller backfire
        }

        // === Speed & position ===
        self.speed = self.rpm * 1.2; // slower motion
        self.pos += self.speed * self.direction * delta * 50.0;

        // === Edge bounce, softer ===
        if self.pos > NUM_ZONES as f32 + 6.0 {
            self.direction = -1.0;
            self.backfire = 2;
        } else if self.pos < -6.0 {
            self.direction = 1.0;
            self.backfire = 2;
        }

        // === Clear buffer ===
        controller.fill(Color::black());

        // === Heat trail ===
        for z in 0..NUM_ZONES {
            let dist = (z as f32 - self.pos).abs();
            if dist < 6.0 {
                let mut heat = 1.0 - dist / 6.0;
                heat = heat.max(0.0).powf(1.5); // softer falloff

                let hue = 5.0 + heat * 35.0;
                let val = (0.2 + heat * (0.7 + self.rpm * 0.4)).min(1.0);

                let color = Color::from_hsv(hue, 1.0, val);
                controller.set_zone(z, color);
            }
        }

        // === Gear shift flash, softer ===
        if self.shift_flash > 0 {
            let center = self.pos.round() as i32;
            for dz in -1..=1 {
                let zi = center + dz;
                if zi >= 0 && zi < NUM_ZONES as i32 {
                    controller.set_zone(zi as usize, Color::white());
                }
            }
            self.shift_flash -= 1;
        }

        // === Backfire pops, softer ===
        if self.backfire > 0 {
            let tail = (self.pos - 2.0 * self.direction).round() as i32;
            if tail >= 0 && tail < NUM_ZONES as i32 {
                let color = match rng.gen_range(0..3) {
                    0 => Color::new(255, 140, 50),
                    1 => Color::new(255, 180, 80),
                    _ => Color::white(),
                };
                controller.set_zone(tail as usize, color);
            }
            self.backfire -= 1;
        }

        // === Send via 0x04 (buffered per-zone) ===
        let _ = controller.flush_buffered();
    }

    fn name(&self) -> &str {
        "FerrariRpmEffect"
    }
}
