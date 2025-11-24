use eframe::emath;
use egui::{Id, Color32, Vec2, Rect, Pos2, LayerId, Order};
use crate::models::{MyApp, Pattern};
use crate::components::draggable_button::draw_draggable_button;

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
                    let button_id = Id::new(("pattern", idx));
                    let button = draw_draggable_button(
                        ui,
                        button_id,
                        120.0,
                        25.0,
                        &pattern.name,
                        Color32::from_rgb(60, 60, 80),
                        Color32::from_rgb(70, 70, 90),
                        Color32::from_rgb(120, 120, 140),
                        Color32::from_rgb(150, 150, 170),
                        Color32::WHITE,
                    );

                    // Manual payload registration for playlist detection
                    if button.handle_response.drag_started() {
                        ui.memory_mut(|mem| {
                            mem.data.insert_temp(button.handle_id.with("_egui_dnd_drag_payload"), idx);
                        });
                    }

                    // Track dragging
                    if button.handle_response.dragged() {
                        dragging_info = Some((idx, pattern.name.clone()));
                    }

                    // Cursor
                    if button.handle_response.dragged() {
                        ctx.set_cursor_icon(egui::CursorIcon::Grabbing);
                    } else if button.handle_response.hovered() {
                        ctx.set_cursor_icon(egui::CursorIcon::Grab);
                    }

                    button.name_response.context_menu(|ui| {
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

                    if button.name_response.clicked() {
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

            // Draw dots in preview
            let center = preview_handle_rect.center();
            let dot_radius = 1.25;
            let spacing_x = 3.5;
            let spacing_y = 5.5;
            let start_x = center.x - spacing_x;
            let start_y = center.y - spacing_y;

            for row in 0..3 {
                for col in 0..3 {
                    let x = start_x + col as f32 * spacing_x;
                    let y = start_y + row as f32 * spacing_y;
                    painter.circle_filled(Pos2::new(x, y), dot_radius, Color32::from_rgb(200, 200, 220));
                }
            }

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