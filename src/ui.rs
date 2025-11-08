use crate::audio::{path_to_vector};
use crate::model::{Instrument, MyApp};

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
            if i.consume_key(egui::Modifiers::NONE, egui::Key::U) {
                let mut state = self.audio_state.lock().unwrap();
                let vector = path_to_vector("instruments/Boss DR-660/Clap/Clap Dance.wav");
                state.instruments.push(Instrument {is_playing: false, position: 0, samples: vector });
            }
            if i.consume_key(egui::Modifiers::NONE, egui::Key::I) {
                let mut state = self.audio_state.lock().unwrap();
                // Trigger the first instrument (index 0)
                state.instruments[2].position = 0;
                state.instruments[2].is_playing = true;
            }
        });

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Top Panel").clicked(){
                    self.is_channel_rack_open = !self.is_channel_rack_open;
                }
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");

            ui.horizontal(|ui| {
                ui.label("this is a label")
            });

              if self.is_channel_rack_open {
                  egui::Window::new("My Window").show(ctx, |ui| {
                      ui.label("Hello World!");
                  });
              }
        });
    }
}