use std::sync::{Arc, Mutex};
use cpal::{Stream};
use crate::audio;
use crate::audio::path_to_vector;
use crate::config::AppConfig;

// small pieces of audio within a track that can be moved around
pub struct Clip {
    pub samples: Vec<f32>,
}

// one track = one horizontal line of music in a DAW , usually holds one pattern or one file
pub struct Track {
    pub clips: Vec<Clip>,
}

// loaded sounds
pub struct Instrument {
    pub is_playing: bool,
    pub position: usize,  // where we are in the sample
    pub samples: Vec<f32>, // the actual WAV data
    pub name: String,
}

// app config
pub struct MyApp {
    pub _audio_stream: Stream,
    pub audio_state: Arc<Mutex<AudioState>>,
    pub is_channel_rack_open: bool,
    pub is_settings_open: bool,
    pub config: AppConfig,
}

impl Default for MyApp {
    fn default() -> Self {
        let (_audio_stream, audio_state) = audio::init();
        Self {
            _audio_stream,
            audio_state,
            is_channel_rack_open: false,
            is_settings_open: false,
            config: AppConfig::load(),
        }
    }
}

// shared state between gui and cpal
pub struct AudioState {
    pub instruments: Vec<Instrument>,
    pub bpm: i16,
    pub sampling_rate: f32,
    pub samples_per_beat: f32,
    pub metronome_counter: f32,
    pub is_playing: bool,
    pub is_metronome: bool,
    pub pattern: Vec<Vec<bool>>,
    pub current_step: usize,
}

impl AudioState {
    pub fn new(sampling_rate: f32) -> Self {
        let mut instruments = Vec::new();

        instruments.push(Instrument {
            samples: path_to_vector("instruments/cowbell.wav"),
            position: 0,
            is_playing: false,
            name: "cowbell.wav".to_string(),
        });

        instruments.push(Instrument {
            samples: path_to_vector("instruments/Boss DR-660/Clap/Clap Dance.wav"),
            position: 0,
            is_playing: false,
            name: "Clap Dance.wav".to_string(),
        });

        instruments.push(Instrument {
            samples: path_to_vector("instruments/Boss DR-660/Rim/St 808.wav"),
            position: 0,
            is_playing: false,
            name: "St 808.wav".to_string(),
        });
        let samples_per_beat =  sampling_rate * 60.0 / 130.0 ;

        // Initialize pattern: one row per instrument, 16 steps each
        let num_instruments = instruments.len();
        let pattern = vec![vec![false; 16]; num_instruments];

        AudioState {
            instruments,
            bpm: 130,
            sampling_rate,
            samples_per_beat,
            metronome_counter: 0.0,
            is_playing: false,
            is_metronome: false,
            pattern,
            current_step: 0
        }
    }
}