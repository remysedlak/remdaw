use std::sync::{Arc, Mutex};
use cpal::{Stream};
use crate::audio;
use crate::audio::path_to_vector;

pub struct Instrument {
    pub is_playing: bool,
    pub position: usize,  // where we are in the sample
    pub samples: Vec<f32>, // the actual WAV data
}
pub struct MyApp {
    pub audio_stream: Stream,
    pub audio_state: Arc<Mutex<AudioState>>,
    pub is_channel_rack_open: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        let (audio_stream, audio_state) = audio::init();
        Self {
            audio_stream,
            audio_state,
            is_channel_rack_open: true,
        }
    }
}

pub struct AudioState {
    pub instruments: Vec<Instrument>,
    pub bpm: i16,
    pub sampling_rate: f32,
    pub samples_per_beat: f32,
    pub metronome_counter: f32,
    pub is_playing: bool,
    pub is_metronome: bool,
}

impl AudioState {
    pub fn new(sampling_rate: f32) -> Self {
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

        instruments.push(Instrument {
            samples: path_to_vector("instruments/Boss DR-660/Rim/St 808.wav"),
            position: 0,
            is_playing: false,
        });
        let samples_per_beat =  sampling_rate * 60.0 / 130.0 ;
        AudioState {
            instruments,
            bpm: 130,
            sampling_rate,
            samples_per_beat,
            metronome_counter: 0.0,
            is_playing: false,
            is_metronome: false,
        }
    }
}