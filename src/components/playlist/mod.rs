// src/components/playlist/mod.rs

mod config;
mod drawing;
mod drag_drop;
mod resize;

use std::path::PathBuf;
use egui::Id;
use crate::models::MyApp;

pub use config::PlaylistConfig;

pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    let config = PlaylistConfig::default();
    ctx.request_repaint();

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Playlist").strong().size(20.0));
        });
        ui.separator();

        let (response, painter) = ui.allocate_painter(
            egui::Vec2::new(ui.available_width(), app.ui_state.playlist_height),
            egui::Sense::click_and_drag(),
        );

        let rect = response.rect;

        // Check for drops when pointer is released
        // Check for audio file drop
        if let Some(pointer_pos) = ctx.pointer_interact_pos() {
            if ctx.input(|i| i.pointer.any_released()) {
                // Check for PATTERN drops
                // Check for pattern drops
                for idx in 0..100 {
                    let handle_id = Id::new(("pattern", idx)).with("_handle");  // Added .with("_handle")
                    if let Some(pattern_idx) = ctx.memory(|mem| {
                        mem.data.get_temp::<usize>(handle_id.with("_egui_dnd_drag_payload"))
                    }) {
                        drag_drop::handle_pattern_drop_at(app, pointer_pos, rect, &config, pattern_idx);
                        ctx.memory_mut(|mem| {
                            mem.data.remove::<usize>(handle_id.with("_egui_dnd_drag_payload"));
                        });
                        break;
                    }
                }

                // Check for AUDIO FILE drops
                if let Some(file_path) = ctx.memory(|mem| {
                    mem.data.get_temp::<PathBuf>(Id::new("dragging_audio_file_payload"))
                }) {
                    drag_drop::handle_audio_drop_at(app, pointer_pos, rect, &config, file_path);
                    ctx.memory_mut(|mem| {
                        mem.data.remove::<PathBuf>(Id::new("dragging_audio_file_payload"));
                    });
                }
            }
        }

        let state = app.audio_state.lock().unwrap();

        drawing::draw_timeline_header(&painter, rect, &config);
        drawing::draw_beat_markers(&painter, rect, &config, 40);
        drawing::draw_tracks(&painter, rect, &state.playlist, &config);
        drawing::draw_clips(&painter, rect, &state.playlist, &config, ui);
        drawing::draw_playhead(&painter, rect, state.playhead_position, &config);

        drop(state);
    });
}