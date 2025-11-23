use eframe::emath;
use egui::{Id, Color32, Vec2, Sense, Rect, Pos2, LayerId, Order};
use crate::models::{MyApp, Pattern};

pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    let mut pattern_to_load: Option<usize> = None;
    let mut should_add_pattern = false;
    let mut dragging_info: Option<(usize, String)> = None;

    egui::SidePanel::left("patterns")
        .resizable(true)
        .default_width(140.0)
        .max_width(170.0)
        .show(ctx, |ui| {
            ui.label(egui::RichText::new("Patterns").strong().size(20.0));
            ui.separator();

            ui.vertical_centered(|ui| {
                let patterns = {
                    let state = app.audio_state.lock().unwrap();
                    state.patterns.clone()
                };

                for (idx, pattern) in patterns.iter().enumerate() {
                    let total_width = 120.0;
                    let handle_width = 20.0;
                    let name_width = total_width - handle_width;
                    let height = 25.0;

                    let (full_rect, _) = ui.allocate_exact_size(
                        Vec2::new(total_width, height),
                        Sense::hover()
                    );

                    // grab portion
                    let handle_rect = Rect::from_min_size(
                        full_rect.min,
                        Vec2::new(handle_width, height)
                    );
                    // interactive portion
                    let name_rect = Rect::from_min_size(
                        Pos2::new(full_rect.min.x + handle_width, full_rect.min.y),
                        Vec2::new(name_width, height)
                    );

                    let name_id = ui.id().with(("pattern_name", idx));
                    let name_response = ui.interact(name_rect, name_id, Sense::click());

                    let handle_id = Id::new(("pattern_drag_handle", idx));
                    let handle_response = ui.interact(handle_rect, handle_id, Sense::click_and_drag());

                    // Manual payload registration for playlist detection
                    if handle_response.drag_started() {
                        ui.memory_mut(|mem| {
                            mem.data.insert_temp(handle_id.with("_egui_dnd_drag_payload"), idx);
                        });
                    }

                    // Track what's being dragged for visual preview
                    if handle_response.dragged() {
                        dragging_info = Some((idx, pattern.name.clone()));
                    }

                    // Change cursor based on state
                    if handle_response.dragged() {
                        ctx.set_cursor_icon(egui::CursorIcon::Grabbing);
                    } else if handle_response.hovered() {
                        ctx.set_cursor_icon(egui::CursorIcon::Grab);
                    }

                    // Color of the pattern buttons
                    let bg_color = if name_response.hovered() {
                        Color32::from_rgb(70, 70, 90)
                    } else {
                        Color32::from_rgb(60, 60, 80)
                    };
                    ui.painter().rect_filled(full_rect, 3.0, bg_color);

                    // Draw handle icon
                    let handle_color = if handle_response.hovered() {
                        Color32::from_rgb(150, 150, 170)
                    } else {
                        Color32::from_rgb(120, 120, 140)
                    };

                    draw_drag_dots(ui, handle_rect, handle_color);

                    ui.painter().vline(
                        handle_rect.right(),
                        full_rect.y_range(),
                        egui::Stroke::new(1.0, Color32::from_rgb(80, 80, 100))
                    );

                    ui.painter().text(
                        name_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        &pattern.name,
                        egui::FontId::default(),
                        Color32::WHITE
                    );

                    name_response.context_menu(|ui| {
                        if ui.button("Delete").clicked() {
                            app.audio_state.lock().unwrap().patterns.remove(idx);
                            ui.close();
                        }
                        if ui.button("Rename").clicked() {
                            app.ui_state.pattern_rename_popup = Some(idx);
                            app.ui_state.rename_buffer = pattern.name.clone();
                            ui.close();
                        }
                        if ui.button("Duplicate").clicked() {
                            let mut state = app.audio_state.lock().unwrap();
                            state.patterns.push(pattern.clone());
                            ui.close();
                        }
                    });

                    if name_response.clicked() {
                        pattern_to_load = Some(idx);
                    }

                    ui.add_space(4.0);
                }

                ui.add_space(4.0);
                let add_button = egui::Button::new("+").min_size(emath::vec2(120.0, 20.0));
                if ui.add(add_button).clicked() {
                    should_add_pattern = true;
                }
            });
        });

    // Draw drag preview AFTER panel on Foreground layer
    if let Some((_idx, name)) = dragging_info {
        if let Some(pointer_pos) = ctx.pointer_interact_pos() {
            let total_width = 120.0;
            let handle_width = 20.0;
            let name_width = total_width - handle_width;
            let height = 25.0;

            let layer_id = LayerId::new(Order::Foreground, Id::new("drag_preview"));
            let painter = ctx.layer_painter(layer_id);

            let preview_rect = Rect::from_min_size(
                pointer_pos,
                Vec2::new(total_width, height)
            );

            painter.rect_filled(
                preview_rect,
                3.0,
                Color32::from_rgb(100, 100, 120).gamma_multiply(0.9)
            );

            painter.rect_stroke(
                preview_rect,
                3.0,
                egui::Stroke::new(2.0, Color32::from_rgb(200, 200, 255)),
                egui::StrokeKind::Middle
            );

            let preview_handle_rect = Rect::from_min_size(
                preview_rect.min,
                Vec2::new(handle_width, height)
            );
            painter.text(
                preview_handle_rect.center(),
                egui::Align2::CENTER_CENTER,
                "ll",
                egui::FontId::default(),
                Color32::from_rgb(200, 200, 220)
            );

            painter.vline(
                preview_handle_rect.right(),
                preview_rect.y_range(),
                egui::Stroke::new(1.0, Color32::from_rgb(150, 150, 180))
            );

            let preview_name_rect = Rect::from_min_size(
                Pos2::new(preview_rect.min.x + handle_width, preview_rect.min.y),
                Vec2::new(name_width, height)
            );
            painter.text(
                preview_name_rect.center(),
                egui::Align2::CENTER_CENTER,
                &name,
                egui::FontId::default(),
                Color32::WHITE
            );
        }
    }

    if should_add_pattern || pattern_to_load.is_some() {
        let mut state = app.audio_state.lock().unwrap();

        if should_add_pattern {
            let num = state.patterns.len() + 1;
            let num_instruments = state.instruments.len();
            let blank_pattern = vec![vec![false; 16]; num_instruments];

            state.patterns.push(Pattern {
                name: format!("Pattern {}", num),
                data: blank_pattern,
            });
        }

        if let Some(idx) = pattern_to_load {
            if let Some(current_idx) = state.current_pattern_index {
                state.patterns[current_idx].data = state.pattern.clone();
            }

            if let Some(pattern) = state.patterns.get(idx) {
                state.pattern = pattern.data.clone();
                state.current_pattern_index = Some(idx);
            }
        }
    }
}

pub fn draw_drag_dots(ui: &mut egui::Ui, handle_rect: Rect, handle_color: Color32) {
    // Draw 3x3 grid of dots (braille pattern)
    let center = handle_rect.center();
    let dot_radius = 1.25;
    let spacing_x = 3.5;
    let spacing_y = 5.5;

    // Calculate starting position (top-left dot)
    let start_x = center.x - spacing_x;
    let start_y = center.y - spacing_y;

    // Draw 3x3 grid
    for row in 0..3 {
        for col in 0..3 {
            let x = start_x + col as f32 * spacing_x;
            let y = start_y + row as f32 * spacing_y;
            ui.painter().circle_filled(Pos2::new(x, y), dot_radius, handle_color);
        }
    }
}