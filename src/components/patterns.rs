use eframe::emath;
use crate::components::popups::rename_pattern;
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
                let patterns = {
                    let state = app.audio_state.lock().unwrap();
                    state.patterns.clone()
                };

                for (idx, mut pattern) in patterns.iter().enumerate() {
                    let (rect, response) = ui.allocate_exact_size(
                        emath::vec2(100.0, 25.0),
                        egui::Sense::click_and_drag()
                    );

                    // Check if this pattern is being dragged
                    let is_being_dragged = ctx.memory(|mem| {
                        mem.data.get_temp::<usize>(egui::Id::new("dragging_pattern")) == Some(idx)
                    });

                    // Draw background
                    let color = if is_being_dragged {
                        egui::Color32::from_rgb(100, 150, 200) // Blue when dragging
                    } else if response.hovered() {
                        egui::Color32::from_gray(80)
                    } else {
                        egui::Color32::from_gray(60)
                    };

                    ui.painter().rect_filled(rect, 3.0, color);

                    // Draw text
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        &pattern.name,
                        egui::FontId::default(),
                        egui::Color32::WHITE
                    );

                    // Handle drag
                    if response.drag_started() {
                        println!("Started dragging pattern {}", idx);
                        ctx.memory_mut(|mem| {
                            mem.data.insert_temp(egui::Id::new("dragging_pattern"), idx);
                        });
                    }

                    if response.dragged() {
                        println!("Dragging pattern {} - delta: {:?}", idx, response.drag_delta());
                    }

                    // Handle click to load
                    if response.clicked() {
                        pattern_to_load = Some(idx);
                    }

                    response.context_menu(|ui| {
                        if ui.button("Delete").clicked() {
                            // Handle delete
                            app.audio_state.lock().unwrap().patterns.remove(idx);
                            ui.close();
                        }
                        if ui.button("Rename").clicked() {
                            // Handle rename
                            app.ui_state.pattern_rename_popup = Some(idx);
                            app.ui_state.rename_buffer = pattern.name.clone();

                            ui.close();
                        }
                        if ui.button("Duplicate").clicked() {
                            // Handle duplicate
                            let mut state = app.audio_state.lock().unwrap();
                            state.patterns.push(pattern.clone());
                            ui.close();
                        }
                        if ui.button("Open").clicked() {
                            // Handle duplicate
                            ui.close();
                        }
                    });

                    ui.add_space(4.0);
                }

                ui.add_space(4.0);
                let add_button = egui::Button::new("+").min_size(emath::vec2(100.0, 20.0));
                if ui.add(add_button).clicked() {
                    should_add_pattern = true;
                }
            });
        });

    if should_add_pattern || pattern_to_load.is_some() {
        let mut state = app.audio_state.lock().unwrap();

        if should_add_pattern {
            let num = state.patterns.len() + 1;
            // Create a blank pattern with the same number of instruments as current
            let num_instruments = state.instruments.len();
            let blank_pattern = vec![vec![false; 16]; num_instruments];

            state.patterns.push(Pattern {
                name: format!("Pattern {}", num),
                data: blank_pattern,
            });
        }

        if let Some(idx) = pattern_to_load {
            // FIRST: Save the current pattern before switching
            if let Some(current_idx) = state.current_pattern_index {
                state.patterns[current_idx].data = state.pattern.clone();
            }

            // THEN: Load the new pattern
            if let Some(pattern) = state.patterns.get(idx) {
                state.pattern = pattern.data.clone();
                state.current_pattern_index = Some(idx); // Update which pattern we're editing
            }
        }
    }
}