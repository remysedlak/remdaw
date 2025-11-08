use std::fs;
use std::path::{Path, PathBuf};
use crate::model::MyApp;

pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    egui::SidePanel::left("files")
        .resizable(true)
        .show(ctx, |ui| {
            ui.add_space(10.0);
            ui.label(egui::RichText::new("Files").strong().size(20.0));
            ui.separator();

            let root_path = PathBuf::from(&app.config.file_path);

            if root_path.exists() && root_path.is_dir() {
                render_directory(ui, app, &root_path, 0);
            } else {
                ui.label("Invalid directory path");
            }
        });
}

fn render_directory(ui: &mut egui::Ui, app: &mut MyApp, path: &Path, depth: usize) {
    if let Ok(entries) = fs::read_dir(path) {
        let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
        entries.sort_by_key(|e| e.path());

        for entry in entries {
            let path = entry.path();
            let name = path.file_name().unwrap().to_string_lossy().to_string();

            ui.vertical(|ui| {
                ui.add_space(depth as f32 * 15.0); // Indent based on depth

                if path.is_dir() {
                    // Folder
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
                            render_directory(ui, app, &path, depth + 1);
                        });
                } else {
                    // File - check if it's a WAV file
                    if ui.button(format!("📄 {}", name)).clicked() {
                        if path.extension().and_then(|s| s.to_str()) == Some("wav") {
                            let samples = crate::audio::path_to_vector(path.to_str().unwrap());
                            let mut state = app.audio_state.lock().unwrap();

                            // Set as preview sound (replaces any existing preview)
                            state.preview_sound = Some(crate::model::Instrument {
                                name: name.clone(),
                                file_path: path.clone(),
                                samples,
                                position: 0,
                                is_playing: true,
                            });
                        }
                    }
                }
            });
        }
    }
}