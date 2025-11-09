// components/playlist.rs
use crate::models::MyApp;

pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Playlist");

        let state = app.audio_state.lock().unwrap();
        let playlist = &state.playlist;

        // Draw timeline grid
        let (response, painter) = ui.allocate_painter(
            egui::Vec2::new(ui.available_width(), 400.0),
            egui::Sense::click_and_drag()
        );

        let rect = response.rect;

        // Draw tracks
        for (idx, track) in playlist.tracks.iter().enumerate() {
            let y = rect.top() + idx as f32 * track.height;

            // Track background
            painter.rect_filled(
                egui::Rect::from_min_size(
                    egui::pos2(rect.left(), y),
                    egui::vec2(rect.width(), track.height)
                ),
                0.0,
                if idx % 2 == 0 {
                    egui::Color32::from_gray(40)
                } else {
                    egui::Color32::from_gray(50)
                }
            );

            // Track name
            painter.text(
                egui::pos2(rect.left() + 5.0, y + track.height / 2.0),
                egui::Align2::LEFT_CENTER,
                &track.name,
                egui::FontId::default(),
                egui::Color32::WHITE
            );
        }

        // Draw clips
        for clip in &playlist.clips {
            let track = &playlist.tracks[clip.track_index];
            let y = rect.top() + clip.track_index as f32 * track.height;

            // Convert time to pixels (simple: 100px per beat)
            let x = rect.left() + 150.0 + (clip.start_time as f32 * 100.0);
            let width = clip.length as f32 * 100.0;

            // Draw clip rectangle
            painter.rect_filled(
                egui::Rect::from_min_size(
                    egui::pos2(x, y + 5.0),
                    egui::vec2(width, track.height - 10.0)
                ),
                5.0,
                clip.color
            );

            // Clip label
            painter.text(
                egui::pos2(x + 5.0, y + track.height / 2.0),
                egui::Align2::LEFT_CENTER,
                format!("Pattern {}", clip.pattern_id),
                egui::FontId::default(),
                egui::Color32::WHITE
            );
        }

        // Draw playhead
        let playhead_x = rect.left() + 150.0 + (state.playhead_position as f32 * 100.0);
        painter.vline(
            playhead_x,
            rect.top()..=rect.bottom(),
            egui::Stroke::new(2.0, egui::Color32::RED)
        );
    });
}