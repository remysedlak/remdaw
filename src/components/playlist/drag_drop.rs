// src/components/playlist/drag_drop.rs

use std::path::PathBuf;
use egui::{Context, Pos2, Rect, Id};
use crate::models::{MyApp, PlacedClip, ClipType, Instrument};
use crate::audio::path_to_vector;
use super::config::PlaylistConfig;

pub fn handle_pattern_drop(
    app: &mut MyApp,
    ctx: &Context,
    pointer_pos: Pos2,
    rect: Rect,
    config: &PlaylistConfig,
) {
    if let Some(pattern_idx) = ctx.memory(|mem| {
        mem.data.get_temp::<usize>(Id::new("dragging_pattern"))
    }) {
        if rect.contains(pointer_pos) {
            let timeline_start_x = rect.left() + config.track_label_width;
            let tracks_start_y = rect.top() + config.timeline_header_height;

            let relative_y = pointer_pos.y - tracks_start_y;
            let track_idx = (relative_y / config.track_height).floor() as usize;

            let relative_x = pointer_pos.x - timeline_start_x;
            let start_beat = (relative_x / config.pixels_per_beat).max(0.0).round();

            let mut state = app.audio_state.lock().unwrap();
            if track_idx < state.playlist.tracks.len() {
                let name = state.patterns.get(pattern_idx)
                    .map(|p| p.name.clone())
                    .unwrap_or_else(|| "Unknown".to_string());

                state.playlist.clips.push(PlacedClip {
                    clip_type: ClipType::Pattern(pattern_idx),
                    track_index: track_idx,
                    start_time: start_beat as f64,
                    name,
                    length: config.preview_default_length as f64,
                    color: config.pattern_clip_color,
                });
            }
        }

        ctx.memory_mut(|mem| {
            mem.data.remove::<usize>(Id::new("dragging_pattern"));
        });
    }
}

pub fn handle_audio_drop(
    app: &mut MyApp,
    ctx: &Context,
    pointer_pos: Pos2,
    rect: Rect,
    config: &PlaylistConfig,
) {
    if let Some(file_path) = ctx.memory(|mem| {
        mem.data.get_temp::<PathBuf>(Id::new("dragging_audio_file"))
    }) {
        if rect.contains(pointer_pos) {
            let timeline_start_x = rect.left() + config.track_label_width;
            let tracks_start_y = rect.top() + config.timeline_header_height;

            let relative_y = pointer_pos.y - tracks_start_y;
            let track_idx = (relative_y / config.track_height).floor() as usize;

            let relative_x = pointer_pos.x - timeline_start_x;
            let start_beat = (relative_x / config.pixels_per_beat).max(0.0).round();

            let mut state = app.audio_state.lock().unwrap();
            if track_idx < state.playlist.tracks.len() {
                let samples = path_to_vector(file_path.to_str().unwrap());
                let name = file_path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Unknown")
                    .to_string();

                state.instruments.push(Instrument {
                    name: name.clone(),
                    file_path: file_path.clone(),
                    samples,
                    position: 0,
                    is_playing: false,
                });
                state.pattern.push(vec![false; 16]);

                let instrument_idx = state.instruments.len() - 1;

                state.playlist.clips.push(PlacedClip {
                    clip_type: ClipType::AudioFile(instrument_idx),
                    track_index: track_idx,
                    start_time: start_beat as f64,
                    name,
                    length: config.preview_default_length as f64,
                    color: config.audio_clip_color,
                });
            }
        }

        ctx.memory_mut(|mem| {
            mem.data.remove::<PathBuf>(Id::new("dragging_audio_file"));
        });
    }
}