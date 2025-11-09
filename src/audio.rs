use crate::models::{AudioState};
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
            let current_beat = state.playhead_position;
            let samples_per_beat = state.samples_per_beat;

            // Clone what we need to avoid borrow issues
            let clips = state.playlist.clips.clone();
            let patterns = state.patterns.clone();

            // Collect triggers
            let mut triggers: Vec<(usize, usize)> = Vec::new();
            let mut audio_triggers: Vec<usize> = Vec::new();

            for clip in &clips {
                let clip_start = clip.start_time;
                let clip_end = clip.start_time + clip.length;

                if current_beat >= clip_start && current_beat < clip_end {
                    let position_in_clip = current_beat - clip_start;

                    match &clip.clip_type {
                        crate::models::ClipType::Pattern(pattern_idx) => {
                            let step_in_pattern = ((position_in_clip * 4.0) as usize) % 16;
                            let last_step = (((current_beat - 1.0 / samples_per_beat as f64) - clip_start) * 4.0) as usize % 16;

                            if step_in_pattern != last_step {
                                if let Some(pattern) = patterns.get(*pattern_idx) {
                                    for (i, row) in pattern.data.iter().enumerate() {
                                        if row[step_in_pattern] {
                                            triggers.push((i, step_in_pattern));
                                        }
                                    }
                                }
                            }
                        }
                        crate::models::ClipType::AudioFile(instrument_idx) => {
                            if position_in_clip < 1.0 / samples_per_beat as f64 {
                                audio_triggers.push(*instrument_idx);
                            }
                        }
                    }
                }
            }

            // Apply triggers
            for (instrument_idx, _) in triggers {
                if let Some(instrument) = state.instruments.get_mut(instrument_idx) {
                    instrument.is_playing = true;
                    instrument.position = 0;
                }
            }

            for instrument_idx in audio_triggers {
                if let Some(instrument) = state.instruments.get_mut(instrument_idx) {
                    if !instrument.is_playing {
                        instrument.is_playing = true;
                        instrument.position = 0;
                    }
                }
            }

            // Metronome
            if state.is_metronome {
                let beat = state.playhead_position.floor() as usize;
                let last_beat = (state.playhead_position - 1.0 / samples_per_beat as f64).floor() as usize;
                if beat != last_beat {
                    state.metronome_playing = true;
                    state.metronome_position = 0;
                }
            }

            state.metronome_counter += 1.0;
            state.playhead_position = (state.metronome_counter / samples_per_beat) as f64;
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

        if state.metronome_playing {
            if state.metronome_position < state.metronome_sample.len() {
                mix += state.metronome_sample[state.metronome_position];
                state.metronome_position += 1;
            } else {
                state.metronome_playing = false;
            }
        }

        if let Some(ref mut preview) = state.preview_sound {
            if preview.is_playing {
                if preview.position < preview.samples.len() {
                    mix += preview.samples[preview.position];
                    preview.position += 1;
                } else {
                    preview.is_playing = false;
                    state.preview_sound = None;
                }
            }
        }

        for sample in frame.iter_mut() {
            *sample = mix;
        }
    }
}