// src/components/playlist/drawing.rs

use eframe::epaint::{Color32, Stroke, Rect, Pos2, FontId, Vec2};
use egui::{Align2, Painter};
use crate::models::{Playlist};
use super::config::PlaylistConfig;

pub fn draw_timeline_header(
    painter: &Painter,
    rect: Rect,
    config: &PlaylistConfig,
) {
    let timeline_start_x = rect.left() + config.track_label_width;

    painter.rect_filled(
        Rect::from_min_size(
            Pos2::new(timeline_start_x, rect.top()),
            Vec2::new(rect.width() - config.track_label_width, config.timeline_header_height)
        ),
        0.0,
        config.timeline_bg
    );
}

pub fn draw_beat_markers(
    painter: &Painter,
    rect: Rect,
    config: &PlaylistConfig,
    max_beats: i32,
) {
    let timeline_start_x = rect.left() + config.track_label_width;

    for beat in 0..max_beats {
        let x = timeline_start_x + (beat as f32 * config.pixels_per_beat);

        if x > rect.right() {
            break;
        }

        let is_bar = beat % config.beats_per_bar == 0;
        let tick_height = if is_bar { 30.0 } else { 15.0 };
        let color = if is_bar { config.bar_line_color } else { config.beat_line_color };

        painter.line_segment(
            [Pos2::new(x, rect.top()), Pos2::new(x, rect.top() + tick_height)],
            Stroke::new(if is_bar { 2.0 } else { 1.0 }, color)
        );

        if is_bar {
            let bar_number = (beat / config.beats_per_bar) + 1;
            painter.text(
                Pos2::new(x + 3.0, rect.top() + 2.0),
                Align2::LEFT_TOP,
                format!("{}", bar_number),
                FontId::proportional(14.0),
                config.bar_text_color
            );
        }
    }
}

pub fn draw_tracks(
    painter: &Painter,
    rect: Rect,
    playlist: &Playlist,
    config: &PlaylistConfig,
) {
    let tracks_start_y = rect.top() + config.timeline_header_height;

    for (idx, track) in playlist.tracks.iter().enumerate() {
        let y = tracks_start_y + idx as f32 * config.track_height;

        painter.rect_filled(
            Rect::from_min_size(
                Pos2::new(rect.left(), y),
                Vec2::new(rect.width(), config.track_height)
            ),
            0.0,
            if idx % 2 == 0 { config.track_even_bg } else { config.track_odd_bg }
        );

        // Mute button
        painter.circle(
            Pos2::new(rect.left() + 15.0, y + config.track_height / 2.0),
            10.0,
            config.mute_button_color,
            Stroke::new(1.0, config.mute_button_outline)
        );

        painter.text(
            Pos2::new(rect.left() + 30.0, y + config.track_height / 2.0),
            Align2::LEFT_CENTER,
            &track.name,
            FontId::default(),
            config.track_text_color
        );
    }
}

pub fn draw_clips(
    painter: &Painter,
    rect: Rect,
    playlist: &Playlist,
    config: &PlaylistConfig,
) {
    let timeline_start_x = rect.left() + config.track_label_width;
    let tracks_start_y = rect.top() + config.timeline_header_height;

    for clip in &playlist.clips {
        let y = tracks_start_y + clip.track_index as f32 * config.track_height;
        let x = timeline_start_x + (clip.start_time as f32 * config.pixels_per_beat);
        let width = clip.length as f32 * config.pixels_per_beat;

        let clip_rect = Rect::from_min_size(
            Pos2::new(x, y + config.clip_vertical_padding),
            Vec2::new(width, config.track_height - config.clip_vertical_padding * 2.0)
        );

        painter.rect_filled(clip_rect, config.clip_corner_radius, clip.color);

        painter.text(
            Pos2::new(x + 5.0, y + config.track_height / 2.0),
            Align2::LEFT_CENTER,
            &clip.name,
            FontId::default(),
            config.clip_text_color
        );
    }
}

pub fn draw_playhead(
    painter: &Painter,
    rect: Rect,
    playhead_position: f64,
    config: &PlaylistConfig,
) {
    let timeline_start_x = rect.left() + config.track_label_width;
    let playhead_x = timeline_start_x + (playhead_position as f32 * config.pixels_per_beat);

    painter.vline(
        playhead_x,
        rect.top()..=rect.bottom(),
        Stroke::new(config.playhead_width, config.playhead_color)
    );
}

pub fn draw_dashed_rect(
    painter: &Painter,
    rect: Rect,
    color: Color32,
    config: &PlaylistConfig,
) {
    let total_length = config.preview_dash_length + config.preview_gap_length;

    // Top edge
    let mut x = rect.left();
    while x < rect.right() {
        let end_x = (x + config.preview_dash_length).min(rect.right());
        painter.line_segment(
            [Pos2::new(x, rect.top()), Pos2::new(end_x, rect.top())],
            Stroke::new(2.0, color)
        );
        x += total_length;
    }

    // Bottom edge
    let mut x = rect.left();
    while x < rect.right() {
        let end_x = (x + config.preview_dash_length).min(rect.right());
        painter.line_segment(
            [Pos2::new(x, rect.bottom()), Pos2::new(end_x, rect.bottom())],
            Stroke::new(2.0, color)
        );
        x += total_length;
    }

    // Left edge
    let mut y = rect.top();
    while y < rect.bottom() {
        let end_y = (y + config.preview_dash_length).min(rect.bottom());
        painter.line_segment(
            [Pos2::new(rect.left(), y), Pos2::new(rect.left(), end_y)],
            Stroke::new(2.0, color)
        );
        y += total_length;
    }

    // Right edge
    let mut y = rect.top();
    while y < rect.bottom() {
        let end_y = (y + config.preview_dash_length).min(rect.bottom());
        painter.line_segment(
            [Pos2::new(rect.right(), y), Pos2::new(rect.right(), end_y)],
            Stroke::new(2.0, color)
        );
        y += total_length;
    }
}