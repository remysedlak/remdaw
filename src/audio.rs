use crate::model::{AudioState};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, Stream};
use hound;
use std::sync::{Arc, Mutex};

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


pub fn init() -> (Stream, Arc<Mutex<AudioState>>) {

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");
    let supported_config = device
        .default_output_config()
        .expect("error getting default config");
    let config = supported_config.config();
    let sample_format = supported_config.sample_format();
    let sample_rate = config.sample_rate.0 as f32;

    let audio_state = Arc::new(Mutex::new(AudioState::new(sample_rate)));
    let audio_state_clone = audio_state.clone();

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
    let mut state = state.lock().unwrap();
    let channels = 2; // or get from config

    for frame in data.chunks_mut(channels) {
        // Check metronome timing
        if state.is_playing {
            if state.is_metronome {
                if state.metronome_counter >= state.samples_per_beat {
                    state.instruments[2].is_playing = true;
                    state.instruments[2].position = 0;
                    state.metronome_counter -= state.samples_per_beat;
                }
                state.metronome_counter += 1.0;
            }
        }

        // Mix all instruments
        let mut mix = 0.0;
        for instrument in &mut state.instruments {
            if instrument.is_playing {
                if instrument.position < instrument.samples.len() {
                    mix += instrument.samples[instrument.position];
                    instrument.position += 1;
                } else {
                    instrument.is_playing = false;
                }
            }
        }

        // Write to both channels
        for sample in frame.iter_mut() {
            *sample = mix;
        }
    }
}
