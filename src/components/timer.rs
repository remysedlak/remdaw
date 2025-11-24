use crate::models::MyApp;
use egui::Ui;

pub fn render(ui: &mut Ui, ctx: &egui::Context, app: &mut MyApp) {
    ui.label(
        egui::RichText::new(
            app.audio_state
                .lock()
                .unwrap()
                .get_playhead_time_display()
                .to_string(),
        )
        .color(egui::Color32::RED)
        .size(30.0),
    );
}
