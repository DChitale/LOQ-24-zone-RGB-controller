use rdev::{listen, Event, EventType, Key};
use once_cell::sync::Lazy;
use std::sync::Mutex;

// Store the index of the last pressed zone (0-23)
// We use a Mutex<Vec<u32>> to store a queue of recent key presses for the ripple effect to consume
pub static KEY_EVENTS: Lazy<Mutex<Vec<u32>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn start_key_listener() {
    std::thread::spawn(|| {
        if let Err(error) = listen(callback) {
            println!("Error starting key listener: {:?}", error);
        }
    });
}

fn callback(event: Event) {
    if let EventType::KeyPress(key) = event.event_type {
        let zone = map_key_to_zone(key);
        if let Ok(mut events) = KEY_EVENTS.lock() {
            events.push(zone);
            // Keep only the last 10 events to avoid bloating if the effect isn't running
            if events.len() > 10 {
                events.remove(0);
            }
        }
    }
}

fn map_key_to_zone(key: Key) -> u32 {
    match key {
        // Col 1 & 2 (Left Edge)
        Key::Escape | Key::BackQuote | Key::Tab | Key::CapsLock | Key::ShiftLeft | Key::ControlLeft => 0,
        Key::F1 | Key::Num1 => 1,
        
        // Col 3 & 4
        Key::F2 | Key::Num2 | Key::KeyQ | Key::KeyA => 2,
        Key::KeyW | Key::KeyZ | Key::MetaLeft => 3,
        
        // Col 5 & 6
        Key::F3 | Key::Num3 | Key::KeyS | Key::KeyX | Key::Alt => 4,
        Key::F4 | Key::Num4 | Key::KeyE | Key::KeyD => 5,
        
        // Col 7 & 8 (Center Left)
        Key::F5 | Key::Num5 | Key::KeyR | Key::KeyF | Key::KeyC => 6,
        Key::F6 | Key::KeyT | Key::KeyV => 7,
        
        // Col 9 & 10 (Center Right)
        Key::F7 | Key::Num6 | Key::KeyG | Key::KeyB => 8,
        Key::Num7 | Key::KeyY | Key::KeyH => 9,
        
        // Col 11 & 12
        Key::F8 | Key::KeyU | Key::KeyN => 10,
        Key::F9 | Key::Num8 | Key::KeyJ | Key::KeyM => 11,
        
        // Col 13 & 14
        Key::F10 | Key::Num9 | Key::KeyI | Key::KeyK | Key::Comma | Key::AltGr => 12,
        Key::F11 | Key::Num0 | Key::KeyO | Key::KeyL => 13,
        
        // Col 15 & 16
        Key::F12 | Key::KeyP | Key::Dot | Key::MetaRight | Key::Function => 14,
        Key::Minus | Key::SemiColon | Key::Slash | Key::LeftArrow => 15,
        
        // Mid-Right (Arrows/Edit Block)
        Key::Insert | Key::Equal | Key::LeftBracket | Key::Quote | Key::UpArrow | Key::DownArrow => 16,
        Key::PrintScreen | Key::Backspace | Key::RightBracket | Key::Return | Key::ShiftRight => 17,
        Key::Delete | Key::BackSlash | Key::RightArrow => 18,
        
        // Numpad Area
        Key::Home | Key::NumLock | Key::Kp7 | Key::Kp4 | Key::Kp1 | Key::Kp0 => 20,
        Key::End | Key::KpDivide | Key::Kp8 | Key::Kp5 | Key::Kp2 => 21,
        Key::PageUp | Key::KpMultiply | Key::Kp9 | Key::Kp6 | Key::Kp3 | Key::KpDelete => 22,
        Key::PageDown | Key::KpMinus | Key::KpPlus | Key::KpReturn => 23,

        // Spacebar spans many zones (05-12), mapping to 8 (center-ish)
        Key::Space => 8,

        _ => 12, // Default to middle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specific_mapping_cases() {
        // Col 1 & 2
        assert_eq!(map_key_to_zone(Key::Escape), 0);
        assert_eq!(map_key_to_zone(Key::F1), 1);
        
        // Mid-Right (Edit Block) - The PR review concerns
        assert_eq!(map_key_to_zone(Key::Insert), 16);
        assert_eq!(map_key_to_zone(Key::PrintScreen), 17);
        assert_eq!(map_key_to_zone(Key::Delete), 18);
        
        // Arrows
        assert_eq!(map_key_to_zone(Key::LeftArrow), 15);
        assert_eq!(map_key_to_zone(Key::UpArrow), 16);
        assert_eq!(map_key_to_zone(Key::DownArrow), 16);
        assert_eq!(map_key_to_zone(Key::RightArrow), 18);
        
        // Numpad
        assert_eq!(map_key_to_zone(Key::NumLock), 20);
        assert_eq!(map_key_to_zone(Key::Kp5), 21);
        assert_eq!(map_key_to_zone(Key::Kp9), 22);
        assert_eq!(map_key_to_zone(Key::KpReturn), 23);
        
        // Space
        assert_eq!(map_key_to_zone(Key::Space), 8);
    }
}
