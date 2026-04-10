use cpal::SampleFormat;
use cpal::StreamConfig;
use cpal::traits::StreamTrait;
use cpal::traits::{DeviceTrait, HostTrait};

fn main() {
    println!("STARTING REMY'S DAW");
    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

    // use the default host to find devices
    let host = cpal::default_host();

    // access the devices data streams
    let device = host
        .default_output_device()
        .expect("no output device available");

    // a config must be defined to use the device properlyz
    let supported_config = device
        .default_output_config()
        .expect("error getting default config");

    let config = supported_config.config();
    let sample_format = supported_config.sample_format();

    println!("{}", config.sample_rate);

    // track sample position and fill buffer
    let mut i: f32 = 0.0;
    let sine_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        for sample in data.iter_mut() {
            *sample = (2.0 * std::f32::consts::PI * 220.0 * i / config.sample_rate as f32).sin();
            i += 1.0;
        }
    };

    let mut j: f32 = 0.0;
    let snare: Vec<f32> = path_to_vector("AttackS.wav");
    let snare_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        for sample in data.iter_mut() {
            if j < snare.len() as f32 {
                *sample = snare[j as usize];
                j += 1.0;
            }
        }
    };

    let stream = match sample_format {
        SampleFormat::F32 => device.build_output_stream(&config, snare_callback, err_fn, None),
        sample_format => panic!("Unsupported sample format '{sample_format}'"),
    }
    .expect("Failed to build the output stream.");

    // start the output stream
    stream.play().expect("Failed to play the output stream.");
    std::thread::sleep(std::time::Duration::from_secs(5));
}

// load an instrument path into a vector of floats
pub fn path_to_vector(instrument_path: &str) -> Vec<f32> {
    // Open the WAV file using the hound library
    let mut reader = match hound::WavReader::open(instrument_path) {
        Ok(result) => result,
        Err(err) => panic!("{}", err),
    };

    let spec = reader.spec();
    let divisor = 1 << (spec.bits_per_sample - 1);

    // Read all samples as i32 (32-bit audio)
    let samples = reader.samples::<i32>();

    // Convert i16 samples to f32 normalized values
    let vector: Vec<f32> = samples
        .map(|result| result.unwrap()) // Unwrap each Result<i32>
        .map(|i32_value| i32_value as f32 / divisor as f32) // Normalize to [-1.0, 1.0]
        .collect();
    vector
}
