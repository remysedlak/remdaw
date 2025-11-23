use std::sync::Arc;
use eframe::emath::vec2;
use eframe::epaint::Color32;
use egui::{Frame, Id};
use crate::audio::path_to_vector;
use crate::models::{Instrument, MyApp};
use crate::utils::get_file_name;

pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    ctx.request_repaint();
    egui::Window::new("Channel Rack")
        .collapsible(true)
        .open(&mut app.ui_state.is_channel_rack_open)
        .show(ctx, |ui| {
            let mut state = app.audio_state.lock().unwrap();
            let mut clicked_instrument: Option<usize> = None;
            let current_step = state.current_step;

            let frame = Frame::default()
                .fill(Color32::from_rgb(20, 20, 20))
                .inner_margin(8.0);

            let mut from_idx = None;
            let mut to_idx = None;

            let (_, dropped) = ui.dnd_drop_zone::<usize, ()>(frame, |ui| {
                ui.set_min_size(vec2(200.0, 300.0));
                ui.spacing_mut().item_spacing = egui::Vec2::new(1.0, 5.0);

                // In channel_rack.rs

                for instrument in 0..state.instruments.len() {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 5.0;

                        // Draggable instrument label
                        let id = Id::new(("instrument", instrument));
                        let response = ui.dnd_drag_source(id, instrument, |ui| {
                            if ui.add_sized(
                                [100.0, 25.0],
                                egui::Button::new(&state.instruments[instrument].name).truncate()
                            )
                                .on_hover_text(&state.instruments[instrument].name)
                                .clicked() && !ui.input(|i| i.pointer.button_down(egui::PointerButton::Secondary))
                            {
                                clicked_instrument = Some(instrument);
                            }
                        }).response;

                        // Context menu on the drag source response
                        response.context_menu(|ui| {
                            if ui.button("Delete").clicked() {
                                // Mark for deletion
                                ui.close();
                            }
                            if ui.button("Rename").clicked() {
                                // Handle rename
                                ui.close();
                            }
                        });

                        // Detect hover for reordering
                        if let (Some(pointer), Some(hovered_payload)) = (
                            ui.input(|i| i.pointer.interact_pos()),
                            response.dnd_hover_payload::<usize>(),
                        ) {
                            let rect = response.rect;
                            let stroke = egui::Stroke::new(2.0, Color32::YELLOW);

                            let insert_idx = if pointer.y < rect.center().y {
                                ui.painter().hline(rect.x_range(), rect.top(), stroke);
                                instrument
                            } else {
                                ui.painter().hline(rect.x_range(), rect.bottom(), stroke);
                                instrument + 1
                            };

                            if let Some(dragged_idx) = response.dnd_release_payload::<usize>() {
                                from_idx = Some(dragged_idx);
                                to_idx = Some(insert_idx);
                            }
                        }

                        // Step buttons...
                        // Step buttons (you had "// ... rest of step button code" - here's the actual code)
                        for step in 0..16 {
                            let is_active = state.pattern[instrument][step];
                            let is_current = step == current_step && state.is_playing;

                            let button = egui::Button::new("")
                                .min_size(egui::Vec2::new(20.0, 25.0));

                            let valid = vec![4, 5, 6, 7, 12, 13, 14, 15];
                            let is_colored = !valid.contains(&step);

                            let button = if is_current {
                                button.fill(egui::Color32::from_rgb(0, 200, 255))
                            } else if is_active {
                                button.fill(egui::Color32::from_rgb(150, 0, 0))
                            } else if is_colored {
                                button.fill(egui::Color32::from_rgb(50, 50, 50))
                            } else {
                                button.fill(egui::Color32::from_rgb(90, 90, 90))
                            };

                            let response = ui.add(button);

                            if response.clicked() {
                                state.pattern[instrument][step] = true;
                                if let Some(current_idx) = state.current_pattern_index {
                                    state.patterns[current_idx].data = state.pattern.clone();
                                }
                            }
                            if response.secondary_clicked(){
                                state.pattern[instrument][step] = false;
                                if let Some(current_idx) = state.current_pattern_index {
                                    state.patterns[current_idx].data = state.pattern.clone();
                                }
                            }




                        }
                    });
                }

                if state.instruments.is_empty() {
                    ui.centered_and_justified(|ui| {
                        ui.label("No instruments - click + to add");
                    });
                }
            });

            // Handle drop into empty zone
            if let Some(dragged_idx) = dropped {
                from_idx = Some(dragged_idx);
                to_idx = Some(state.instruments.len());
            }

            // Perform the reorder
            if let (Some(from), Some(to)) = (from_idx, to_idx) {
                if from != Arc::from(to) {
                    // Adjust index if moving within same list
                    let adjusted_to = if from < Arc::from(to) {
                        to - 1
                    } else {
                        to
                    };

                    // Move the instrument
                    let instrument = state.instruments.remove(*from);
                    state.instruments.insert(adjusted_to, instrument);

                    // Move the corresponding pattern
                    let pattern_row = state.pattern.remove(*from);
                    state.pattern.insert(adjusted_to, pattern_row);
                }
            }

            if ui.button("+").on_hover_text("Add new file").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    state.instruments.push(Instrument {
                        file_path: path.clone(),
                        name: get_file_name(&path),
                        is_playing: false,
                        position: 0,
                        samples: path_to_vector(path.to_str().unwrap())
                    });
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