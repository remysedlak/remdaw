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
    let vector: Vec<f32> = samples
        .map(|result| result.unwrap()) /* unwrap result */
        .map(|i16_value| i16_value as f32 / i16::MAX as f32) /* convert to f32 */
        .collect();
    vector
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
    let channels = 2;

    for frame in data.chunks_mut(channels) {
        if state.is_playing {
            // 16th note resolution: divide samples_per_beat by 4
            let step = (state.metronome_counter / (state.samples_per_beat / 4.0)) as usize % 16;

            if step != state.current_step {
                state.current_step = step;

                // Trigger instruments based on pattern
                for i in 0..state.instruments.len() {
                    if state.pattern[i][step] {
                        state.instruments[i].is_playing = true;
                        state.instruments[i].position = 0;
                    }
                }

                // Trigger metronome only every 4 steps (on the beat)
                if state.is_metronome && step % 4 == 0 {
                    state.metronome_playing = true;
                    state.metronome_position = 0;
                }
            }

            state.metronome_counter += 1.0;
        }

        // Mix all instruments
        let mut mix = 0.0;

        // Mix channel rack instruments
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

        // Mix metronome separately
        if state.metronome_playing {
            if state.metronome_position < state.metronome_sample.len() {
                mix += state.metronome_sample[state.metronome_position];
                state.metronome_position += 1;
            } else {
                state.metronome_playing = false;
            }
        }

        // Mix preview sound separately
        if let Some(ref mut preview) = state.preview_sound {
            if preview.is_playing {
                if preview.position < preview.samples.len() {
                    mix += preview.samples[preview.position];
                    preview.position += 1;
                } else {
                    preview.is_playing = false;
                    state.preview_sound = None;  // Clear after playing
                }
            }
        }

        for sample in frame.iter_mut() {
            *sample = mix;
        }
    }
}