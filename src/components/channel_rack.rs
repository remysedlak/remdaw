use crate::model::MyApp;

pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    egui::Window::new("Channel Rack")
        .collapsible(false)
        .open(&mut app.is_channel_rack_open)
        .show(ctx, |ui| {
            let mut state = app.audio_state.lock().unwrap();

            ui.spacing_mut().item_spacing = egui::Vec2::new(1.0, 5.0); // Global item spacing

            for instrument in 0..state.instruments.len() {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 5.0; // Tight horizontal spacing

                    // Label
                    ui.add_sized(
                        [100.0, 25.0],
                        egui::Label::new(&state.instruments[instrument].name).truncate()
                    ).on_hover_text(&state.instruments[instrument].name);

                    // Step buttons
                    for step in 0..16 {
                        let is_active = state.pattern[instrument][step];

                        let button = egui::Button::new("")
                            .min_size(egui::Vec2::new(20.0, 25.0));

                        let valid = vec![4, 5, 6, 7, 12, 13, 14, 15];
                        let is_colored = !valid.contains(&step);

                        let button = if is_active{
                            button.fill(egui::Color32::from_rgb(150, 0, 0))
                        }
                        else if is_colored  {
                            button.fill(egui::Color32::from_rgb(50, 50, 50))
                        }
                        else {
                            button.fill(egui::Color32::from_rgb(90, 90, 90))
                        };

                        if ui.add(button).clicked() {
                            state.pattern[instrument][step] = !is_active;
                        }
                    }
                });
            }
        });
}