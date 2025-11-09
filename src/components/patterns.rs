use eframe::emath;
use crate::models::{MyApp, Pattern};

pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    egui::SidePanel::left("patterns")
        .resizable(true)
        .default_width(120.0)
        .max_width(150.0)
        .show(ctx, |ui| {
            ui.label(egui::RichText::new("Patterns").strong().size(20.0));
            ui.separator();
            ui.vertical_centered(|ui|{
                let patterns = app.audio_state.lock().unwrap().patterns.clone();

                for pattern in &patterns {
                    ui.add(egui::Button::new(format!("{}", pattern.name)).min_size(emath::vec2(100.0, 20.0)));
                    ui.add_space(4.0);
                }

                ui.add_space(4.0);

                let add_button = egui::Button::new("+").min_size(emath::vec2(100.0, 20.0));
                if ui.add(add_button).clicked() {
                    let mut state = app.audio_state.lock().unwrap();
                    let num = state.patterns.len() + 1;
                    state.patterns.push(Pattern { name: format!("Pattern {num}") });
                }
            });
        });
}