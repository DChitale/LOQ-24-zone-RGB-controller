// effect_runner.rs
// Effect runner that manages effect execution and timing

use std::time::{Duration, Instant};
use crate::led_driver::LedController;
use crate::effects::Effect;

/// Manages effect execution with timing
pub struct EffectRunner {
    controller: LedController,
    current_effect: Option<Box<dyn Effect>>,
    start_time: Instant,
    last_update: Instant,
    target_fps: u32,
    is_running: bool,
}

impl EffectRunner {
    /// Create a new effect runner
    /// 
    /// # Arguments
    /// * `controller` - LED controller (must be connected)
    /// * `target_fps` - Target frames per second (default: 30)
    pub fn new(controller: LedController, target_fps: u32) -> Result<Self, String> {
        if !controller.is_connected() {
            return Err("Controller must be connected".to_string());
        }
        
        Ok(EffectRunner {
            controller,
            current_effect: None,
            start_time: Instant::now(),
            last_update: Instant::now(),
            target_fps: target_fps.max(1),
            is_running: false,
        })
    }
    
    /// Set and start a new effect
    pub fn set_effect(&mut self, mut effect: Box<dyn Effect>) {
        // Stop current effect
        if let Some(mut old_effect) = self.current_effect.take() {
            old_effect.stop(&mut self.controller);
        }
        
        // Start new effect
        effect.start();
        self.current_effect = Some(effect);
        self.start_time = Instant::now();
        self.last_update = Instant::now();
        self.is_running = true;
    }
    
    /// Stop the current effect
    pub fn stop(&mut self) {
        if let Some(mut effect) = self.current_effect.take() {
            effect.stop(&mut self.controller);
        }
        self.is_running = false;
    }
    
    /// Check if an effect is currently running
    pub fn is_running(&self) -> bool {
        self.is_running && self.current_effect.is_some()
    }
    
    /// Update the current effect (call this in your main loop)
    pub fn update(&mut self) {
        if !self.is_running {
            return;
        }
        
        if let Some(ref mut effect) = self.current_effect {
            let now = Instant::now();
            let time = (now - self.start_time).as_secs_f32();
            let delta = (now - self.last_update).as_secs_f32();
            self.last_update = now;
            
            effect.update(&mut self.controller, time, delta);
        }
    }
    
    /// Run the current effect for a specific duration
    pub fn run_for_duration(&mut self, duration: Duration) {
        let end_time = Instant::now() + duration;
        let frame_duration = Duration::from_secs_f32(1.0 / self.target_fps as f32);
        
        while Instant::now() < end_time && self.is_running {
            let frame_start = Instant::now();
            
            self.update();
            
            // Sleep to maintain target FPS
            let elapsed = frame_start.elapsed();
            if elapsed < frame_duration {
                std::thread::sleep(frame_duration - elapsed);
            }
        }
    }
    
    /// Run the current effect until stopped manually
    pub fn run_until_stopped(&mut self) {
        let frame_duration = Duration::from_secs_f32(1.0 / self.target_fps as f32);
        
        while self.is_running {
            let frame_start = Instant::now();
            
            self.update();
            
            // Sleep to maintain target FPS
            let elapsed = frame_start.elapsed();
            if elapsed < frame_duration {
                std::thread::sleep(frame_duration - elapsed);
            }
        }
    }
    
    /// Get mutable reference to the controller
    pub fn controller_mut(&mut self) -> &mut LedController {
        &mut self.controller
    }
    
    /// Get reference to the controller
    pub fn controller(&self) -> &LedController {
        &self.controller
    }
    
    /// Set target FPS
    pub fn set_target_fps(&mut self, fps: u32) {
        self.target_fps = fps.max(1);
    }
    
    /// Get current effect name
    pub fn current_effect_name(&self) -> Option<&str> {
        self.current_effect.as_ref().map(|e| e.name())
    }
}

impl Drop for EffectRunner {
    fn drop(&mut self) {
        self.stop();
    }
}

// ===================================================================
// EFFECT PLAYLIST
// ===================================================================

/// Manages a playlist of effects with durations
pub struct EffectPlaylist {
    runner: EffectRunner,
    playlist: Vec<(Box<dyn Effect>, Duration)>,
    current_index: usize,
    loop_playlist: bool,
}

impl EffectPlaylist {
    pub fn new(controller: LedController, target_fps: u32) -> Result<Self, String> {
        Ok(EffectPlaylist {
            runner: EffectRunner::new(controller, target_fps)?,
            playlist: Vec::new(),
            current_index: 0,
            loop_playlist: false,
        })
    }
    
    /// Add an effect to the playlist
    pub fn add(&mut self, effect: Box<dyn Effect>, duration: Duration) {
        self.playlist.push((effect, duration));
    }
    
    /// Set whether the playlist should loop
    pub fn set_loop(&mut self, should_loop: bool) {
        self.loop_playlist = should_loop;
    }
    
    /// Play the entire playlist
    pub fn play(&mut self) {
        loop {
            if self.current_index >= self.playlist.len() {
                if self.loop_playlist {
                    self.current_index = 0;
                } else {
                    break;
                }
            }
            
            let (effect, duration) = self.playlist.remove(self.current_index);
            println!("Playing: {}", effect.name());
            
            self.runner.set_effect(effect);
            self.runner.run_for_duration(duration);
            
            // Re-add the effect for potential loops
            let effect = self.runner.current_effect.take().unwrap();
            self.playlist.insert(self.current_index, (effect, duration));
            
            self.current_index += 1;
        }
    }
}

// ===================================================================
// USAGE EXAMPLES
// ===================================================================

#[cfg(test)]
mod examples {
    use super::*;
    use crate::led_driver::{LedController, Color, NUM_ZONES};
    use crate::effects::{SolidEffect, RainbowWaveEffect, BreathingEffect};
    use std::sync::{Arc, Mutex};
    
    #[test]
    #[ignore] // Requires actual hardware
    fn example_basic_usage() {
        // Create and connect controller
        let ui_frame = Arc::new(Mutex::new(vec![Color::black(); NUM_ZONES]));
        let mut controller = LedController::new(ui_frame.clone());
        controller.connect().expect("Failed to connect");
        
        // Create runner
        let mut runner = EffectRunner::new(controller, 30).expect("Failed to create runner");
        
        // Run rainbow effect for 5 seconds
        runner.set_effect(Box::new(RainbowWaveEffect::new(1.0)));
        runner.run_for_duration(Duration::from_secs(5));
        
        // Switch to breathing effect
        runner.set_effect(Box::new(BreathingEffect::new(Color::blue(), 0.8)));
        runner.run_for_duration(Duration::from_secs(5));
        
        // Clean up (automatically stops on drop)
    }
    
    #[test]
    #[ignore] // Requires actual hardware
    fn example_playlist() {
        let ui_frame = Arc::new(Mutex::new(vec![Color::black(); NUM_ZONES]));
        let mut controller = LedController::new(ui_frame.clone());
        controller.connect().expect("Failed to connect");
        
        let mut playlist = EffectPlaylist::new(controller, 30).expect("Failed to create playlist");
        
        // Add effects to playlist
        playlist.add(Box::new(SolidEffect::new(Color::red())), Duration::from_secs(2));
        playlist.add(Box::new(RainbowWaveEffect::new(1.0)), Duration::from_secs(5));
        playlist.add(Box::new(BreathingEffect::new(Color::blue(), 0.8)), Duration::from_secs(5));
        
        // Play once
        playlist.set_loop(false);
        playlist.play();
    }
    
    #[test]
    #[ignore] // Requires actual hardware
    fn example_manual_loop() {
        let ui_frame = Arc::new(Mutex::new(vec![Color::black(); NUM_ZONES]));
        let mut controller = LedController::new(ui_frame.clone());
        controller.connect().expect("Failed to connect");
        
        let mut runner = EffectRunner::new(controller, 30).expect("Failed to create runner");
        
        // Set effect
        runner.set_effect(Box::new(RainbowWaveEffect::new(1.0)));
        
        // Manual update loop (useful for integration with game loops, etc.)
        for _ in 0..300 { // ~10 seconds at 30 FPS
            runner.update();
            std::thread::sleep(Duration::from_millis(33));
        }
    }
}