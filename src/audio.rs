use crate::model::{AudioState, Instrument};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, Stream};
use hound;
use std::sync::{Arc, Mutex};

impl Default for AudioState {
    fn default() -> Self {
        let mut instruments = Vec::new();

        instruments.push(Instrument {
            samples: path_to_vector("instruments/cowbell.wav"),
            position: 0,
            is_playing: false,
        });

        instruments.push(Instrument {
            samples: path_to_vector("instruments/Boss DR-660/Clap/Clap Dance.wav"),
            position: 0,
            is_playing: false,
        });

        Self {
            instruments
        }
    }
}

pub fn path_to_vector(instrument_path: &str) -> Vec<f32> {
     let mut reader = match hound::WavReader::open(instrument_path) {
        Err(err) => panic!("{}", err),
        Ok(result) => result,
    };
    let samples = reader.samples::<i16>();
    let kick_samples: Vec<f32> = samples
        .map(|result| result.unwrap()) /* unwrap result */
        .map(|i16_value| i16_value as f32 / i16::MAX as f32) /* convert to f32 */
        .collect();
    kick_samples
}

pub fn add_to_instruments(samples: Vec<f32>, mut instruments: Vec<Instrument>) -> Vec<Instrument> {
    let new_instrument = Instrument {
        is_playing: false,
        samples,
        position: 0,
    };
    instruments.push(new_instrument);
    instruments
}

pub fn init() -> (Stream, Arc<Mutex<AudioState>>) {
    let audio_state = Arc::new(Mutex::new(AudioState::default()));
    let audio_state_clone = audio_state.clone();

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");
    let supported_config = device
        .default_output_config()
        .expect("error getting default config");
    let config = supported_config.config();
    let sample_format = supported_config.sample_format();

    let err_fn = |err| eprintln!("error: {}", err);
    let stream = match sample_format {
        SampleFormat::F32 => device.build_output_stream(
            &config,
            move |data: &mut [f32], _| play_instrument(data, &audio_state_clone),
            err_fn,
            None,
        ),
        _ => panic!("Unsupported format"),
    }
    .unwrap();
    stream.play().unwrap();
    (stream, audio_state)
}

fn play_instrument(data: &mut [f32], state: &Arc<Mutex<AudioState>>) {
    let mut state = state.lock().unwrap(); // lock ONCE

    for sample in data.iter_mut() {
        let mut mix = 0.0; // accumulator ― multiple tracks is simply addition

        for instrument in &mut state.instruments {
            if instrument.is_playing {
                if instrument.position < instrument.samples.len() {
                    mix += instrument.samples[instrument.position]; // ADD to mix
                    instrument.position += 1;
                } else {
                    instrument.is_playing = false;
                }
            }
        }
        *sample = mix; // output the accumulated mix
    }
}
