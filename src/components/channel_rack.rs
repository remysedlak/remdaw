use crate::audio::path_to_vector;
use crate::models::{Instrument, MyApp};
use crate::utils::get_file_name;

pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    ctx.request_repaint();
    egui::Window::new("Channel Rack")
        .collapsible(true)
        .open(&mut app.ui_state.is_channel_rack_open)
        .show(ctx, |ui| {
            let mut state = app.audio_state.lock().unwrap(); // unlock audio state mutex
            let mut clicked_instrument: Option<usize> = None;
            let current_step = state.current_step;

            ui.spacing_mut().item_spacing = egui::Vec2::new(1.0, 5.0);

            for instrument in 0..state.instruments.len() {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 5.0;

                    // Label
                    if ui.add_sized(
                        [100.0, 25.0],
                        egui::Button::new(&state.instruments[instrument].name).truncate()
                    ).on_hover_text(&state.instruments[instrument].name).clicked() {
                        clicked_instrument = Some(instrument);
                    }

                    // Step buttons
                    for step in 0..16 {
                        let is_active = state.pattern[instrument][step];
                        let is_current = step == current_step && state.is_playing;

                        let button = egui::Button::new("")
                            .min_size(egui::Vec2::new(20.0, 25.0));

                        let valid = vec![4, 5, 6, 7, 12, 13, 14, 15];
                        let is_colored = !valid.contains(&step);

                        let button = if is_current {
                            // Highlight current step with bright color
                            button.fill(egui::Color32::from_rgb(0, 200, 255))
                        } else if is_active {
                            button.fill(egui::Color32::from_rgb(150, 0, 0))
                        } else if is_colored {
                            button.fill(egui::Color32::from_rgb(50, 50, 50))
                        } else {
                            button.fill(egui::Color32::from_rgb(90, 90, 90))
                        };

                        if ui.add(button).clicked() {
                            state.pattern[instrument][step] = !is_active;
                        }
                    }
                });
            }

            if ui.button("+").on_hover_text("Add new file").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    state.instruments.push(Instrument {file_path: path.clone(), name: get_file_name(&path), is_playing: false, position: 0, samples: path_to_vector(path.to_str().unwrap())});
                    state.pattern.push(vec![false; 16]);
                }
            }

            // Handle the click after the loop
            if let Some(idx) = clicked_instrument {
                let file_path = state.instruments[idx].file_path.clone();
                drop(state);
                app.selected_file = Some(file_path);
                app.ui_state.is_file_info_open = true;
            }
        });
}