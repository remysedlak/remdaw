use crate::model::{MyApp};
use crate::components::{channel_rack, file_explorer, file_information, settings, toolbar};
use crate::config::AppConfig;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

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

        if self.is_channel_rack_open {
            channel_rack::render(self, ctx);
        }

        if self.is_settings_open {
            settings::render(self, ctx);
        }

        if self.is_file_info_open {
            let file_path = self.selected_file.clone();
            if let Some(ref path) = file_path {
                file_information::render(self, ctx, path);
            }
        }

        toolbar::render(self, ctx);

        file_explorer::render(self, ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");

            ui.horizontal(|ui| {
                ui.label("this is a label")
            });


        });
    }
    fn on_exit(&mut self, _data: Option<&eframe::glow::Context>) {
        self.config.save();
    }
}