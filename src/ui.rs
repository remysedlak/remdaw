use crate::models::{MyApp};
use crate::components::{channel_rack, file_explorer, file_information, patterns, playlist, settings, toolbar};
use crate::components::popups::rename_pattern;

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

        // conditionally render popups
        if self.ui_state.is_channel_rack_open {
            channel_rack::render(self, ctx);
        }

        if self.ui_state.is_file_info_open {
            let file_path = self.selected_file.clone();
            if let Some(ref path) = file_path {
                file_information::render(self, ctx, path);
            }
        }


        // PATTERN rename window
        if let Some(idx) = self.ui_state.pattern_rename_popup {
            rename_pattern::render(self, ctx, idx);
        }

        // render toolbar at top
        toolbar::render(self, ctx);

        // conditionally render side panels
        if self.ui_state.is_files_explorer_open {
            file_explorer::render(self, ctx);
        }
        if self.ui_state.is_patterns_open {
            patterns::render(self, ctx);
        }
        if self.ui_state.is_settings_open {
            settings::render(self, ctx);
        }

        // where the playlist will be modularized
        playlist::render(self, ctx);
    }

    // runs on app close. save user config to storage
    fn on_exit(&mut self, _data: Option<&eframe::glow::Context>) {
        self.config.save();
    }
}