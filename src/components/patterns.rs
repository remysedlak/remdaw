use eframe::emath;
use crate::models::{MyApp, Pattern};

pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    let mut pattern_to_load: Option<usize> = None;
    let mut should_add_pattern = false;

    egui::SidePanel::left("patterns")
        .resizable(true)
        .default_width(120.0)
        .max_width(150.0)
        .show(ctx, |ui| {
            ui.label(egui::RichText::new("Patterns").strong().size(20.0));
            ui.separator();

            ui.vertical_centered(|ui| {
                // Clone patterns for display only
                let patterns = {
                    let state = app.audio_state.lock().unwrap();
                    state.patterns.clone()
                };

                for (idx, pattern) in patterns.iter().enumerate() {
                    let button = egui::Button::new(format!("{}", pattern.name))
                        .min_size(emath::vec2(100.0, 20.0));

                    if ui.add(button).clicked() {
                        pattern_to_load = Some(idx);
                    }
                    ui.add_space(4.0);
                }

                ui.add_space(4.0);
                let add_button = egui::Button::new("+").min_size(emath::vec2(100.0, 20.0));
                if ui.add(add_button).clicked() {
                    should_add_pattern = true;
                }
            });
        });

    // Handle mutations OUTSIDE the show() closure
    if should_add_pattern || pattern_to_load.is_some() {
        let mut state = app.audio_state.lock().unwrap();

        if should_add_pattern {
            let num = state.patterns.len() + 1;
            let pattern_data = state.pattern.clone(); // Clone FIRST
            state.patterns.push(Pattern {
                name: format!("Pattern {}", num),
                data: pattern_data, // Use the clone
            });
        }

        if let Some(idx) = pattern_to_load {
            if let Some(pattern) = state.patterns.get(idx) {
                state.pattern = pattern.data.clone();
            }
        }
    }
}