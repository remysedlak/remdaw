use crate::models::{AudioState};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, Stream};
use hound;
use std::sync::{Arc, Mutex};

/// Loads a WAV file from disk and converts it to a vector of f32 samples
/// normalized to the range [-1.0, 1.0]
///
/// # Arguments
/// * `instrument_path` - File path to the WAV file
///
/// # Returns
/// * `Vec<f32>` - Vector of normalized audio samples
pub fn path_to_vector(instrument_path: &str) -> Vec<f32> {
    // Open the WAV file using the hound library
    let mut reader = match hound::WavReader::open(instrument_path) {
        Err(err) => panic!("{}", err),
        Ok(result) => result,
    };

    // Read all samples as i16 (16-bit audio)
    let samples = reader.samples::<i16>();

    // Convert i16 samples to f32 normalized values
    let vector: Vec<f32> = samples
        .map(|result| result.unwrap()) // Unwrap each Result<i16>
        .map(|i16_value| i16_value as f32 / i16::MAX as f32) // Normalize to [-1.0, 1.0]
        .collect();
    vector
}

/// Initializes the audio system by setting up the output device and audio stream
///
/// # Returns
/// * `(Stream, Arc<Mutex<AudioState>>)` - The audio stream and shared audio state
pub fn init() -> (Stream, Arc<Mutex<AudioState>>) {
    // Get the default audio host (OS audio system)
    let host = cpal::default_host();

    // Get the default output device (speakers/headphones)
    let device = host
        .default_output_device()
        .expect("no output device available");

    // Get the default audio configuration for this device
    let supported_config = device
        .default_output_config()
        .expect("error getting default config");

    let config = supported_config.config();
    let sample_format = supported_config.sample_format();
    let sample_rate = config.sample_rate.0 as f32; // e.g., 48000 Hz

    // Create shared audio state with the device's sample rate
    let audio_state = Arc::new(Mutex::new(AudioState::new(sample_rate)));
    let audio_state_clone = audio_state.clone();

    // Error callback for the audio stream
    let err_fn = |err| eprintln!("error: {}", err);

    // Build the output stream with F32 sample format
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

    // Start the audio stream
    stream.play().unwrap();
    (stream, audio_state)
}

/// Audio callback function that gets called continuously by the audio system
/// Fills the output buffer with mixed audio from all active sources
///
/// # Arguments
/// * `data` - Output buffer to fill with audio samples (stereo interleaved)
/// * `state` - Shared audio state containing all instruments, patterns, and playback info
fn play_instrument(data: &mut [f32], state: &Arc<Mutex<AudioState>>) {
    // Lock the audio state for this callback
    let mut state = state.lock().unwrap();

    // Stereo audio (left and right channels)
    let channels = 2;

    // Process each stereo frame (2 samples: left + right)
    for frame in data.chunks_mut(channels) {

        // Only process audio if playback is active
        if state.is_playing {

            // Get current playback position in beats
            let current_beat = state.playhead_position;
            let samples_per_beat = state.samples_per_beat;

            // Clone data to avoid borrow checker issues
            // (we need to mutate instruments while reading clips/patterns)
            let clips = state.playlist.clips.clone();
            let patterns = state.patterns.clone();

            // Vectors to store which sounds should be triggered this frame
            let mut triggers: Vec<(usize, usize)> = Vec::new(); // (instrument_idx, step)
            let mut audio_triggers: Vec<usize> = Vec::new(); // instrument_idx for audio files

            // Check all clips in the playlist
            for clip in &clips {
                let clip_start = clip.start_time;
                let clip_end = clip.start_time + clip.length;

                // Only process clips that are currently playing
                if current_beat >= clip_start && current_beat < clip_end {
                    // Calculate position within this clip (in beats)
                    let position_in_clip = current_beat - clip_start;

                    match &clip.clip_type {
                        // Pattern clips: trigger instruments based on 16-step sequencer
                        crate::models::ClipType::Pattern(pattern_idx) => {
                            // Convert beat position to step (0-15)
                            // Multiply by 4 because each beat = 4 steps in a 16-step pattern
                            let step_in_pattern = ((position_in_clip * 4.0) as usize) % 16;

                            // Get the previous step to detect step changes
                            let last_step = (((current_beat - 1.0 / samples_per_beat as f64) - clip_start) * 4.0) as usize % 16;

                            // Only trigger on step boundaries (when step changes)
                            if step_in_pattern != last_step {
                                if let Some(pattern) = patterns.get(*pattern_idx) {
                                    // Check each instrument row in the pattern
                                    for (i, row) in pattern.data.iter().enumerate() {
                                        // If this step is active, trigger the instrument
                                        if row[step_in_pattern] {
                                            triggers.push((i, step_in_pattern));
                                        }
                                    }
                                }
                            }
                        }

                        // Audio file clips: trigger the audio file to play
                        crate::models::ClipType::AudioFile(instrument_idx) => {
                            // Only trigger at the very start of the clip
                            if position_in_clip < 1.0 / samples_per_beat as f64 {
                                audio_triggers.push(*instrument_idx);
                            }
                        }
                    }
                }
            }

            // Apply all pattern triggers: start playing instruments
            for (instrument_idx, _) in triggers {
                if let Some(instrument) = state.instruments.get_mut(instrument_idx) {
                    instrument.is_playing = true;
                    instrument.position = 0; // Reset to start of sample
                }
            }

            // Apply all audio file triggers
            for instrument_idx in audio_triggers {
                if let Some(instrument) = state.instruments.get_mut(instrument_idx) {
                    // Only start if not already playing (prevents retriggering)
                    if !instrument.is_playing {
                        instrument.is_playing = true;
                        instrument.position = 0;
                    }
                }
            }

            // Handle metronome click on beat boundaries
            if state.is_metronome {
                let beat = state.playhead_position.floor() as usize;
                let last_beat = (state.playhead_position - 1.0 / samples_per_beat as f64).floor() as usize;

                // Trigger metronome when crossing a beat boundary
                if beat != last_beat {
                    state.metronome_playing = true;
                    state.metronome_position = 0;
                }
            }

            // Advance the playhead
            state.metronome_counter += 1.0; // Increment sample counter
            state.playhead_position = (state.metronome_counter / samples_per_beat) as f64; // Convert to beats
        }

        // Mix all active audio sources together
        let mut mix = 0.0;

        // Mix all instruments
        for instrument in &mut state.instruments {
            if instrument.is_playing {
                // If we haven't reached the end of the sample
                if instrument.position < instrument.samples.len() {
                    mix += instrument.samples[instrument.position]; // Add to mix
                    instrument.position += 1; // Advance playback position
                } else {
                    // Sample finished playing
                    instrument.is_playing = false;
                }
            }
        }

        // Mix metronome
        if state.metronome_playing {
            if state.metronome_position < state.metronome_sample.len() {
                mix += state.metronome_sample[state.metronome_position];
                state.metronome_position += 1;
            } else {
                state.metronome_playing = false;
            }
        }

        // Mix preview sound (file browser preview)
        if let Some(ref mut preview) = state.preview_sound {
            if preview.is_playing {
                if preview.position < preview.samples.len() {
                    mix += preview.samples[preview.position];
                    preview.position += 1;
                } else {
                    // Preview finished, remove it
                    preview.is_playing = false;
                    state.preview_sound = None;
                }
            }
        }

        // Write the mixed audio to both channels (stereo)
        for sample in frame.iter_mut() {
            *sample = mix;
        }
    }
}