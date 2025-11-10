use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use cpal::{Stream};
use egui::Color32;
use crate::audio;
use crate::audio::path_to_vector;
use crate::config::AppConfig;


#[derive(Clone)]
pub enum ResizeEdge {
    Left,
    Right,
}

#[derive(Clone)]
pub struct ResizeState {
    pub clip_index: usize,
    pub edge: ResizeEdge,
    pub initial_start: f64,
    pub initial_length: f64,
}

// Where all music positions are stored for playback and export
pub struct Playlist {
    pub(crate) tracks: Vec<Track>,
    pub(crate) clips: Vec<PlacedClip>,
    zoom_level: f32,
    scroll_position: f32,
}

// one group of patterns of drums from channel rack
#[derive(Clone)]
pub struct Pattern {
    pub name: String,
    pub data: Vec<Vec<bool>>,  // The actual 16-step pattern for each instrument
}

// In models.rs
#[derive(Clone)]
pub enum ClipType {
    Pattern(usize), // Index into patterns vec
    AudioFile(usize), // Index into instruments vec
}

#[derive(Clone)]
pub struct PlacedClip {
    pub clip_type: ClipType,
    pub name: String,
    pub track_index: usize,
    pub start_time: f64, // in beats
    pub length: f64,
    pub color: Color32,
}

pub struct Track {
    pub(crate) name: String,
    pub(crate) height: f32,
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
    pub selected_file: Option<PathBuf>,
    pub config: AppConfig,
    pub ui_state: UiState,
}

pub struct UiState {
    pub snap_to_grid: bool,        // Add this
    pub snap_division: f32,
    pub resizing_clip: Option<ResizeState>,
    pub playlist_height: f32,
    pub is_channel_rack_open: bool,
    pub is_settings_open: bool,
    pub is_file_info_open: bool,
    pub is_files_explorer_open: bool,
    pub pattern_rename_popup: Option<usize>, // Changed from bool to Option<usize>
    pub rename_buffer: String, // Store the temporary name
    pub is_pattern_delete: bool,
    pub is_patterns_open: bool,
}

// shared state between gui and cpal
pub struct AudioState {
    pub current_pattern_index: Option<usize>,
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
    pub just_started: bool,
    pub playlist: Playlist,
    pub playhead_position: f64,
    pub patterns: Vec<Pattern>,
}

impl AudioState {
    pub fn new(sampling_rate: f32) -> Self {
        let mut instruments = Vec::new();
        let paths = ["test_instruments/cowbell.wav", "test_instruments/Clap Dance.wav", "test_instruments/St 808.wav"];
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

        let metronome_sample = path_to_vector("test_instruments/St 808.wav");
        let mut patterns = Vec::new();
        patterns.push(Pattern { name:"Pattern 1".to_string(), data: pattern.clone() } );
        AudioState {
            just_started: false,
            current_pattern_index: Some(0),
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
            patterns
        }
    }
}

impl Default for MyApp {
    fn default() -> Self {
        let ui_state = UiState {
            snap_to_grid: false,
            snap_division: 1.0, // 1.0 = bar, 0.25 = beat, 0.0625 = 16th note)
            is_channel_rack_open: false,
            playlist_height: 300.0,
            is_settings_open: false,
            is_patterns_open: true,
            pattern_rename_popup: None,
            is_files_explorer_open: true,
            resizing_clip: None,
            is_file_info_open: false, rename_buffer: String::new(), is_pattern_delete: false };

        let (_audio_stream, audio_state) = audio::init();
        Self {
            _audio_stream,
            audio_state,
            ui_state,
            config: AppConfig::load(),
            selected_file: None,
        }
    }
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
                    height: 50.0,
                    muted: false,
                    solo: false,
                },
                Track {
                    name: "Track 3".to_string(),
                    height: 50.0,
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