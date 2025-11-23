// src/components/playlist/mod.rs

mod config;
mod drawing;
mod drag_drop;
mod resize;

use std::path::PathBuf;
use eframe::emath::Align::Center;
use crate::models::MyApp;
use crate::components::snap_to_grid;

pub use config::PlaylistConfig;

// src/components/playlist/mod.rs

// src/components/playlist/mod.rs

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
        if let Some(pointer_pos) = ctx.pointer_interact_pos() {
            if ctx.input(|i| i.pointer.any_released()) {
                // Check for pattern drop - look for ANY pattern handle payload
                for idx in 0..100 {  // Check up to 100 patterns
                    let handle_id = egui::Id::new(("pattern_drag_handle", idx));
                    if let Some(pattern_idx) = ctx.memory(|mem| {
                        mem.data.get_temp::<usize>(handle_id.with("_egui_dnd_drag_payload"))
                    }) {
                        drag_drop::handle_pattern_drop_at(app, pointer_pos, rect, &config, pattern_idx);
                        // Clear the payload
                        ctx.memory_mut(|mem| {
                            mem.data.remove::<usize>(handle_id.with("_egui_dnd_drag_payload"));
                        });
                        break;
                    }
                }
            }
        }

        // Check for audio file drops
        if let (Some(pointer_pos), Some(file_path)) = (
            ctx.pointer_interact_pos(),
            response.dnd_release_payload::<PathBuf>(),
        ) {
            drag_drop::handle_audio_drop_at(app, pointer_pos, rect, &config, (*file_path).clone());
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