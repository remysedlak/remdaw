use std::sync::{Arc, Mutex};
use cpal::Stream;
use crate::audio;
use crate::audio::AudioState;

pub struct MyApp {
    name: String,
    age: u32,
    audio_stream: Stream,
    audio_state: Arc<Mutex<AudioState>>
}

impl Default for MyApp {
    fn default() -> Self {
        let (audio_stream, audio_state) = audio::init();
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            audio_stream,
            audio_state
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        ctx.input_mut(|i| {
            if i.consume_key(egui::Modifiers::NONE, egui::Key::T) {
                let mut state = self.audio_state.lock().unwrap(); // lock only here
                state.kick_playing = true;
                state.kick_position = 0;
            } // unlock immediately
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}