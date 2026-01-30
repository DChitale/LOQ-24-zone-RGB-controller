// use crate::led_driver::{LedController, Color, NUM_ZONES};
// use crate::effects::Effect;

// use sysinfo::Components;
// use std::time::{Instant, Duration};

// pub struct ThermalStatusEffect {
//     // temperatures
//     cpu_temp: f32,
//     gpu_temp: f32,
//     aux_temp: f32,

//     // sysinfo
//     components: Components,
//     last_poll: Instant,
// }

// impl ThermalStatusEffect {
//     pub fn new() -> Self {
//         let components = Components::new_with_refreshed_list();

//         Self {
//             cpu_temp: 0.0,
//             gpu_temp: 0.0,
//             aux_temp: 0.0,
//             components,
//             last_poll: Instant::now(),
//         }
//     }

//     fn poll_temperatures(&mut self) {
//         // limit polling rate (sysinfo is not free)
//         if self.last_poll.elapsed() < Duration::from_millis(500) {
//             return;
//         }

//         self.components.refresh();

//         let mut cpu = None;
//         let mut gpu = None;
//         let mut aux = None;

//         for component in &self.components {
//             let name = component.label().to_lowercase();
//             let temp = component.temperature();

//             if cpu.is_none() && name.contains("cpu") {
//                 cpu = Some(temp);
//             } else if gpu.is_none() && name.contains("gpu") {
//                 gpu = Some(temp);
//             } else if aux.is_none() {
//                 aux = Some(temp);
//             }
//         }

//         self.cpu_temp = cpu.unwrap_or(0.0);
//         self.gpu_temp = gpu.unwrap_or(0.0);
//         self.aux_temp = aux.unwrap_or(0.0);

//         self.last_poll = Instant::now();
//     }

//     fn temp_to_color(temp: f32) -> Color {
//         let t = ((temp - 30.0) / 60.0).clamp(0.0, 1.0);

//         let hue = if t < 0.5 {
//             220.0 + (120.0 - 220.0) * (t * 2.0)
//         } else {
//             120.0 + (0.0 - 120.0) * ((t - 0.5) * 2.0)
//         };

//         Color::from_hsv(hue, 0.85, 0.6)
//     }
// }

// impl Effect for ThermalStatusEffect {
//     fn update(&mut self, controller: &mut LedController, _time: f32, _delta: f32) {
//         // pull temps internally
//         self.poll_temperatures();

//         let third = NUM_ZONES / 3;

//         let cpu_color = Self::temp_to_color(self.cpu_temp);
//         let gpu_color = Self::temp_to_color(self.gpu_temp);
//         let aux_color = Self::temp_to_color(self.aux_temp);

//         // LEFT — CPU
//         for i in 0..third {
//             controller.set_zone(i, cpu_color);
//         }

//         // MIDDLE — AUX glow
//         for i in third..(2 * third) {
//             let dist =
//                 ((i - third) as f32 / third as f32 - 0.5).abs() * 2.0;
//             let brightness = (1.0 - dist).clamp(0.2, 1.0);

//             controller.set_zone(i, aux_color.scale(brightness));
//         }

//         // RIGHT — GPU
//         for i in (2 * third)..NUM_ZONES {
//             controller.set_zone(i, gpu_color);
//         }

//         let _ = controller.flush_buffered();
//     }

//     fn name(&self) -> &str {
//         "Thermal Status"
//     }
// }