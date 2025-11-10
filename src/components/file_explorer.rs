use std::fs;
use std::path::{Path, PathBuf};
use crate::models::MyApp;

pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    egui::SidePanel::left("files")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label(egui::RichText::new("Files").strong().size(20.0));
            ui.separator();

            let root_path = PathBuf::from(&app.config.file_path);

            if root_path.exists() && root_path.is_dir() {
                render_directory(ui, app, ctx, &root_path, 0);
            } else {
                ui.label("Invalid directory path");
            }
        });
}

fn render_directory(ui: &mut egui::Ui, app: &mut MyApp, ctx: &egui::Context, path: &Path, depth: usize) {
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
                        false
                    )
                        .show_header(ui, |ui| {
                            ui.label(format!("📁 {}", name));
                        })
                        .body(|ui| {
                            render_directory(ui, app, ctx, &path, depth + 1);
                        });
                } else {
                    let is_wav = path.extension().and_then(|s| s.to_str()) == Some("wav");

                    if is_wav {
                        // Check if this file is being dragged
                        let is_being_dragged = ctx.memory(|mem| {
                            mem.data.get_temp::<PathBuf>(egui::Id::new("dragging_audio_file"))
                                .as_ref() == Some(&path)
                        });

                        // Use a custom draggable area instead of button
                        let (rect, response) = ui.allocate_exact_size(
                            egui::Vec2::new(150.0, 20.0),
                            egui::Sense::click_and_drag()
                        );

                        // Draw background
                        let color = if is_being_dragged {
                            egui::Color32::from_rgb(200, 120, 80) // Orange when dragging
                        } else if response.hovered() {
                            egui::Color32::from_gray(80)
                        } else {
                            egui::Color32::from_gray(60)
                        };

                        ui.painter().rect_filled(rect, 3.0, color);

                        // Draw text
                        ui.painter().text(
                            rect.left_center() + egui::vec2(5.0, 0.0),
                            egui::Align2::LEFT_CENTER,
                            format!(">> {}", name),
                            egui::FontId::default(),
                            egui::Color32::WHITE
                        );

                        // Handle drag
                        if response.drag_started() {
                            println!("Started dragging file: {}", name);
                            ctx.memory_mut(|mem| {
                                mem.data.insert_temp(egui::Id::new("dragging_audio_file"), path.clone());
                            });
                        }

                        if response.dragged() {
                            println!("Dragging file: {} - delta: {:?}", name, response.drag_delta());
                        }

                        // Handle click to preview
                        if response.clicked() {
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