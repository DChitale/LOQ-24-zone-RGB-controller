# Lenovo 24-Zone RGB Controller
<div align="center">
  <img width="100" height="100" alt="Fiery angular wing emblem" src="https://github.com/user-attachments/assets/4a365164-08b4-4a9d-bb85-35735d0511d4" />
</div>
<div align="center">
  <p>A lightweight controller for complete control over all 24 RGB zones of Lenovo Legion LOQ series keyboards.</p>
  
  [![Version](https://img.shields.io/github/v/release/DChitale/LOQ-24-zone-RGB-controller?include_prereleases&style=for-the-badge&color=a855f7&label=version)](https://github.com/DChitale/LOQ-24-zone-RGB-controller/releases/latest)
  [![Downloads](https://img.shields.io/github/downloads/DChitale/LOQ-24-zone-RGB-controller/total?style=for-the-badge&color=a855f7&label=downloads)](https://github.com/DChitale/LOQ-24-zone-RGB-controller/releases)
  [![Platform](https://img.shields.io/badge/platform-Windows-0078d4?style=for-the-badge)](https://github.com/DChitale/LOQ-24-zone-RGB-controller/releases/latest)
  [![Rust](https://img.shields.io/badge/Rust-Backend-ce422b?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
  [![License](https://img.shields.io/badge/license-MIT-3da639?style=for-the-badge)](LICENSE)
</div>

## Features
 
- **Granular Control:** 24 RGB zones with independent control
- **High Performance:** Low-latency updates for smooth animations up to 60 FPS with Rust backend and high-frequency FFI callbacks
- **Minimal Footprint:** Native XAML compositor reduces memory from ~200MB to ~40MB with startup times under 500ms
- **Modular Architecture:** Build and integrate custom lighting presets in Rust
- **System Tray Integration:** Background operation, minimize to tray, menu options
- **Global Hotkey:** Configurable system-wide hotkey for preset cycling
- **Direct USB HID Control:** Native hardware communication
## Supported Effects
 
**Static Effects**
- Static Color
- Color Breath
- Pulse Center
- Aurora
- Nebula
- Chromatic Breath
- Sparkle
**Dynamic Effects**
- Rainbow Breath, Rainbow Cycle, Rainbow Wave
- Color Wheel, Color Sweep
- Color Scan
- Horse Color, Horse Cycle
- Ferrari RPM
**Reactive Effects**
- Screen Ambiance
- Audio Sparkle, Audio Sparkle Rainbow, Audio Sparkle Media
- Audio Ripple
- Typing Rainbow Ripple
**System Monitoring**
- CPU/Memory/GPU Usage Status
---
 
## Getting Started
 
### Prerequisites
 
- [.NET 8 SDK](https://dotnet.microsoft.com/download/dotnet/8.0)
- [Rust & Cargo](https://rustup.rs/) (MSVC toolchain)
- Visual Studio 2022 or Build Tools with Windows App SDK workload
### Build Instructions
 
```powershell
# Clone and navigate to repository
git clone https://github.com/DChitale/LOQ-24-zone-RGB-controller
cd LOQ-24-zone-RGB-controller
 
# Run build script (compiles Rust DLL and C# app)
powershell -ExecutionPolicy Bypass -File build.ps1
```
 
### Running the Application
 
```powershell
RGBController\bin\x86\Release\net8.0-windows10.0.26100.0\win-x86\RGBController.exe
```
 
---
 
## Architecture
 
### Frontend Layer
**Location:** `RGBController/`
- **Technology:** WinUI 3 (XAML/C#), .NET 8
- **Main Shell:** MainWindow.xaml - Application entry point and layout
- **Visual Canvas:** HomePage.xaml - Win2D CanvasControl with rendering pipeline
- **Interop Layer:** RgbInterop.cs - P/Invoke bindings for Rust DLL communication
### Backend Layer
**Location:** `rust-backend/`
- **FFI Entry Point:** lib.rs - Compiles to rgb_backend.dll, exposes C# callable functions
- **Hardware Driver:** led_driver.rs - USB HID communication and LED protocol
- **Effect System:** presets/ - Modular effect implementations
### Build Output
```
RGBController/bin/x86/Release/net8.0-windows10.0.26100.0/win-x86/
```
 
---
 
## Developing Custom Effects
 
### Overview
 
Effects are implemented as Rust modules in the presets system. Each effect implements the Effect trait and can accept configurable parameters (speed, density, size, colors).
 
### Step 1: Create Effect File
 
Create a new file under `rust-backend/src/presets/`:
 
```rust
// rust-backend/src/presets/my_custom_effect.rs
use crate::effect::{Effect, EffectConfig};
 
pub struct MyCustomEffect;
 
impl Effect for MyCustomEffect {
    fn name(&self) -> &str {
        "My Custom Effect"
    }
 
    fn update(&mut self, config: &EffectConfig) -> Vec<[u8; 3]> {
        // Return RGB values for each of 24 zones
        vec![[255, 0, 0]; 24] // Example: all red
    }
}
```
 
### Step 2: Register Module
 
Add to `rust-backend/src/presets/mod.rs`:
 
```rust
pub mod my_custom_effect;
```
 
### Step 3: Define Metadata
 
In `presets/mod.rs`, add preset configuration:
 
```rust
PresetMetadata {
    id: "my_custom_effect",
    name: "My Custom Effect",
    has_speed: true,
    has_density: true,
    has_size: false,
    has_color: true,
    colors: vec![(255, 0, 0)], // Default colors
}
```
 
### Step 4: Hook into Runner
 
In `rust-backend/src/lib.rs`, add to `rgb_set_preset()` match block:
 
```rust
"my_custom_effect" => {
    let mut effect = my_custom_effect::MyCustomEffect;
    // Effect loop implementation
}
```
 
### Available Parameters
 
- **Speed:** Effect animation speed (0-100)
- **Density:** Effect intensity/frequency (0-100)
- **Size:** Effect scale/spread (0-100)
- **Color:** RGB values for customization
---
 
 
## Troubleshooting
 
**Build Fails - Rust Not Found**
- Ensure Rust is installed via rustup with MSVC toolchain
- Run: `rustup toolchain install stable-msvc`
**DLL Not Loading**
- Verify rgb_backend.dll exists in output directory
- Check Visual C++ redistributables are installed
**No LED Response**
- Ensure Lenovo Legion LOQ keyboard is connected
- Try manual USB HID device enumeration in Device Manager
---
 
## License
 
MIT License - See LICENSE file for details.
 
---

## Tested on

LOQ 15AHP10
Legion 5 15AHP10
Supports all 24 zone rgb keyboard laptop by Lenovo.
