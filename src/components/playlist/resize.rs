// src/components/playlist/resize.rs

use egui::{Context, Pos2, Rect, Vec2};
use crate::models::{MyApp, ResizeEdge, ResizeState, ClipType};
use super::config::PlaylistConfig;

/// Check if mouse is hovering near a clip edge (for resize detection)
pub fn detect_resize_hover(
    pointer_pos: Option<Pos2>,
    app: &MyApp,
    rect: Rect,
    config: &PlaylistConfig,
) -> Option<(usize, ResizeEdge)> {
    let pointer_pos = pointer_pos?;

    let state = app.audio_state.lock().unwrap();
    let timeline_start_x = rect.left() + config.track_label_width;
    let tracks_start_y = rect.top() + config.timeline_header_height;

    for (clip_idx, clip) in state.playlist.clips.iter().enumerate() {
        // Only patterns are resizable
        if !matches!(clip.clip_type, ClipType::Pattern(_)) {
            continue;
        }

        let y = tracks_start_y + clip.track_index as f32 * config.track_height;
        let x = timeline_start_x + (clip.start_time as f32 * config.pixels_per_beat);
        let width = clip.length as f32 * config.pixels_per_beat;

        let clip_rect = Rect::from_min_size(
            Pos2::new(x, y + config.clip_vertical_padding),
            Vec2::new(width, config.track_height - config.clip_vertical_padding * 2.0)
        );

        if clip_rect.contains(pointer_pos) {
            let dist_to_left = (pointer_pos.x - clip_rect.left()).abs();
            let dist_to_right = (pointer_pos.x - clip_rect.right()).abs();

            if dist_to_left < config.edge_grab_distance {
                return Some((clip_idx, ResizeEdge::Left));
            } else if dist_to_right < config.edge_grab_distance {
                return Some((clip_idx, ResizeEdge::Right));
            }
        }
    }

    None
}

/// Start a resize operation
pub fn start_resize(
    app: &mut MyApp,
    clip_idx: usize,
    edge: ResizeEdge,
) {
    let state = app.audio_state.lock().unwrap();
    let clip = &state.playlist.clips[clip_idx];

    app.ui_state.resizing_clip = Some(ResizeState {
        clip_index: clip_idx,
        edge,
        initial_start: clip.start_time,
        initial_length: clip.length,
    });
}

/// Perform the resize operation (called while dragging)
pub fn perform_resize(
    app: &mut MyApp,
    drag_delta: Vec2,
    config: &PlaylistConfig,
) {
    let resize_state = match &app.ui_state.resizing_clip {
        Some(state) => state.clone(),
        None => return,
    };

    if drag_delta.x.abs() < 0.01 {
        return; // No movement
    }

    let delta_beats = drag_delta.x / config.pixels_per_beat;
    let mut state = app.audio_state.lock().unwrap();
    let clip = &mut state.playlist.clips[resize_state.clip_index];

    match resize_state.edge {
        ResizeEdge::Left => {
            let new_start = clip.start_time + delta_beats as f64;
            let new_length = clip.length - delta_beats as f64;

            if new_length > config.min_clip_length && new_start >= 0.0 {
                if app.ui_state.snap_to_grid {
                    let snap_div = app.ui_state.snap_division as f64;
                    clip.start_time = (new_start / snap_div).round() * snap_div;
                    clip.length = (new_length / snap_div).round() * snap_div;
                } else {
                    clip.start_time = new_start;
                    clip.length = new_length;
                }
            }
        }
        ResizeEdge::Right => {
            let new_length = clip.length + delta_beats as f64;

            if new_length > config.min_clip_length {
                if app.ui_state.snap_to_grid {
                    let snap_div = app.ui_state.snap_division as f64;
                    clip.length = (new_length / snap_div).round() * snap_div;
                } else {
                    clip.length = new_length;
                }
            }
        }
    }
}

/// End the resize operation
pub fn end_resize(app: &mut MyApp) {
    app.ui_state.resizing_clip = None;
}