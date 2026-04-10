use cpal::SampleFormat;
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
    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range
        .find(|c| c.sample_format() == SampleFormat::F32)
        .expect("no f32 config")
        .with_max_sample_rate();
    let sample_format = supported_config.sample_format();
    let config = supported_config.into();

    // track sample position and fill buffer
    let mut i: f32 = 0.0;
    let callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        for sample in data.iter_mut() {
            *sample = (2.0 * std::f32::consts::PI * 220.0 * i / 48_000.0).sin();
            i += 1.0;
        }
    };

    let stream = match sample_format {
        SampleFormat::F32 => device.build_output_stream(&config, callback, err_fn, None),
        sample_format => panic!("Unsupported sample format '{sample_format}'"),
    }
    .expect("Failed to build the output stream.");

    // start the output stream
    stream.play().expect("Failed to play the output stream.");
    std::thread::sleep(std::time::Duration::from_secs(5));
}
