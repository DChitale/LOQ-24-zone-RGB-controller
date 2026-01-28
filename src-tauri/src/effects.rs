// effects.rs
// Effects system template for RGB keyboard
// Each effect should implement the Effect trait

use crate::led_driver::{LedController, Color, NUM_ZONES};

/// Effect trait that all effects must implement
pub trait Effect: Send {
    /// Called once when the effect is activated
    fn start(&mut self) {}
    
    /// Called every frame to update the effect
    /// 
    /// # Arguments
    /// * `controller` - LED controller to manipulate
    /// * `time` - Total time since effect started (seconds)
    /// * `delta` - Time since last update (seconds)
    fn update(&mut self, controller: &mut LedController, time: f32, delta: f32);
    
    /// Called once when the effect is stopped
    fn stop(&mut self, controller: &mut LedController) {
        let _ = controller.clear();
    }
    
    /// Get effect name (for debugging/UI)
    fn name(&self) -> &str {
        "Unknown Effect"
    }
}

// ===================================================================
// EXAMPLE EFFECTS
// ===================================================================

// --- Example 1: Solid Color Effect ---

pub struct SolidEffect {
    color: Color,
}

impl SolidEffect {
    pub fn new(color: Color) -> Self {
        SolidEffect { color }
    }
}

impl Effect for SolidEffect {
    fn start(&mut self) {
        // Optional: initialization code
    }
    
    fn update(&mut self, controller: &mut LedController, _time: f32, _delta: f32) {
        // Use command 0x05 for efficiency (all zones same color)
        let _ = controller.set_all_instant(self.color);
    }
    
    fn name(&self) -> &str {
        "Solid Color"
    }
}

// --- Example 2: Rainbow Wave Effect ---

pub struct RainbowWaveEffect {
    speed: f32,
}

impl RainbowWaveEffect {
    pub fn new(speed: f32) -> Self {
        RainbowWaveEffect { speed }
    }
}

impl Effect for RainbowWaveEffect {
    fn update(&mut self, controller: &mut LedController, time: f32, _delta: f32) {
        // Update frame buffer with rainbow colors
        for i in 0..NUM_ZONES {
            let hue = ((i as f32 / NUM_ZONES as f32) + (time * self.speed)) % 1.0;
            let color = Color::from_hsv(hue * 360.0, 1.0, 1.0);
            controller.set_zone(i, color);
        }
        
        // Flush buffer using command 0x04 (each zone has different color)
        let _ = controller.flush_buffered();
    }
    
    fn name(&self) -> &str {
        "Rainbow Wave"
    }
}

// --- Example 3: Breathing Effect ---

pub struct BreathingEffect {
    color: Color,
    speed: f32,
}

impl BreathingEffect {
    pub fn new(color: Color, speed: f32) -> Self {
        BreathingEffect { color, speed }
    }
}

impl Effect for BreathingEffect {
    fn update(&mut self, controller: &mut LedController, time: f32, _delta: f32) {
        // Calculate breathing brightness using sine wave
        let brightness = ((time * self.speed * std::f32::consts::PI * 2.0).sin() + 1.0) / 2.0;
        let scaled_color = self.color.scale(brightness);
        
        // Update the internal frame buffer for all zones
        for i in 0..NUM_ZONES {
            controller.set_zone(i, scaled_color);
        }
        
        // Send to hardware using command 0x05 for efficiency
        let _ = controller.set_all_instant(scaled_color);
    }
    
    fn name(&self) -> &str {
        "Breathing"
    }
}

// ===================================================================
// EFFECT TEMPLATES FOR COMMON PATTERNS
// ===================================================================

// Template 1: PER-ZONE CALCULATION
// Use this when each zone needs a different color based on its position
pub struct PerZoneTemplate {
    // Your parameters here
    speed: f32,
}

impl PerZoneTemplate {
    pub fn new(speed: f32) -> Self {
        PerZoneTemplate { speed }
    }
}

impl Effect for PerZoneTemplate {
    fn update(&mut self, controller: &mut LedController, time: f32, _delta: f32) {
        // Calculate color for each zone
        for i in 0..NUM_ZONES {
            // Your calculation here - example: wave effect
            let phase = (i as f32 / NUM_ZONES as f32) * std::f32::consts::PI * 2.0;
            let brightness = ((time * self.speed + phase).sin() + 1.0) / 2.0;
            let color = Color::white().scale(brightness);
            
            controller.set_zone(i, color);
        }
        
        // Flush using command 0x04
        let _ = controller.flush_buffered();
    }
    
    fn name(&self) -> &str {
        "Per-Zone Template"
    }
}

// Template 2: ZONE RANGE EFFECT
// Use this when you need to set different ranges to different colors
pub struct ZoneRangeTemplate {
    // Your parameters here
}

impl ZoneRangeTemplate {
    pub fn new() -> Self {
        ZoneRangeTemplate {}
    }
}

impl Effect for ZoneRangeTemplate {
    fn update(&mut self, controller: &mut LedController, time: f32, _delta: f32) {
        // Example: split keyboard in half with alternating colors
        let state = (time * 2.0) as i32 % 2;
        
        if state == 0 {
            let _ = controller.set_range(0, 11, Color::red());
            let _ = controller.set_range(12, 23, Color::blue());
        } else {
            let _ = controller.set_range(0, 11, Color::blue());
            let _ = controller.set_range(12, 23, Color::red());
        }
    }
    
    fn name(&self) -> &str {
        "Zone Range Template"
    }
}

// Template 3: ANIMATED SINGLE ELEMENT
// Use this for effects like comets, chasers, etc.
pub struct AnimatedElementTemplate {
    speed: f32,
    tail_length: usize,
}

impl AnimatedElementTemplate {
    pub fn new(speed: f32, tail_length: usize) -> Self {
        AnimatedElementTemplate { speed, tail_length }
    }
}

impl Effect for AnimatedElementTemplate {
    fn update(&mut self, controller: &mut LedController, time: f32, _delta: f32) {
        // Clear all zones
        controller.fill(Color::black());
        
        // Calculate position of animated element
        let position = ((time * self.speed * 10.0) as usize) % NUM_ZONES;
        
        // Draw element with tail
        for i in 0..self.tail_length {
            let zone = (position + NUM_ZONES - i) % NUM_ZONES;
            let brightness = 1.0 - (i as f32 / self.tail_length as f32);
            let color = Color::white().scale(brightness);
            controller.set_zone(zone, color);
        }
        
        // Flush using command 0x04
        let _ = controller.flush_buffered();
    }
    
    fn name(&self) -> &str {
        "Animated Element Template"
    }
}

// Template 4: GRADIENT/TRANSITION EFFECT
// Use this for smooth color transitions
pub struct GradientTemplate {
    color1: Color,
    color2: Color,
}

impl GradientTemplate {
    pub fn new(color1: Color, color2: Color) -> Self {
        GradientTemplate { color1, color2 }
    }
}

impl Effect for GradientTemplate {
    fn update(&mut self, controller: &mut LedController, _time: f32, _delta: f32) {
        // Create gradient across all zones
        for i in 0..NUM_ZONES {
            let t = i as f32 / (NUM_ZONES - 1) as f32;
            let color = self.color1.lerp(&self.color2, t);
            controller.set_zone(i, color);
        }
        
        // Flush using command 0x04
        let _ = controller.flush_buffered();
    }
    
    fn name(&self) -> &str {
        "Gradient Template"
    }
}

// Template 5: STATEFUL EFFECT WITH INTERNAL TIMER
// Use this when you need to track state between updates
pub struct StatefulTemplate {
    phase: f32,
    speed: f32,
    state: usize,
}

impl StatefulTemplate {
    pub fn new(speed: f32) -> Self {
        StatefulTemplate { 
            phase: 0.0,
            speed,
            state: 0,
        }
    }
}

impl Effect for StatefulTemplate {
    fn start(&mut self) {
        self.phase = 0.0;
        self.state = 0;
    }
    
    fn update(&mut self, controller: &mut LedController, _time: f32, delta: f32) {
        // Update internal state
        self.phase += delta * self.speed;
        
        if self.phase >= 1.0 {
            self.phase = 0.0;
            self.state = (self.state + 1) % 4; // Cycle through 4 states
        }
        
        // Use state to determine what to display
        let color = match self.state {
            0 => Color::red(),
            1 => Color::green(),
            2 => Color::blue(),
            _ => Color::white(),
        };
        
        let _ = controller.set_all_instant(color);
    }
    
    fn stop(&mut self, controller: &mut LedController) {
        self.phase = 0.0;
        self.state = 0;
        let _ = controller.clear();
    }
    
    fn name(&self) -> &str {
        "Stateful Template"
    }
}

// ===================================================================
// HELPER FUNCTIONS FOR EFFECTS
// ===================================================================

/// Convert zone index to normalized position (0.0 to 1.0)
pub fn zone_to_normalized(zone: usize) -> f32 {
    zone as f32 / (NUM_ZONES - 1) as f32
}

/// Get distance from center zone
pub fn distance_from_center(zone: usize) -> f32 {
    let center = NUM_ZONES as f32 / 2.0;
    (zone as f32 - center).abs() / center
}

/// Smooth step interpolation (ease in/out)
pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Generate a wave value at position
pub fn wave(position: f32, time: f32, frequency: f32) -> f32 {
    ((position * frequency + time) * std::f32::consts::PI * 2.0).sin()
}

// ===================================================================
// USAGE EXAMPLE
// ===================================================================

#[cfg(test)]
mod effect_examples {
    use super::*;
    
    // Example: Create a custom effect
    pub struct MyCustomEffect {
        my_parameter: f32,
    }
    
    impl MyCustomEffect {
        pub fn new(my_parameter: f32) -> Self {
            MyCustomEffect { my_parameter }
        }
    }
    
    impl Effect for MyCustomEffect {
        fn update(&mut self, controller: &mut LedController, time: f32, _delta: f32) {
            // Your custom logic here
            for i in 0..NUM_ZONES {
                let hue = (time * self.my_parameter + i as f32 * 10.0) % 360.0;
                let color = Color::from_hsv(hue, 1.0, 1.0);
                controller.set_zone(i, color);
            }
            let _ = controller.flush_buffered();
        }
        
        fn name(&self) -> &str {
            "My Custom Effect"
        }
    }
}