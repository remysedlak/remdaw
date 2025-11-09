use crate::models::{MyApp};
use crate::components::{channel_rack, file_explorer, file_information, patterns, playlist, settings, toolbar};

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // test hotkeys here
        ctx.input_mut(|i| {
            if i.consume_key(egui::Modifiers::NONE, egui::Key::T) {
                let mut state = self.audio_state.lock().unwrap();
                // Trigger the first instrument (index 0)
                state.instruments[0].position = 0;
                state.instruments[0].is_playing = true;
            }
            if i.consume_key(egui::Modifiers::NONE, egui::Key::Y) {
                let mut state = self.audio_state.lock().unwrap();
                // Trigger the first instrument (index 0)
                state.instruments[1].position = 0;
                state.instruments[1].is_playing = true;
            }
            if i.consume_key(egui::Modifiers::NONE, egui::Key::I) {
                let mut state = self.audio_state.lock().unwrap();
                // Trigger the first instrument (index 0)
                state.instruments[2].position = 0;
                state.instruments[2].is_playing = true;
            }
        });

        // conditionally render channel rack and file information
        if self.is_channel_rack_open {
            channel_rack::render(self, ctx);
        }
        if self.is_file_info_open {
            let file_path = self.selected_file.clone();
            if let Some(ref path) = file_path {
                file_information::render(self, ctx, path);
            }
        }

        // render toolbar at top
        toolbar::render(self, ctx);

        // conditionally render side panels
        if self.is_files_explorer_open {
            file_explorer::render(self, ctx);
        }
        if self.is_patterns_open {
            patterns::render(self, ctx);
        }
        if self.is_settings_open {
            settings::render(self, ctx);
        }

        // where the playlist will be modularized
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Playlist");
            ui.horizontal(|ui| {
                ui.label("this is a label")
            });
            playlist::render(self, ctx);
        });
    }

    // runs on app close. save user config to storage
    fn on_exit(&mut self, _data: Option<&eframe::glow::Context>) {
        self.config.save();
    }
}