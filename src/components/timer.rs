use crate::models::AudioState;

pub fn render(ui: &mut egui::Ui, audio_state: &AudioState) {
    ui.label(
        egui::RichText::new(audio_state.get_playhead_time_display().to_string())
            .color(egui::Color32::GREEN)
            .background_color(egui::Color32::BLACK)
            .size(18.0),
    );
}
