use std::sync::{Arc, Mutex};
use cpal::Stream;

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

pub struct AudioState {
    pub instruments: Vec<Instrument>,
}