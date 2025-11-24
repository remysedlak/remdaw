use crate::models::MyApp;
use crate::components::draggable_button::draw_draggable_button;
use egui::{Color32, Id, LayerId, Order, Pos2, Rect, Vec2};
use std::fs;
use std::path::{Path, PathBuf};

pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    let mut dragging_info: Option<(PathBuf, String)> = None;

    egui::SidePanel::left("files")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label(egui::RichText::new("Files").strong().size(20.0));
            ui.separator();

            let root_path = PathBuf::from(&app.config.file_path);

            if root_path.exists() && root_path.is_dir() {
                render_directory(ui, app, ctx, &root_path, 0, &mut dragging_info);
            } else {
                ui.label("Invalid directory path");
            }
        });

    // Store what's being dragged in memory
    if let Some((path, _name)) = &dragging_info {
        ctx.memory_mut(|mem| {
            mem.data
                .insert_temp(Id::new("dragging_audio_file_payload"), path.clone());
        });
    }

    // Draw drag preview AFTER panel
    if let Some((_path, name)) = dragging_info {
        if let Some(pointer_pos) = ctx.pointer_interact_pos() {
            let total_width = 150.0;
            let handle_width = 20.0;
            let name_width = total_width - handle_width;
            let height = 20.0;

            let layer_id = LayerId::new(Order::Foreground, Id::new("file_drag_preview"));
            let painter = ctx.layer_painter(layer_id);

            let preview_rect = Rect::from_min_size(pointer_pos, Vec2::new(total_width, height));

            painter.rect_filled(
                preview_rect,
                3.0,
                Color32::from_rgb(200, 120, 80).gamma_multiply(0.9),
            );

            painter.rect_stroke(
                preview_rect,
                3.0,
                egui::Stroke::new(2.0, Color32::from_rgb(255, 150, 100)),
                egui::StrokeKind::Middle
            );

            let preview_handle_rect =
                Rect::from_min_size(preview_rect.min, Vec2::new(handle_width, height));

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
                    painter.circle_filled(
                        Pos2::new(x, y),
                        dot_radius,
                        Color32::from_rgb(220, 180, 140),
                    );
                }
            }

            painter.vline(
                preview_handle_rect.right(),
                preview_rect.y_range(),
                egui::Stroke::new(1.0, Color32::from_rgb(180, 120, 80)),
            );

            let preview_name_rect = Rect::from_min_size(
                Pos2::new(preview_rect.min.x + handle_width, preview_rect.min.y),
                Vec2::new(name_width, height),
            );
            painter.text(
                preview_name_rect.center(),
                egui::Align2::CENTER_CENTER,
                &name,
                egui::FontId::default(),
                Color32::WHITE,
            );
        }
    }
}

fn render_directory(
    ui: &mut egui::Ui,
    app: &mut MyApp,
    ctx: &egui::Context,
    path: &Path,
    depth: usize,
    dragging_info: &mut Option<(PathBuf, String)>,
) {
    if let Ok(entries) = fs::read_dir(path) {
        let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
        entries.sort_by_key(|e| e.path());

        for entry in entries {
            let path = entry.path();
            let name = path.file_name().unwrap().to_string_lossy().to_string();

            ui.horizontal(|ui| {
                ui.add_space(depth as f32 * 15.0);

                if path.is_dir() {
                    let id = ui.make_persistent_id(&path);
                    egui::collapsing_header::CollapsingState::load_with_default_open(
                        ui.ctx(),
                        id,
                        false,
                    )
                        .show_header(ui, |ui| {
                            ui.label(format!("📁 {}", name));
                        })
                        .body(|ui| {
                            render_directory(ui, app, ctx, &path, depth + 1, dragging_info);
                        });
                } else {
                    let is_wav = path.extension().and_then(|s| s.to_str()) == Some("wav");

                    if is_wav {
                        let button_id = Id::new(("file", path.to_str().unwrap_or("")));
                        let button = draw_draggable_button(
                            ui,
                            button_id,
                            150.0,
                            20.0,
                            &name,
                            Color32::from_gray(60),
                            Color32::from_gray(80),
                            Color32::from_rgb(150, 120, 90),
                            Color32::from_rgb(200, 160, 120),
                            Color32::WHITE,
                        );

                        if button.handle_response.dragged() {
                            ctx.set_cursor_icon(egui::CursorIcon::Grabbing);
                            *dragging_info = Some((path.clone(), name.clone()));
                        } else if button.handle_response.hovered() {
                            ctx.set_cursor_icon(egui::CursorIcon::Grab);
                        }

                        // Play sound on click
                        if button.name_response.clicked() {
                            let samples = crate::audio::path_to_vector(path.to_str().unwrap());
                            let mut state = app.audio_state.lock().unwrap();

                            state.preview_sound = Some(crate::models::Instrument {
                                name: name.clone(),
                                file_path: path.clone(),
                                samples,
                                position: 0,
                                is_playing: true,
                            });
                        }
                    } else {
                        ui.label(format!("📄 {}", name));
                    }
                }
            });
        }
    }
}