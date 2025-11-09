use crate::model::MyApp;

pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            let mut state = app.audio_state.lock().unwrap();

            // Editable BPM with DragValue
            ui.label("BPM:");
            if ui
                .add(
                    egui::DragValue::new(&mut state.bpm)
                        .speed(1.0)
                        .range(40..=300),
                )
                .changed()
            {
                // Recalculate samples_per_beat when BPM changes
                state.samples_per_beat = state.sampling_rate * 60.0 / state.bpm as f32;
            }

            ui.add_space(24.0);

            ui.label(format!("SR: {}", state.sampling_rate));
            ui.label(format!("SPB: {:.0}", state.samples_per_beat));

            ui.add_space(24.0);

            let label = if state.is_playing { "pause" } else { "play" };
            if ui.button(label).clicked() {
                state.is_playing = !state.is_playing;
                state.metronome_counter = 0.0;
            }

            if ui.button("stop").clicked() {
                state.is_playing = false;
                state.metronome_counter = 0.0;
            }

            if ui.button("metro").clicked() {
                state.is_metronome = !state.is_metronome;
            }

            drop(state); // Release lock before next button

            if ui.button("rack").clicked() {
                app.is_channel_rack_open = !app.is_channel_rack_open;
            }

            if ui.button("settings").clicked() {
                app.is_settings_open = !app.is_settings_open;
            }

            if ui.button("files").clicked() {
                app.is_files_explorer_open = !app.is_files_explorer_open;
            }

            if ui.button("patterns").clicked() {
                app.is_patterns_open = !app.is_patterns_open
            }


        })
    });
}
