use cpal::{SampleFormat, Stream};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use hound;

pub struct AudioState {
    pub kick_playing: bool,
    pub kick_position: usize,  // where we are in the sample
    pub kick_samples: Vec<f32>, // the actual WAV data
}

impl Default for AudioState {
    fn default() -> Self {
        let mut reader = hound::WavReader::open("instruments/cowbell.wav").unwrap();
        let samples = reader.samples::<i16>();
        let kick_samples: Vec<f32> = samples
                .map(|result| result.unwrap())/* unwrap result */
                .map(|i16_value| i16_value as f32 / i16::MAX as f32)/* convert to f32 */
                .collect();
        Self {
            kick_position: 0,
            kick_playing: false,
            kick_samples,
        }
    }
}

pub fn init() -> (Stream, Arc<Mutex<AudioState>>) {

    let audio_state = Arc::new(Mutex::new(AudioState::default()));
    let audio_state_clone = audio_state.clone();

    let host = cpal::default_host();
    let device = host.default_output_device()
        .expect("no output device available");
    let supported_config = device.default_output_config()
        .expect("error getting default config");
    let config = supported_config.config();
    let sample_format = supported_config.sample_format();

    let err_fn = |err| eprintln!("error: {}", err);
    let stream = match sample_format {
        SampleFormat::F32 => device.build_output_stream(
            &config,
            move |data: &mut [f32], _| write_tone(data, &audio_state_clone),
            err_fn,
            None
        ),
        _ => panic!("Unsupported format")
    }.unwrap();
    stream.play().unwrap();
    (stream, audio_state)
}

fn write_tone(data: &mut [f32], state: &Arc<Mutex<AudioState>>) {
    let mut state = state.lock().unwrap(); // lock ONCE

    for sample in data.iter_mut() {
        if state.kick_playing {
            // Am I past the end? If so, stop playing
            if state.kick_position >= state.kick_samples.len() {
                state.kick_playing = false;
                *sample = 0.0; // silence
            } else {
                *sample = state.kick_samples[state.kick_position];
                state.kick_position += 1;
            }
        } else {
            *sample = 0.0; // silence
        }
    }
}