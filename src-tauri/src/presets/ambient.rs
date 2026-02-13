#![allow(non_snake_case)]

use crate::led_driver::{LedController, Color, NUM_ZONES};

//
// ================= CONFIG =================
//

const AMBIENT_WIDTH: usize = NUM_ZONES;
const AMBIENT_HEIGHT: usize = 12;
const LUMINANCE_THRESHOLD: f32 = 0.015;
/// Boost saturation so colors stay rich instead of washing to white (1.0 = no change).
const SATURATION_BOOST: f32 = 2.8;
/// Slight brightness curve so midtones pop (1.0 = linear).
const GAMMA: f32 = 0.78;
/// How much faster the transition responds (higher = snappier follow).
const TRANSITION_SPEED_MULT: f32 = 1.0;
/// Suppress white/gray: luminance above this and low chroma = scale down (0 = off).
const WHITE_LUMA_THRESHOLD: f32 = 0.48;
const WHITE_CHROMA_THRESHOLD: f32 = 0.18;
const WHITE_SUPPRESS: f32 = 0.42;
/// Suppress blue: when blue is dominant, scale blue channel by this (1.0 = no suppress).
const BLUE_SUPPRESS: f32 = 0.5;

const AMBIENT_TARGET_FPS: f32 = 45.0;

//
// ================= RGB FLOAT =================
//

#[derive(Copy, Clone)]
pub struct RgbF {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl RgbF {
    fn black() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0 }
    }

    fn add(self, o: Self) -> Self {
        Self { r: self.r + o.r, g: self.g + o.g, b: self.b + o.b }
    }

    fn scale(self, s: f32) -> Self {
        Self { r: self.r * s, g: self.g * s, b: self.b * s }
    }

    fn lerp(self, t: Self, a: f32) -> Self {
        Self {
            r: self.r + (t.r - self.r) * a,
            g: self.g + (t.g - self.g) * a,
            b: self.b + (t.b - self.b) * a,
        }
    }

    fn luminance(self) -> f32 {
        0.2126 * self.r + 0.7152 * self.g + 0.0722 * self.b
    }

    fn to_color(self) -> Color {
        Color::new(
            (self.r.clamp(0.0, 1.0) * 255.0) as u8,
            (self.g.clamp(0.0, 1.0) * 255.0) as u8,
            (self.b.clamp(0.0, 1.0) * 255.0) as u8,
        )
    }

    /// Boost saturation: pull color away from gray so it stays rich, not white.
    fn saturate(self, amount: f32) -> Self {
        let l = self.luminance();
        let gray = RgbF { r: l, g: l, b: l };
        let s = gray.add((self.add(gray.scale(-1.0))).scale(amount));
        Self {
            r: s.r.clamp(0.0, 1.0),
            g: s.g.clamp(0.0, 1.0),
            b: s.b.clamp(0.0, 1.0),
        }
    }

    /// Apply gamma for richer midtones (gamma < 1 brightens midtones).
    fn apply_gamma(self, gamma: f32) -> Self {
        Self {
            r: self.r.powf(gamma),
            g: self.g.powf(gamma),
            b: self.b.powf(gamma),
        }
    }

    /// Suppress whites (bright near-neutral) and dominant blues so other colors stand out.
    fn suppress_whites_and_blues(self) -> Self {
        let (r, g, b) = (self.r, self.g, self.b);
        let l = self.luminance();
        let max_c = r.max(g).max(b);
        let min_c = r.min(g).min(b);
        let chroma = max_c - min_c;

        // Suppress white/gray: high luminance + low chroma => scale down
        let (r, g, b) = if l >= WHITE_LUMA_THRESHOLD && chroma < WHITE_CHROMA_THRESHOLD {
            (
                r * WHITE_SUPPRESS,
                g * WHITE_SUPPRESS,
                b * WHITE_SUPPRESS,
            )
        } else {
            (r, g, b)
        };

        // Suppress blue when it's the dominant channel
        // let (r, g, b) = if b >= r && b >= g && b > 0.12 {
        //     (r, g, b * BLUE_SUPPRESS)
        // } else {
        //     (r, g, b)
        // };

        Self {
            r: r.clamp(0.0, 1.0),
            g: g.clamp(0.0, 1.0),
            b: b.clamp(0.0, 1.0),
        }
    }
}

//
// ================= SCREEN SAMPLER TRAIT =================
//

pub trait ScreenSampler: Send {
    fn sample(&mut self, out: &mut [[RgbF; AMBIENT_HEIGHT]; AMBIENT_WIDTH]) -> bool;
}

//
// ================= DXGI SAMPLER (WINDOWS) =================
//

#[cfg(target_os = "windows")]
mod dxgi {
    use super::*;
    use windows::{
        core::*,
        Win32::{
            Foundation::*,
            Graphics::{
                Direct3D::*,
                Direct3D11::*,
                Dxgi::*,
                Dxgi::Common::*,
            },
        },
    };

    pub struct DxgiScreenSampler {
        context: ID3D11DeviceContext,
        duplication: IDXGIOutputDuplication,
        staging: ID3D11Texture2D,
        width: u32,
        height: u32,
        // sampling region expressed as fractions [0.0..1.0]
        sample_top_frac: f32,    // fraction from top where sampling region starts (was 0.85)
        sample_left_frac: f32,   // fraction from left where horizontal region starts
        sample_width_frac: f32,  // fraction of total width to sample
    }

    impl DxgiScreenSampler {
        pub fn new() -> anyhow::Result<Self> {
            unsafe {
                let mut device = None;
                let mut context = None;

                D3D11CreateDevice(
                    None,
                    D3D_DRIVER_TYPE_HARDWARE,
                    HMODULE(0),
                    D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                    None::<&[_]>,
                    D3D11_SDK_VERSION,
                    Some(&mut device),
                    None,
                    Some(&mut context),
                )?;

                let device: ID3D11Device = device.unwrap();
                let context = context.unwrap();

                let dxgi_device: IDXGIDevice = device.cast()?;
                let adapter: IDXGIAdapter = dxgi_device.GetAdapter()?;
                let output: IDXGIOutput = adapter.EnumOutputs(0)?;
                let output1: IDXGIOutput1 = output.cast()?;

                let mut desc = std::mem::MaybeUninit::<DXGI_OUTPUT_DESC>::zeroed();
                output.GetDesc(desc.as_mut_ptr())?;
                let desc = desc.assume_init();
                let width = (desc.DesktopCoordinates.right - desc.DesktopCoordinates.left) as u32;
                let height = (desc.DesktopCoordinates.bottom - desc.DesktopCoordinates.top) as u32;

                let duplication = output1.DuplicateOutput(&device)?;

                let staging_desc = D3D11_TEXTURE2D_DESC {
                    Width: width,
                    Height: height,
                    MipLevels: 1,
                    ArraySize: 1,
                    Format: DXGI_FORMAT_B8G8R8A8_UNORM,
                    SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
                    Usage: D3D11_USAGE_STAGING,
                    BindFlags: 0,
                    CPUAccessFlags: D3D11_CPU_ACCESS_READ.0 as u32,
                    MiscFlags: 0,
                };

                let mut staging = None;
                device.CreateTexture2D(&staging_desc, None, Some(&mut staging))?;
                let staging = staging.unwrap();

                Ok(Self {
                    context,
                    duplication,
                    staging,
                    width,
                    height,
                    sample_top_frac: 0.85,
                    sample_left_frac: 0.0,
                    sample_width_frac: 1.0,
                })
            }
        }

        /// Set the vertical start as a fraction [0.0..1.0] from the top of the screen
        pub fn set_sample_top_fraction(&mut self, f: f32) {
            self.sample_top_frac = f.clamp(0.0, 1.0);
        }

        /// Set the horizontal sampling region using left offset and width (fractions)
        pub fn set_sample_horizontal_region(&mut self, left_frac: f32, width_frac: f32) {
            let left = left_frac.clamp(0.0, 1.0);
            let width = width_frac.clamp(0.0, 1.0);
            self.sample_left_frac = left;
            // ensure region stays within bounds
            self.sample_width_frac = if left + width > 1.0 { 1.0 - left } else { width };
        }
    }

    impl ScreenSampler for DxgiScreenSampler {
        fn sample(&mut self, out: &mut [[RgbF; AMBIENT_HEIGHT]; AMBIENT_WIDTH]) -> bool {
            unsafe {
                let mut frame_info = DXGI_OUTDUPL_FRAME_INFO::default();
                let mut resource = None;

                // Try to acquire frame - return false if no new frame available
                if self.duplication.AcquireNextFrame(0, &mut frame_info, &mut resource).is_err() {
                    return false;
                }

                // Check if there was actually an update
                let has_update = frame_info.LastPresentTime != 0 || frame_info.AccumulatedFrames > 0;

                let texture: ID3D11Texture2D = resource.unwrap().cast().unwrap();
                self.context.CopyResource(&self.staging, &texture);

                let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
                if self.context.Map(&self.staging, 0, D3D11_MAP_READ, 0, Some(&mut mapped)).is_ok() {
                    let data = mapped.pData as *const u8;
                    let pitch = mapped.RowPitch as usize;

                    // compute sampling region in pixels using configured fractions
                    let y_start = (self.height as f32 * self.sample_top_frac) as usize;
                    let region_left = (self.width as f32 * self.sample_left_frac) as usize;
                    let region_width = ((self.width as f32) * self.sample_width_frac).max(1.0) as usize;

                    for x in 0..AMBIENT_WIDTH {
                        for y in 0..AMBIENT_HEIGHT {
                            // map zone index to region pixel (clamped)
                            let sx_rel = x * region_width / AMBIENT_WIDTH;
                            let sx = (region_left + sx_rel).min(self.width as usize - 1);
                            let sy = y_start + y * (self.height as usize - y_start) / AMBIENT_HEIGHT;
                            let p = data.add(sy * pitch + sx * 4);

                            out[x][y] = RgbF {
                                b: *p.add(0) as f32 / 255.0,
                                g: *p.add(1) as f32 / 255.0,
                                r: *p.add(2) as f32 / 255.0,
                            };
                        }
                    }

                    self.context.Unmap(&self.staging, 0);
                }

                let _ = self.duplication.ReleaseFrame();
                
                // Return whether this was a real update
                has_update
            }
        }
    }
}

#[cfg(target_os = "windows")]
pub use dxgi::DxgiScreenSampler;

//
// ================= AMBIENT EFFECT =================
//

pub struct AmbientEffect<S: ScreenSampler> {
    sampler: S,
    smoothing: f32,
    last: [RgbF; AMBIENT_WIDTH],
    last_sample_time: f32,
    last_valid_sample: [[RgbF; AMBIENT_HEIGHT]; AMBIENT_WIDTH],
}

impl<S: ScreenSampler> AmbientEffect<S> {
    pub fn new(sampler: S, smoothing: f32) -> Self {
        Self {
            sampler,
            smoothing,
            last: [RgbF::black(); AMBIENT_WIDTH],
            last_sample_time: -1.0,
            last_valid_sample: [[RgbF::black(); AMBIENT_HEIGHT]; AMBIENT_WIDTH],
        }
    }
}

impl<S: ScreenSampler> crate::effects::Effect for AmbientEffect<S> {
    fn start(&mut self) {
        self.last = [RgbF::black(); AMBIENT_WIDTH];
        self.last_sample_time = -1.0;
        self.last_valid_sample = [[RgbF::black(); AMBIENT_HEIGHT]; AMBIENT_WIDTH];
    }

    fn update(&mut self, controller: &mut LedController, time: f32, delta: f32) {
        let interval = 1.0 / AMBIENT_TARGET_FPS;
        if self.last_sample_time >= 0.0 && time - self.last_sample_time < interval {
            return;
        }
        self.last_sample_time = time;

        let mut buffer = [[RgbF::black(); AMBIENT_HEIGHT]; AMBIENT_WIDTH];
        
        // Only process if we got a new frame
        let got_new_frame = self.sampler.sample(&mut buffer);
        
        if !got_new_frame {
            // No new frame - use last valid sample to avoid processing stale/invalid data
            buffer = self.last_valid_sample;
        } else {
            // Store this valid sample for future use
            self.last_valid_sample = buffer;
        }

        // Faster response: higher effective smoothing. Smoother curve: smoothstep so transition eases in/out.
        let raw_t = 1.0 - (-self.smoothing * delta * TRANSITION_SPEED_MULT).exp();
        let t = raw_t * raw_t * (3.0 - 2.0 * raw_t);

        for x in 0..AMBIENT_WIDTH {
            let mut sum = RgbF::black();
            for y in 0..AMBIENT_HEIGHT {
                sum = sum.add(buffer[x][y]);
            }

            let avg = sum.scale(1.0 / AMBIENT_HEIGHT as f32);
            let target = if avg.luminance() < LUMINANCE_THRESHOLD {
                // For very dark scenes, slowly fade to near-black
                avg.scale(0.15)
            } else {
                avg
                    .suppress_whites_and_blues()
                    .saturate(SATURATION_BOOST)
                    .apply_gamma(GAMMA)
            };

            let smoothed = self.last[x].lerp(target, t);
            self.last[x] = smoothed;

            controller.set_zone(x, smoothed.to_color());
        }

        let _ = controller.flush_buffered();
    }

    fn name(&self) -> &str {
        "Ambient Screen"
    }
}