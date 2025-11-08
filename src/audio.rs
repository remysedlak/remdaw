use cpal::{SampleFormat, Stream};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};

pub fn init() -> Stream {
    let host = cpal::default_host();
    let device = host.default_output_device()
        .expect("no output device available");
    let supported_config = device.default_output_config()
        .expect("error getting default config");
    let config = supported_config.config();
    let sample_format = supported_config.sample_format();
    let sample_rate = config.sample_rate.0 as f32;

    // Track our position in the wave
    let phase = Arc::new(Mutex::new(0.0_f32));
    let phase_clone = phase.clone();

    let err_fn = |err| eprintln!("error: {}", err);
    let stream = match sample_format {
        SampleFormat::F32 => device.build_output_stream(
            &config,
            move |data: &mut [f32], _| write_tone(data, &phase_clone, sample_rate),
            err_fn,
            None
        ),
        _ => panic!("Unsupported format")
    }.unwrap();
    stream.play().unwrap();
    stream
}

fn write_tone(data: &mut [f32], phase: &Arc<Mutex<f32>>, sample_rate: f32) {
    let frequency = 440.0; // A note
    let mut phase_val = phase.lock().unwrap();

    for sample in data.iter_mut() {
        *sample = (*phase_val * 2.0 * std::f32::consts::PI).sin();
        *phase_val = (*phase_val + frequency / sample_rate) % 1.0;
    }
}