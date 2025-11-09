use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use cpal::{Stream};
use egui::Color32;
use crate::audio;
use crate::audio::path_to_vector;
use crate::config::AppConfig;

pub struct Playlist {
    tracks: Vec<Track>,
    clips: Vec<PlacedClip>,
    zoom_level: f32,
    scroll_position: f32,
}

impl Playlist {
    pub fn new() -> Self {
        Playlist {
            tracks: vec![
                Track {
                    name: "Track 1".to_string(),
                    height: 60.0,
                    muted: false,
                    solo: false,
                },
                Track {
                    name: "Track 2".to_string(),
                    height: 60.0,
                    muted: false,
                    solo: false,
                },
                Track {
                    name: "Track 3".to_string(),
                    height: 60.0,
                    muted: false,
                    solo: false,
                },
                Track {
                    name: "Track 4".to_string(),
                    height: 60.0,
                    muted: false,
                    solo: false,
                },
            ],
            clips: Vec::new(),  // Empty - user will add clips
            zoom_level: 1.0,
            scroll_position: 0.0,
        }
    }
}

struct PlacedClip {
    pattern_id: usize, // reference to your pattern/sample
    track_index: usize,
    start_time: f64, // in beats or samples
    length: f64,
    color: Color32,
}

struct Track {
    name: String,
    height: f32,
    muted: bool,
    solo: bool,
}

// loaded sounds
pub struct Instrument {
    pub is_playing: bool,
    pub position: usize,  // where we are in the sample
    pub samples: Vec<f32>, // the actual WAV data
    pub name: String,
    pub file_path: PathBuf,
}

// app config
pub struct MyApp {
    pub _audio_stream: Stream,
    pub audio_state: Arc<Mutex<AudioState>>,
    pub is_channel_rack_open: bool,
    pub is_settings_open: bool,
    pub is_file_info_open: bool,
    pub is_files_explorer_open: bool,
    pub is_patterns_open: bool,
    pub selected_file: Option<PathBuf>,
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
            is_patterns_open: true,
            is_files_explorer_open: true,
            config: AppConfig::load(),
            is_file_info_open: false,
            selected_file: None,
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
    pub metronome_sample: Vec<f32>,  // Just the audio data
    pub metronome_position: usize,
    pub metronome_playing: bool,
    pub preview_sound: Option<Instrument>,

    pub playlist: Playlist,
    pub playhead_position: f64,
}

impl AudioState {
    pub fn new(sampling_rate: f32) -> Self {
        let mut instruments = Vec::new();
        let paths = ["instruments/cowbell.wav", "instruments/Boss DR-660/Clap/Clap Dance.wav", "instruments/Boss DR-660/Rim/St 808.wav"];
        for path in paths.iter() {

            instruments.push(Instrument {
                file_path: path.parse().unwrap(),
                samples: path_to_vector(path),
                position: 0,
                is_playing: false,
                name: Path::new(path).file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Unknown").parse().unwrap()
            })
        }

        let samples_per_beat =  sampling_rate * 60.0 / 130.0 ;

        // Initialize pattern: one row per instrument, 16 steps each
        let num_instruments = instruments.len();
        let pattern = vec![vec![false; 16]; num_instruments];

        let metronome_sample = path_to_vector("instruments/Boss DR-660/Rim/St 808.wav");

        AudioState {
            instruments,
            bpm: 130,
            sampling_rate,
            samples_per_beat,
            metronome_counter: 0.0,
            is_playing: false,
            is_metronome: false,
            pattern,
            current_step: 0,
            metronome_sample,
            metronome_playing: false,
            metronome_position: 0,
            preview_sound: None,
            playlist: Playlist::new(),
            playhead_position: 0.0,
        }
    }
}