use std::path::PathBuf;
use crate::models::{Instrument, MyApp};

pub fn render(app: &mut MyApp, ctx: &egui::Context, file: &PathBuf) {
    egui::Window::new("File Information")
        .open(&mut app.ui_state.is_file_info_open)
        .show(ctx, |ui| {
            // Get filename
            let name = file.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown");

            ui.label(format!("Name: {}", name));
            ui.label(format!("Path: {}", file.display()));

            // Load WAV metadata if it's a WAV file
            if let Ok(reader) = hound::WavReader::open(file) {
                let spec = reader.spec();
                ui.separator();
                ui.label(format!("Sample Rate: {} Hz", spec.sample_rate));
                ui.label(format!("Channels: {}", spec.channels));
                ui.label(format!("Bits per Sample: {}", spec.bits_per_sample));

                let duration = reader.duration() as f32 / spec.sample_rate as f32;
                ui.label(format!("Duration: {:.2}s", duration));

                if ui.button("Load into Channel Rack").clicked() {
                    let samples = crate::audio::path_to_vector(file.to_str().unwrap());
                    let mut state = app.audio_state.lock().unwrap();
                    state.instruments.push(Instrument {
                        name: name.to_string(),
                        file_path: file.clone(),  // Clone the PathBuf
                        samples,
                        position: 0,
                        is_playing: false,
                    });
                }
            } else {
                ui.label("Could not read file");
            }
        });
}