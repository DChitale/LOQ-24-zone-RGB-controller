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

        // === Throttle rhythm ===
        let mut throttle = (time * 3.5).sin();
        throttle = (throttle + 1.0) * 0.5;
        throttle = throttle.powf(1.5);

        // === RPM physics ===
        self.rpm += throttle * 0.08;
        self.rpm -= 0.03;
        self.rpm = self.rpm.clamp(0.2, self.redline + 0.15);

        // === Redline limiter ===
        if self.rpm >= self.redline {
            self.rpm -= 0.25;
            self.shift_flash = 3;
            self.backfire = rng.gen_range(2..=4);
        }

        // === Speed & position ===
        self.speed = self.rpm * 1.5;
        self.pos += self.speed * self.direction * delta * 60.0;

        // === Edge bounce ===
        if self.pos > NUM_ZONES as f32 + 6.0 {
            self.direction = -1.0;
            self.backfire = 3;
        } else if self.pos < -6.0 {
            self.direction = 1.0;
            self.backfire = 3;
        }

        // === Clear buffer ===
        controller.fill(Color::black());

        // === Heat trail ===
        for z in 0..NUM_ZONES {
            let dist = (z as f32 - self.pos).abs();

            if dist < 5.0 {
                let mut heat = 1.0 - dist / 5.0;
                heat = heat.max(0.0).powf(2.0);

                let hue = 5.0 + heat * 40.0;
                let val = (0.25 + heat * (0.8 + self.rpm * 0.6)).min(1.0);

                let color = Color::from_hsv(hue, 1.0, val);
                controller.set_zone(z, color);
            }
        }

        // === Gear shift flash ===
        if self.shift_flash > 0 {
            let center = self.pos.round() as i32;
            for dz in -2..=2 {
                let zi = center + dz;
                if zi >= 0 && zi < NUM_ZONES as i32 {
                    controller.set_zone(zi as usize, Color::white());
                }
            }
            self.shift_flash -= 1;
        }

        // === Backfire pops ===
        if self.backfire > 0 {
            let tail = (self.pos - 3.0 * self.direction).round() as i32;
            if tail >= 0 && tail < NUM_ZONES as i32 {
                let color = match rng.gen_range(0..3) {
                    0 => Color::new(255, 120, 0),
                    1 => Color::new(255, 200, 50),
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
