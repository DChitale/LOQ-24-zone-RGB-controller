# Lenovo 24-Zone RGB controller
## Blazing-fast Tauri app using Rust + Next.js

![Logo](https://skillicons.dev/icons?i=tauri,rust,next)



A lightweight, high-performance controller for managing 24 independent RGB zones. Designed for speed, flexibility, and smooth lighting effects.
## Features
- 24 RGB zones with independent control
- Low-latency updates for smooth animations upto 60fps
- Modular architecture for easy customization
- Lightweight and efficient — minimal resource usage
- Direct USB HID control

# Currently Supported Effects

- Static color
- CPU-Mem-GPU usage status
- Screen Ambiance light effect.
- Color Breath
- Pulse Center
- Horse Color
- Horse Cycle
- Ferrari RPM
- Rainbow Breath
- Rainbow Cycle
- Rainbow Wave
- ColorWheelEffect
- Color sweep
- Aurora
- Heat Wave
- Color Scan
- Sparkle
- Ocean Wave
- Horizon
- Nebula
- Chromatic Breath
- Silk
- Still Gradient


> A few effects may have some hiccups in their settings.
> Dynamic Effects like "Screen Ambiance" depends on eternal services like DXGI
> "CPU-Mem-GPU usage status" depends on Sysinfo and NVML (rust crates).
> More complex effects can be implemented, but they must strictly follow the provided template.

Quick start — development (Windows)
----------------------------------
Prereqs (minimal):
- Node.js 18+ and npm (or pnpm/yarn)
- Rust + cargo (installed via `rustup`, MSVC toolchain)
- Visual Studio Build Tools ("Desktop development with C++" workload)

Frontend-only (fast):

```bash
# start Next.js dev server
npm install
npm run dev
# open http://localhost:3000
```

Full desktop app (Tauri + frontend):

```bash
# from repo root
npm install
# run the integrated app (bundles frontend + Rust backend in dev)
npm run tauri -- dev
# or, if you have tauri installed globally:
# npx tauri dev
```

Build & produce installers (Windows)
------------------------------------
1. Produce frontend production build:

```bash
npm run build
```

2. Build Tauri bundle (creates installer in `src-tauri/target/release/bundle/`):

```bash
npm run tauri -- build
# artifacts -> src-tauri/target/release/bundle/
```

Notes for Windows packaging
- Ensure the MSVC toolchain is installed (Rust + MSVC). Visual Studio Build Tools with C++ is required.
- Installer artifacts (NSIS/MSI) appear under `src-tauri/target/release/bundle/` after a successful build.
- Try to use .msi installer but if windows gives any error run the nsis installer.

Architecture — where things live
--------------------------------
- Frontend (UI): `src/app/` — React + Next.js (components in `src/app/components/`).
- Tauri + Rust backend: `src-tauri/src/` — hardware driver, effect runner, presets.
  - LED driver & hardware: `src-tauri/src/led_driver.rs`
  - Effects & presets: `src-tauri/src/presets/`
  - App entry: `src-tauri/src/main.rs`
- Build outputs: `src-tauri/target/release/bundle/` (installers/bundles).

Key features
- Real-time LED effects driven in Rust for low-latency control
- Modern React UI in Next.js for configuration & previews
- Cross-platform packaging via Tauri (Windows-first assets present)

Common developer tasks
----------------------
- Lint frontend: `npm run lint`
- Run only Rust code (inside `src-tauri`):
  - `cd src-tauri && cargo run` (runs the Rust binary)
  - `cd src-tauri && cargo build --release`
- Format & check Rust: `cd src-tauri && cargo fmt && cargo clippy`

Custom effects
--------------
If you want to make any custom effect then strictly follow the template given in the `src-tauri/src/effects.rs`.
One can write any static or dynamic effect he/she wants, it is recommended to  also checkout the `src-tauri/src/led_driver.rs` as this file contains the actual driver code which communicates with the hardware.

Effects can be made using rust and any external crate that you may need, Steps to follow-
- Create a rust file under `src-tauri/src/presets` name the file according to you effects.
- Implement all the traits/functions/methods that you need from the `effects.rs` file (You can also just give the entire `effects.rs` file to a LLM and and make a effect that way).
- Once the effects logic is completed then we have to register that effects for frontend and for the backend which actually runs the effects.
    1. In `src-tauri/src/presets/mod.rs` add your effect, example 
        ``` Rust
        pub mod ambient;
        ```
    2.  Then in the same file add all the `PresetMetadata` in the function `get_available_presets()`, example
        ```Rust
        PresetMetadata {
            name: "ambient".to_string(),
            display_name: "Screen Ambiance light effect.".to_string(),
            description: "Mimics ambient light based on screen content.".to_string(),
            parameters: vec![
                ParameterConfig
                {name: "speed".to_string(),
                    label: "Speed".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.1,
                    max: 5.0,
                    default: 1.0,
                    step: 0.1,},
                ParameterConfig {
                    name: "smoothing".to_string(),
                    label: "Smoothing".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.0,
                    max: 10.0,
                    default: 5.0,
                    step: 0.1,
                },
                ParameterConfig {
                    name: "sample_top".to_string(),
                    label: "Screen sample top".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.0,
                    max: 1.0,
                    default: 0.85,
                    step: 0.01,
                },
                ParameterConfig {
                    name: "sample_left".to_string(),
                    label: "Screen sample left".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.0,
                    max: 1.0,
                    default: 0.0,
                    step: 0.01,
                },
                ParameterConfig {
                    name: "sample_width".to_string(),
                    label: "Screen sample width".to_string(),
                    param_type: ParameterType::Float,
                    min: 0.01,
                    max: 1.0,
                    default: 1.0,
                    step: 0.01,
                },
            ],
        },
        ```
    3. Then register the effect in `src-tauri/src/main.rs`  under `use crate::presets`, example
        ```Rust
        ambient::AmbientEffect,
        ```
    4. Under function `set_preset` in `main.rs` register your effect.
        ```Rust
        "ambient" => {
            #[cfg(not(target_os = "windows"))]
            return Err("Ambient effect is only supported on Windows".to_string());
            #[cfg(target_os = "windows")]
            {
                let smoothing: f32 = preset_config
                    .parameters
                    .get("smoothing")
                    .and_then(|v| match v {
                        ParameterValue::Float(f) => Some(*f),
                        _ => None,
                    })
                    .unwrap_or(1.0);

                // create sampler and apply parameters (preset overrides global settings)
                let mut sampler = crate::presets::ambient::DxgiScreenSampler::new()
                    .map_err(|e| e.to_string())?;

                // apply preset parameters if provided
                let preset_top = preset_config
                    .parameters
                    .get("sample_top")
                    .and_then(|v| match v { ParameterValue::Float(f) => Some(*f), _ => None });
                let preset_left = preset_config
                    .parameters
                    .get("sample_left")
                    .and_then(|v| match v { ParameterValue::Float(f) => Some(*f), _ => None });
                let preset_width = preset_config
                    .parameters
                    .get("sample_width")
                    .and_then(|v| match v { ParameterValue::Float(f) => Some(*f), _ => None });

                if let (Some(t), Some(l), Some(w)) = (preset_top, preset_left, preset_width) {
                    sampler.set_sample_top_fraction(t);
                    sampler.set_sample_horizontal_region(l, w);
                } else if let Ok(s) = crate::settings::load_settings() {
                    // fallback to global settings
                    sampler.set_sample_top_fraction(s.ambient_sample_top_fraction);
                    sampler.set_sample_horizontal_region(
                        s.ambient_sample_left_fraction,
                        s.ambient_sample_width_fraction,
                    );
                }

                Box::new(AmbientEffect::new(sampler, smoothing))
            }
        },
        ```
    5. Only if you are using External crates then add those in `src-tauri/cargo.toml`


Troubleshooting & tips
----------------------
> If the hardware isn't detected: run the app as Administrator (Windows), check Device Manager, and confirm USB/HID permissions.
> If the effect is not running on hardware check the `Windows Dynamic Lighting` and make sure it is turned on and in the driver application under settings click on `Take_Control_Now`
> The lenovo legion software might try to take the control back sometimes.

- Check for the `PID` and `VID` of your device and update that in `src-tauri/src/led_driver.rs`
- Tauri dev logs: visible in the terminal where you ran `npm run tauri -- dev`.
- Frontend-only issues: check the Next.js terminal output and `console` in the browser.
- Common Rust build errors: install the MSVC toolchain and run `rustup default stable-x86_64-pc-windows-msvc`.
- If packaging fails with NSIS errors, confirm `makensis` is in PATH (Tauri typically installs it or documents how to get it).

Where to look when changing behavior
-----------------------------------
- UI: `src/app/components/` (controls, sidebar, brightness, setter)
- Effects engine & presets: `src-tauri/src/effect_runner.rs`, `src-tauri/src/presets/*.rs`
- Hardware layer: `src-tauri/src/led_driver.rs`, `src-tauri/src/lighting.rs`
- App integration: `src-tauri/src/main.rs` and `src-tauri/tauri.conf.json`

Testing with hardware
---------------------
- Prefer using a development board or loopback to avoid damaging LEDs.
- Add verbose logs in Rust (`log`/`tracing`) and view them during `cargo run` or `npm run tauri -- dev`.

Contributing
------------
- Open issues for bugs or feature requests.
- For code changes: create a branch, keep commits focused, and open a PR with a short description and testing steps.
- Run linters/formatters before PR: `npm run lint` and `cd src-tauri && cargo fmt`.

Useful commands (quick reference)
---------------------------------
- Dev front end: `npm run dev`
- Dev full app: `npm run tauri -- dev`
- Build frontend: `npm run build`
- Build release bundle: `npm run tauri -- build`
- Lint: `npm run lint`
- Rust build: `cd src-tauri && cargo build --release`

License
-------
This project includes a `LICENSE` file in the repository root — follow the terms in that file.

Contact / next steps
---------------------
- To get started: run `npm install` then `npm run tauri -- dev` and connect your controller hardware. ✅
- If you'd like, I can add a short **Contributing** checklist, CI steps, or a Windows quick-install script next. 💡

Pending Features
----------------
- Support for different `PID` and `VID` under hardware.
- Implementation of custom Effects Engine using JSON.
- There are still some bugs left to iron out.
- Spuuort for Linux.