use crate::model::MyApp;

pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    egui::Window::new("Channel Rack")
        .collapsible(false)
        .open(&mut app.is_channel_rack_open)
        .show(ctx, |ui| {
            let mut state = app.audio_state.lock().unwrap();
            let mut clicked_instrument: Option<usize> = None;

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

                        let button = egui::Button::new("")
                            .min_size(egui::Vec2::new(20.0, 25.0));

                        let valid = vec![4, 5, 6, 7, 12, 13, 14, 15];
                        let is_colored = !valid.contains(&step);

                        let button = if is_active {
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

            // Handle the click after the loop
            if let Some(idx) = clicked_instrument {
                let file_path = state.instruments[idx].file_path.clone();
                drop(state); // Drop lock before modifying app
                app.selected_file = Some(file_path);
                app.is_file_info_open = true;
            }
        });
}