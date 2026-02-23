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
        Key::Escape | Key::BackQuote | Key::Tab | Key::CapsLock | Key::ShiftLeft | Key::ControlLeft => 0,
        Key::F1 | Key::Num1 | Key::KeyQ | Key::KeyA | Key::KeyZ => 1,
        Key::F2 | Key::Num2 | Key::KeyW | Key::KeyS | Key::KeyX => 2,
        Key::F3 | Key::Num3 | Key::KeyE | Key::KeyD | Key::KeyC => 3,
        Key::F4 | Key::Num4 | Key::KeyR | Key::KeyF | Key::KeyV => 4,
        Key::F5 | Key::Num5 | Key::KeyT | Key::KeyG | Key::KeyB => 5,
        Key::F6 | Key::Num6 | Key::KeyY | Key::KeyH | Key::KeyN => 6,
        Key::F7 | Key::Num7 | Key::KeyU | Key::KeyJ | Key::KeyM => 7,
        Key::F8 | Key::Num8 | Key::KeyI | Key::KeyK | Key::Comma => 8,
        Key::F9 | Key::Num9 | Key::KeyO | Key::KeyL | Key::Dot => 9,
        Key::F10 | Key::Num0 | Key::KeyP | Key::SemiColon | Key::Slash => 10,
        Key::F11 | Key::Minus | Key::LeftBracket | Key::Quote => 11,
        Key::F12 | Key::Equal | Key::RightBracket | Key::BackSlash => 12,
        Key::Backspace | Key::Return | Key::ShiftRight | Key::ControlRight => 13,
        Key::PrintScreen | Key::Insert | Key::Delete => 14,
        Key::ScrollLock | Key::Home | Key::End => 15,
        Key::Pause | Key::PageUp | Key::PageDown => 16,
        Key::UpArrow | Key::DownArrow | Key::LeftArrow | Key::RightArrow => 17,
        Key::NumLock | Key::Kp7 | Key::Kp4 | Key::Kp1 => 18,
        Key::KpDivide | Key::Kp8 | Key::Kp5 | Key::Kp2 | Key::Kp0 => 19,
        Key::KpMultiply | Key::Kp9 | Key::Kp6 | Key::Kp3 | Key::KpDelete => 20,
        Key::KpMinus | Key::KpPlus | Key::KpReturn => 21,
        _ => 12, // Default to middle
    }
}
