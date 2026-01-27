#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod led_driver;
mod effects;
//mod windows_manager; // Your new file

use crate::led_driver::{LedController, Color};
use std::sync::{Arc, Mutex};
use tauri::{Manager, SystemTray, SystemTrayMenu, CustomMenuItem, SystemTrayEvent, State};

const NUM_ZONES: usize = 24;

pub struct AppState {
    pub controller: Mutex<LedController>,
    pub ui_frame: Arc<Mutex<Vec<Color>>>,
}

#[tauri::command]
fn get_frame(state: State<AppState>) -> Vec<Color> {
    state.ui_frame.lock().unwrap().clone()
}

#[tauri::command]
fn test_set_red(state: State<AppState>) -> Result<(), String> {
    let mut controller = state.controller.lock().unwrap();
    
    // Ensure connection before writing
    if !controller.is_connected() {
        controller.connect()?;
    }

    controller.set_range(0, 23, Color::new(125, 125, 125))?;
    Ok(())
}

use std::time::{Instant, Duration};


//     tauri::async_runtime::spawn(async move {
//         let start_time = Instant::now();
//         let speed = 0.5;    // Speed of color rotation
//         let density = 1.0;  // 1.0 = one full rainbow across the 24 zones

//         loop {
//             let elapsed = start_time.elapsed().as_secs_f32();
            
//             // 1. Get access to the shared state
//             let state = app_handle.state::<AppState>();
            
//             {
//                 let mut controller = state.controller.lock().unwrap();

//                 // 2. Calculate rainbow for each of the 24 zones
//                 for i in 0..NUM_ZONES {
//                     // This formula creates a traveling wave
//                     let hue = ((i as f32 / NUM_ZONES as f32) * density + (elapsed * speed)) % 1.0;
                    
//                     // Convert hue to RGB (Standard HSV to RGB conversion)
//                     let color = Color::from_hsv(hue * 360.0, 1.0, 1.0);
                    
//                     // Update the local buffer
//                     controller.set_zone(i, color);
//                 }

//                 // 3. Flush to Hardware (Sends the 0x04 packets to the Lenovo keyboard)
//                 if controller.is_connected() {
//                     let _ = controller.flush_buffered();
//                 }
//             }

//             // 4. Update frequency (approx 60 FPS)
//             // This allows the frontend polling (get_frame) to see smooth movement
//             tokio::time::sleep(Duration::from_millis(16)).await;
//         }
//     });
// }

fn run_rainbow_test(app_handle: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        let start_time = Instant::now();
        loop {
            let elapsed = start_time.elapsed().as_secs_f32();
            let state = app_handle.state::<AppState>();

            // Phase 1: Update the Shared Memory as fast as possible
            {
                let mut controller = state.controller.lock().unwrap();
                for i in 0..NUM_ZONES {
                    let hue = ((i as f32 / NUM_ZONES as f32) + (elapsed * 0.5)) % 1.0;
                    controller.set_zone(i, Color::from_hsv(hue * 360.0, 1.0, 1.0));
                }
                // Release the lock immediately after math is done
            } 

            // Phase 2: Hardware Communication (Slow part)
            // We lock briefly just to trigger the flush, but your driver 
            // should ideally handle the USB write outside the main UI lock.
            {
                let controller = state.controller.lock().unwrap();
                let _ = controller.flush_buffered();
            }

            // Phase 3: Yield to OS
            // Increase this to 32ms (30fps) if the HID device can't keep up.
            tokio::time::sleep(Duration::from_millis(16)).await;
        }
    });
}


fn main() {
    let ui_frame = Arc::new(Mutex::new(vec![Color::black(); NUM_ZONES]));
    let controller = LedController::new(ui_frame.clone());
    
    let app_state = AppState {
        controller: Mutex::new(controller),
        ui_frame: ui_frame.clone(),
    };

    let tray_menu = SystemTrayMenu::new().add_item(CustomMenuItem::new("quit", "Exit"));

    tauri::Builder::default()
        .manage(app_state)
        // CRITICAL FIX: Only one invoke_handler call with all commands
        .invoke_handler(tauri::generate_handler![get_frame, test_set_red])
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|_app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => if id == "quit" { std::process::exit(0); },
            _ => {}
        })
        .setup(|app| {
            let handle = app.handle();
            let state = handle.state::<AppState>();
            
            // Try to connect the HID device first
            {
                let mut controller = state.controller.lock().unwrap();
                let _ = controller.connect();
            }

            // START THE RAINBOW TEST LOOP
            // run_rainbow_test(handle);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error");
}