// src/components/playlist/config.rs

use eframe::epaint::Color32;

/// Visual configuration for the playlist
pub struct PlaylistConfig {
    // Layout
    pub track_label_width: f32,
    pub timeline_header_height: f32,
    pub track_height: f32,
    pub clip_vertical_padding: f32,

    // Grid
    pub pixels_per_beat: f32,
    pub beats_per_bar: i32,

    // Resize
    pub edge_grab_distance: f32,
    pub min_clip_length: f64,

    // Preview
    pub preview_dash_length: f32,
    pub preview_gap_length: f32,
    pub preview_default_length: f32,

    // Colors - Timeline
    pub timeline_bg: Color32,
    pub bar_line_color: Color32,
    pub beat_line_color: Color32,
    pub bar_text_color: Color32,

    // Colors - Tracks
    pub track_even_bg: Color32,
    pub track_odd_bg: Color32,
    pub track_text_color: Color32,
    pub mute_button_color: Color32,
    pub mute_button_outline: Color32,

    // Colors - Clips
    pub pattern_clip_color: Color32,
    pub audio_clip_color: Color32,
    pub clip_text_color: Color32,
    pub clip_corner_radius: f32,

    // Colors - Playhead
    pub playhead_color: Color32,
    pub playhead_width: f32,

    // Colors - Preview
    pub pattern_preview_color: Color32,
    pub audio_preview_color: Color32,
    pub preview_fill_alpha: u8,
    pub preview_outline_alpha: u8,
}

impl Default for PlaylistConfig {
    fn default() -> Self {
        Self {
            // Layout
            track_label_width: 150.0,
            timeline_header_height: 50.0,
            track_height: 60.0,
            clip_vertical_padding: 5.0,

            // Grid
            pixels_per_beat: 100.0,
            beats_per_bar: 4,

            // Resize
            edge_grab_distance: 8.0,
            min_clip_length: 0.25,

            // Preview
            preview_dash_length: 8.0,
            preview_gap_length: 4.0,
            preview_default_length: 4.0,

            // Colors - Timeline
            timeline_bg: Color32::from_gray(30),
            bar_line_color: Color32::WHITE,
            beat_line_color: Color32::from_gray(120),
            bar_text_color: Color32::WHITE,

            // Colors - Tracks
            track_even_bg: Color32::from_gray(40),
            track_odd_bg: Color32::from_gray(50),
            track_text_color: Color32::WHITE,
            mute_button_color: Color32::from_rgb(155, 0, 0),
            mute_button_outline: Color32::WHITE,

            // Colors - Clips
            pattern_clip_color: Color32::from_rgb(80, 120, 200),
            audio_clip_color: Color32::from_rgb(200, 120, 80),
            clip_text_color: Color32::WHITE,
            clip_corner_radius: 5.0,

            // Colors - Playhead
            playhead_color: Color32::RED,
            playhead_width: 3.0,

            // Colors - Preview
            pattern_preview_color: Color32::from_rgb(80, 120, 200),
            audio_preview_color: Color32::from_rgb(200, 120, 80),
            preview_fill_alpha: 40,
            preview_outline_alpha: 180,
        }
    }
}