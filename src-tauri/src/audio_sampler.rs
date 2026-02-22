use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::atomic::{AtomicU32, Ordering};
use once_cell::sync::Lazy;
use std::sync::Mutex;

static INTENSITY: AtomicU32 = AtomicU32::new(0);

// Wrapper to allow cpal::Stream to be stored in a global static Mutex
// On Windows, cpal::Stream is not Send because it may contain COM pointers.
// We unsafe impl Send because we only use it to keep the stream alive.
struct SendStream(cpal::Stream);
unsafe impl Send for SendStream {}

static STREAM: Lazy<Mutex<Option<SendStream>>> = Lazy::new(|| Mutex::new(None));

pub struct AudioSampler;

impl AudioSampler {
    pub fn new() -> anyhow::Result<Self> {
        let mut stream_lock = STREAM.lock().unwrap();
        if stream_lock.is_none() {
            let host = cpal::default_host();
            // On Windows, we want the default output device for loopback
            let device = host.default_output_device()
                .ok_or_else(|| anyhow::anyhow!("No default output device found"))?;

            let device_name = device.name().unwrap_or_else(|_| "Unknown".to_string());
            println!("Selected audio device for sparkle effect: {}", device_name);

            // Verify if it's likely an output device (for loopback)
            if device_name.to_lowercase().contains("microphone") || device_name.to_lowercase().contains("input") {
                eprintln!("Warning: Default output device '{}' looks like an input device. Loopback might fail.", device_name);
            }

            let config = device.default_output_config()?;
            println!("Audio config: {:?}", config);
            
            let err_fn = |err| eprintln!("an error occurred on audio stream: {}", err);

            let stream = match config.sample_format() {
                cpal::SampleFormat::F32 => device.build_input_stream(
                    &config.into(),
                    move |data: &[f32], _: &cpal::InputCallbackInfo| {
                        Self::process_audio(data);
                    },
                    err_fn,
                    None
                )?,
                cpal::SampleFormat::I16 => device.build_input_stream(
                    &config.into(),
                    move |data: &[i16], _: &cpal::InputCallbackInfo| {
                        let f32_data: Vec<f32> = data.iter().map(|&s| s as f32 / i16::MAX as f32).collect();
                        Self::process_audio(&f32_data);
                    },
                    err_fn,
                    None
                )?,
                cpal::SampleFormat::U16 => device.build_input_stream(
                    &config.into(),
                    move |data: &[u16], _: &cpal::InputCallbackInfo| {
                        let f32_data: Vec<f32> = data.iter().map(|&s| (s as f32 - u16::MAX as f32 / 2.0) / (u16::MAX as f32 / 2.0)).collect();
                        Self::process_audio(&f32_data);
                    },
                    err_fn,
                    None
                )?,
                _ => return Err(anyhow::anyhow!("Unsupported sample format")),
            };

            stream.play()?;
            *stream_lock = Some(SendStream(stream));
        }

        Ok(Self)
    }

    fn process_audio(data: &[f32]) {
        if data.is_empty() { return; }
        
        let sum_sq: f32 = data.iter().map(|&sample| sample * sample).sum();
        let rms = (sum_sq / data.len() as f32).sqrt();
        
        // Smooth the intensity a bit
        let current_bits = INTENSITY.load(Ordering::Relaxed);
        let current = f32::from_bits(current_bits);
        let smoothed = current * 0.8 + rms * 0.2;
        
        INTENSITY.store(smoothed.to_bits(), Ordering::Relaxed);

        // Periodic diagnostic log (every ~100 frames)
        static mut COUNTER: u32 = 0;
        unsafe {
            COUNTER += 1;
            if COUNTER >= 100 {
                COUNTER = 0;
                // println!("Audio Intensity: {:.4}", smoothed); // Re-running tauri dev will show this
            }
        }
    }

    pub fn get_intensity(&self) -> f32 {
        f32::from_bits(INTENSITY.load(Ordering::Relaxed))
    }
}
