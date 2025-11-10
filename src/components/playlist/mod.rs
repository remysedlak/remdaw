// src/components/playlist/mod.rs

mod config;
mod drawing;
mod drag_drop;
mod resize;
mod input;

use eframe::emath::Align::Center;
use crate::models::MyApp;
use crate::components::snap_to_grid;

pub use config::PlaylistConfig;

pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    let config = PlaylistConfig::default();

    ctx.request_repaint();

    egui::CentralPanel::default().show(ctx, |ui| {
        // Title bar
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Playlist").strong().size(20.0));
            ui.with_layout(egui::Layout::right_to_left(Center), |ui| {
                snap_to_grid::render(ui, app);
            });
        });
        ui.separator();

        let (response, painter) = ui.allocate_painter(
            egui::Vec2::new(ui.available_width(), app.ui_state.playlist_height),
            egui::Sense::click_and_drag(),
        );

        let rect = response.rect;
        let pointer_pos = ctx.pointer_interact_pos();

        // Handle input
        input::handle_input(app, ctx, &response, rect, pointer_pos, &config);

        // Lock state for drawing
        let state = app.audio_state.lock().unwrap();

        // Draw everything
        drawing::draw_timeline_header(&painter, rect, &config);
        drawing::draw_beat_markers(&painter, rect, &config, 40);
        drawing::draw_tracks(&painter, rect, &state.playlist, &config);
        drawing::draw_clips(&painter, rect, &state.playlist, &config);
        drawing::draw_playhead(&painter, rect, state.playhead_position, &config);

        drop(state);
    });
}