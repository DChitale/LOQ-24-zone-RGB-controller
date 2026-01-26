// led_driver.rs
// RGB Keyboard LED Driver
// Handles low-level communication with the RGB keyboard controller

use hidapi::{HidApi, HidDevice};

const VID: u16 = 0x048d;
const PID: u16 = 0xc693;
pub const NUM_ZONES: usize = 24;
const PACKET_SIZE: usize = 65; // HID report size (64 + 1 for report ID)

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Color { 
    pub r: u8, 
    pub g: u8, 
    pub b: u8 
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self { 
        Color { r, g, b } 
    }
    
    pub fn black() -> Self { 
        Color::new(0, 0, 0) 
    }
    
    pub fn white() -> Self { 
        Color::new(255, 255, 255) 
    }
    
    pub fn red() -> Self { 
        Color::new(255, 0, 0) 
    }
    
    pub fn green() -> Self { 
        Color::new(0, 255, 0) 
    }
    
    pub fn blue() -> Self { 
        Color::new(0, 0, 255) 
    }
    
    pub fn from_hsv(h: f32, s: f32, v: f32) -> Self {
        let h = h / 60.0;
        let i = h.floor() as i32 % 6;
        let f = h - h.floor();
        let p = v * (1.0 - s);
        let q = v * (1.0 - f * s);
        let t = v * (1.0 - (1.0 - f) * s);
        let (r, g, b) = match i {
            0 => (v, t, p), 
            1 => (q, v, p), 
            2 => (p, v, t),
            3 => (p, q, v), 
            4 => (t, p, v), 
            _ => (v, p, q),
        };
        Color::new((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
    }
    
    pub fn scale(&self, brightness: f32) -> Self {
        Color::new(
            (self.r as f32 * brightness.clamp(0.0, 1.0)) as u8,
            (self.g as f32 * brightness.clamp(0.0, 1.0)) as u8,
            (self.b as f32 * brightness.clamp(0.0, 1.0)) as u8,
        )
    }
    
    pub fn lerp(&self, other: &Color, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Color::new(
            (self.r as f32 + (other.r as f32 - self.r as f32) * t) as u8,
            (self.g as f32 + (other.g as f32 - self.g as f32) * t) as u8,
            (self.b as f32 + (other.b as f32 - self.b as f32) * t) as u8,
        )
    }
}

pub struct LedController {
    device: Option<HidDevice>,
    frame_buffer: [Color; NUM_ZONES],
}

impl LedController {
    pub fn new() -> Self {
        LedController { 
            device: None, 
            frame_buffer: [Color::black(); NUM_ZONES] 
        }
    }

    /// Connect to the RGB keyboard controller (interface 1)
    pub fn connect(&mut self) -> Result<(), String> {
        let api = HidApi::new().map_err(|e| e.to_string())?;
        
        // Find device with interface 1 (RGB controller)
        for device_info in api.device_list() {
            if device_info.vendor_id() == VID 
                && device_info.product_id() == PID 
                && device_info.interface_number() == 1 
            {
                self.device = api.open_path(device_info.path()).ok();
                if self.device.is_some() {
                    return Ok(());
                }
            }
        }
        
        Err("LED device not found on interface 1".to_string())
    }

    /// Check if the controller is connected
    pub fn is_connected(&self) -> bool { 
        self.device.is_some() 
    }

    /// Disconnect from the device
    pub fn disconnect(&mut self) {
        self.device = None;
    }

    // ===================================================================
    // BUFFER MANAGEMENT
    // ===================================================================

    /// Set a specific zone color in the frame buffer (does not send to device)
    pub fn set_zone(&mut self, zone: usize, color: Color) {
        if zone < NUM_ZONES { 
            self.frame_buffer[zone] = color; 
        }
    }

    /// Get the color of a specific zone from the frame buffer
    pub fn get_zone(&self, zone: usize) -> Color {
        self.frame_buffer.get(zone).cloned().unwrap_or(Color::black())
    }
    
    /// Get a reference to the entire frame buffer
    pub fn get_buffer(&self) -> &[Color; NUM_ZONES] {
        &self.frame_buffer
    }
    
    /// Set the entire frame buffer
    pub fn set_buffer(&mut self, buffer: [Color; NUM_ZONES]) {
        self.frame_buffer = buffer;
    }

    /// Fill the entire frame buffer with a single color (does not send to device)
    pub fn fill(&mut self, color: Color) {
        self.frame_buffer = [color; NUM_ZONES];
    }

    // ===================================================================
    // COMMAND 0x05: ZONE RANGE (Efficient for solid colors/ranges)
    // ===================================================================

    /// Set a range of zones to a specific color using command 0x05
    /// This command applies immediately and is more efficient for solid colors
    /// 
    /// Format: [0x05, 0x01, start_zone, 0x00, end_zone, 0x00, R, G, B, 0x01]
    /// 
    /// # Arguments
    /// * `start` - Starting zone (0-23)
    /// * `end` - Ending zone (0-23)
    /// * `color` - RGB color to apply
    pub fn set_range(&self, start: u8, end: u8, color: Color) -> Result<(), String> {
        if start > 23 || end > 23 || start > end {
            return Err(format!("Invalid zone range: {}-{}", start, end));
        }
        
        let device = self.device.as_ref().ok_or("Not connected")?;
        
        let mut buf = vec![
            0x05,     // Command: Vendor lighting
            0x01,     // Subcommand: Zone range RGB
            start,    // Start zone index
            0x00,     // Reserved (must be zero)
            end,      // End zone index
            0x00,     // Reserved (must be zero)
            color.r,  // Red (0-255)
            color.g,  // Green (0-255)
            color.b,  // Blue (0-255)
            0x01,     // Apply/Commit (1 = apply immediately)
        ];
        
        buf.resize(PACKET_SIZE, 0);
        device.send_feature_report(&buf).map_err(|e| e.to_string())?;
        Ok(())
    }
    
    /// Set all zones to a single color instantly using command 0x05
    pub fn set_all_instant(&self, color: Color) -> Result<(), String> {
        self.set_range(0, 23, color)
    }

    // ===================================================================
    // COMMAND 0x04: INDIVIDUAL ZONES (For complex patterns)
    // ===================================================================

    /// Send a packet with 8 individual zone colors using command 0x04
    /// 
    /// Format: [0x04, 0x08, commit_flag, zone_indices (16 bytes), color_data (32 bytes)]
    /// - Byte 0: 0x04 (Command)
    /// - Byte 1: 0x08 (Number of zones, always 8)
    /// - Byte 2: 0x00 or 0x01 (Commit flag - set to 1 for last packet)
    /// - Bytes 3-18: Zone indices (8 zones × 2 bytes: zone_id, 0x00)
    /// - Bytes 19-50: Color data (8 zones × 4 bytes: R, G, B, 0x01)
    /// 
    /// # Arguments
    /// * `zone_start` - Starting zone index (0, 8, or 16)
    /// * `colors` - Array of exactly 8 colors
    /// * `commit` - Set to true for the last packet to apply changes
    fn send_zone_packet(&self, zone_start: u8, colors: &[Color; 8], commit: bool) -> Result<(), String> {
        let device = self.device.as_ref().ok_or("Not connected")?;
        
        let mut buf = vec![
            0x04,                             // Command
            0x08,                             // Number of zones (always 8)
            if commit { 0x01 } else { 0x00 }, // Commit flag
        ];
        
        // Add zone indices (8 zones, 2 bytes each)
        for i in 0..8 {
            buf.push(zone_start + i);  // Zone ID
            buf.push(0x00);            // Spacer
        }
        
        // Add color data (8 zones, 4 bytes each)
        for color in colors {
            buf.push(color.r);  // Red
            buf.push(color.g);  // Green
            buf.push(color.b);  // Blue
            buf.push(0x01);     // Color commit bit
        }
        
        buf.resize(PACKET_SIZE, 0);
        device.send_feature_report(&buf).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Flush the frame buffer to the device using command 0x04
    /// Sends all 24 zones in 3 packets (8 zones each)
    /// The last packet has the commit flag set to apply changes
    pub fn flush_buffered(&self) -> Result<(), String> {
        // Prepare color arrays for each packet
        let mut colors_0_7 = [Color::black(); 8];
        let mut colors_8_15 = [Color::black(); 8];
        let mut colors_16_23 = [Color::black(); 8];
        
        colors_0_7.copy_from_slice(&self.frame_buffer[0..8]);
        colors_8_15.copy_from_slice(&self.frame_buffer[8..16]);
        colors_16_23.copy_from_slice(&self.frame_buffer[16..24]);
        
        // Send zones 0-7 (no commit)
        self.send_zone_packet(0, &colors_0_7, false)?;
        
        // Send zones 8-15 (no commit)
        self.send_zone_packet(8, &colors_8_15, false)?;
        
        // Send zones 16-23 (commit to apply all changes)
        self.send_zone_packet(16, &colors_16_23, true)?;
        
        Ok(())
    }

    /// Clear all zones (set to black) using command 0x05
    pub fn clear(&mut self) -> Result<(), String> {
        self.frame_buffer = [Color::black(); NUM_ZONES];
        self.set_all_instant(Color::black())
    }
}

impl Default for LedController {
    fn default() -> Self {
        Self::new()
    }
}

// ===================================================================
// TESTS
// ===================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let red = Color::red();
        assert_eq!(red.r, 255);
        assert_eq!(red.g, 0);
        assert_eq!(red.b, 0);
    }

    #[test]
    fn test_color_hsv() {
        let red = Color::from_hsv(0.0, 1.0, 1.0);
        assert_eq!(red.r, 255);
        
        let green = Color::from_hsv(120.0, 1.0, 1.0);
        assert_eq!(green.g, 255);
        
        let blue = Color::from_hsv(240.0, 1.0, 1.0);
        assert_eq!(blue.b, 255);
    }

    #[test]
    fn test_color_scale() {
        let white = Color::white();
        let half = white.scale(0.5);
        assert!(half.r > 120 && half.r < 130);
    }

    #[test]
    fn test_color_lerp() {
        let black = Color::black();
        let white = Color::white();
        let gray = black.lerp(&white, 0.5);
        assert!(gray.r > 120 && gray.r < 130);
    }

    #[test]
    fn test_buffer_operations() {
        let mut controller = LedController::new();
        
        controller.set_zone(0, Color::red());
        assert_eq!(controller.get_zone(0), Color::red());
        
        controller.fill(Color::blue());
        assert_eq!(controller.get_zone(0), Color::blue());
        assert_eq!(controller.get_zone(23), Color::blue());
    }
}