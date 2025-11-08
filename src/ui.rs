use crate::audio::{path_to_vector};
use crate::model::{Instrument, MyApp};
use crate::components::{toolbar};

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

        toolbar::render(self, ctx);

        egui::SidePanel::left("files").show(ctx, |ui| {
            ui.label("File Section.")

        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");

            ui.horizontal(|ui| {
                ui.label("this is a label")
            });

              if self.is_channel_rack_open {
                  egui::Window::new("My Window").show(ctx, |ui| {
                      ui.label("Hello World!");

                      let (response, painter) = ui.allocate_painter(
                          egui::Vec2::new(ui.available_width(), 200.0),
                          egui::Sense::click_and_drag()
                      );

                      // Draw a clip as a rectangle
                      let clip_rect = egui::Rect::from_min_size(
                          egui::Pos2::new(100.0, 50.0),  // position
                          egui::Vec2::new(200.0, 80.0)   // size
                      );
                      painter.rect_filled(clip_rect, 5.0, egui::Color32::BLUE);

                      // Check if this "clip" was clicked
                      if response.clicked() {
                          if let Some(pos) = response.interact_pointer_pos() {
                              if clip_rect.contains(pos) {
                                  println!("Clicked the clip!");
                              }
                          }
                      }
                  });
              }
        });
    }
}