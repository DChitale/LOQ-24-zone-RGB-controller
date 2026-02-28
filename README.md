# Lenovo 24-Zone RGB Controller

[![Tauri / Rust / Next.js](https://skillicons.dev/icons?i=tauri,rust,next)](https://tauri.app/)

A lightweight, high-performance controller for managing 24 independent RGB zones. Designed for speed, flexibility, and smooth lighting effects on Lenovo devices, powered by a bleeding-edge stack using Rust, Tauri, and Next.js.

## ✨ Features

- **Granular Control:** 24 RGB zones with independent control.
- **High Performance:** Low-latency updates for smooth animations up to 60 FPS, driven directly by a Rust backend.
- **Modular Architecture:** Build and integrate new custom lighting presets with ease.
- **Resource Efficient:** Minimal system footprint compared to heavy manufacturer software.
- **Direct USB HID Control:** Speaks natively to the hardware backend.

## 🎨 Supported Effects

- Static Color
- CPU-Mem-GPU usage status
- Screen Ambiance *(Reactive)*
- Color Breath
- Pulse Center
- Horse Color & Horse Cycle
- Ferrari RPM
- Rainbow Breath, Rainbow Cycle, & Rainbow Wave
- ColorWheelEffect & Color sweep
- Aurora
- Color Scan
- Sparkle
- Nebula
- Chromatic Breath
- **Audio Reactive:** Audio Sparkle, Audio Sparkle Rainbow, Audio Sparkle Media, Audio Ripple
- **Typing Reactive:** Typing Rainbow Ripple

> **Note:** A few effects may have minor inconsistencies depending on local environment settings (e.g., *Screen Ambiance* relies on external services like DXGI, and *CPU-Mem-GPU usage* depends on Sysinfo/NVML). Complex dynamic effects must follow the provided Rust templates.

---

## 🚀 Quick Start — Development (Windows)

### Prerequisites
- [Node.js](https://nodejs.org/) (18+) and your preferred package manager (npm, pnpm, yarn)
- [Rust & Cargo](https://rustup.rs/) (installed via `rustup`, MSVC toolchain)
- Visual Studio Build Tools (Requires the *"Desktop development with C++"* workload)

### Frontend-Only Mode (Fast UI Iteration)
Mock the UI and tweak frontend logic without spinning up the Rust backend.
```bash
npm install
npm run dev
# App will be accessible at http://localhost:3000
```

### Full Desktop Application (Tauri + Next.js)
Launch the fully integrated desktop application with the Rust backend handling live USB HID communication.
```bash
npm install
npm run tauri -- dev
```

---

## 📦 Building & Packaging (Windows)

1. **Production Frontend Build:**
   Compile the Next.js assets for production.
   ```bash
   npm run build
   ```

2. **Package the Native App:**
   Build the Tauri application bundle.
   ```bash
   npm run tauri -- build
   ```
   > *Build artifacts (MSI / NSIS installers) will be generated and placed in `src-tauri/target/release/bundle/` upon success.*

**Packaging Notes:**
- Ensure the MSVC toolchain is completely installed. 
- Using the `.msi` installer is recommended. If Windows encounters a deployment error, fallback to the NSIS executable.

---

## 🏗️ Architecture Overview

- **Frontend (UI):** `src/app/` — Built with React and Next.js. (Components reside in `src/app/components/`)
- **Backend (Tauri + Rust):** `src-tauri/src/` — Manages the hardware driver, effect loops, and plugin presets.
  - *Hardware Driver:* `src-tauri/src/led_driver.rs`
  - *Effect Protocols:* `src-tauri/src/presets/`
  - *Entry Point:* `src-tauri/src/main.rs`
- **Build Output:** `src-tauri/target/release/bundle/`

---

## 🛠️ Creating Custom Effects

You can implement entirely customized static or dynamic effects using Rust. Use the `src-tauri/src/effects.rs` interface template to get started.

1. **Create the Effect File:**
   Create a new file under `src-tauri/src/presets/` (e.g., `my_custom_effect.rs`) and implement the `Effect` trait.
2. **Register the Module:**
   Open `src-tauri/src/presets/mod.rs` and expose your module:
   ```rust
   pub mod my_custom_effect;
   ```
3. **Define Preset Metadata:**
   In the `get_available_presets()` function inside `presets/mod.rs`, add your `PresetMetadata` config block. This will automatically expose your effect to the Frontend UI along with any adjustable parameters (Speed, Density, Size, Colors).
4. **Hook into the Runner:**
   In `src-tauri/src/main.rs`, import your effect module and append it to the `match preset_name_lc.as_str()` arm block inside the `set_preset` Tauri command.
5. **Manage Dependencies:**
   If your effect relies on external crates (e.g., for audio routing or system hooks), add them to `src-tauri/Cargo.toml`.

> **Tip:** We recommend reading the `src-tauri/src/led_driver.rs` file to understand how the internal buffer pushes frames to the hardware.

---

## 🐛 Troubleshooting & Tips

- **Device Not Detected:** Ensure you are running the app as an Administrator. Check Device Manager to confirm your device is transmitting USB/HID data normally.
- **Lights Freezing/Overridden:** Ensure `Windows Dynamic Lighting` is toggled ON in Windows Settings. Other manufacturer software (e.g., Lenovo Legion Vantage) may try to aggressively regain control of the HID endpoints.
- **Hardware PID/VID Mismatch:** If your keyboard remains dark, check the generic `PID` and `VID` of your device through Device Manager and update the raw declarations inside `src-tauri/src/led_driver.rs`.
- **Debugging:** Tauri developer logs will seamlessly pipe directly into your command line output while running `npm run tauri -- dev`.
- **Rust Toolchains:** Ensure you are fully on the stable MSVC stream: `rustup default stable-x86_64-pc-windows-msvc`.

---

## 🤝 Contributing

We welcome contributions ranging from bug fixes and code refactoring to documentation and new preset effects!

### How to Contribute

1. **Fork & Clone:** Fork the repository to your own account and pull it locally.
2. **Create a Branch:** Keep your workflow clean by branching off `main`.
   ```bash
   git checkout -b feature/my-amazing-effect
   ```
3. **Write Quality Code:**
   - Keep commits focused, modular, and self-contained.
   - Run the frontend linter: `npm run lint`
   - Run the Rust formatter and linter: 
     ```bash
     cd src-tauri 
     cargo fmt
     cargo clippy
     ```
4. **Hardware Validation:** If your PR interacts with the timing loop or the hardware drivers directly, please clearly mention how you tested it (hardware topology, OS version). Try to test complex patterns using a software loopback to prevent writing harmful buffer combinations to the LEDs.
5. **Submit a Pull Request:** Open a PR back to this repository and include a clear summary of your additions and steps confirming how they were verified. 

---

## 📋 Pending Roadmap

- [ ] Support flexible `PID` and `VID` lookup tables to support more generic Lenovo iterations.
- [ ] Transition the internal hard-coded effects engine over to a dynamic JSON-based loader.
- [ ] Ongoing stability, thread-locking, and bug patching across hardware sleep/wake lifecycles.
- [ ] Native support and drivers for Unix/Linux environments.

---

## 📜 License

This project includes a `LICENSE` file in the repository root — please follow and respect the terms provided in that file.

---
*Created by the project contributors. Designed for ultimate keyboard control without the system bloat.*
