use crate::models::MyApp;
use eframe::emath::Align::Center;

pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.add_space(12.0);

        ui.horizontal(|ui| {
            let mut state = app.audio_state.lock().unwrap();

            // Editable BPM with DragValue
            ui.label("BPM:");
            if ui
                .add(egui::DragValue::new(&mut state.bpm)
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

            let label = if state.is_playing { "\u{23F8}" } else { "\u{25B6}" }; // pause else play

            if ui.add_sized([25.0, 20.0], egui::Button::new(label)).clicked() {
                if !state.is_playing {
                    state.just_started = true;
                    state.is_playing = true;
                }
                else {
                    state.just_started = false;
                    state.is_playing = false;
                }
            }
            if ui.add(egui::Button::new("⏹")).clicked() {
                state.is_playing = false;
                state.playhead_position = 0.0;
                state.metronome_counter = 0.0;
            }

            if ui.button("metro").clicked() {
                state.is_metronome = !state.is_metronome;
            }

            drop(state); // Release lock before next button

            ui.add_space(24.0);

            if ui.button("rack").clicked() {
                app.ui_state.is_channel_rack_open = !app.ui_state.is_channel_rack_open;
            }

            if ui.button("files").clicked() {
                app.ui_state.is_files_explorer_open = !app.ui_state.is_files_explorer_open;
            }

            if ui.button("patterns").clicked() {
                app.ui_state.is_patterns_open = !app.ui_state.is_patterns_open
            }

            // Add this somewhere in your UI (maybe in main.rs or a settings panel)
            ui.horizontal(|ui| {
                ui.checkbox(&mut app.ui_state.snap_to_grid, "Snap to Grid");

                if app.ui_state.snap_to_grid {
                    ui.label("Snap:");
                    if ui.selectable_label(app.ui_state.snap_division == 4.0, "Bar").clicked() {
                        app.ui_state.snap_division = 4.0; // 4 beats = 1 bar
                    }
                    if ui.selectable_label(app.ui_state.snap_division == 1.0, "Beat").clicked() {
                        app.ui_state.snap_division = 1.0; // 1 beat
                    }
                    if ui.selectable_label(app.ui_state.snap_division == 0.5, "1/2").clicked() {
                        app.ui_state.snap_division = 0.5; // Half beat
                    }
                    if ui.selectable_label(app.ui_state.snap_division == 0.25, "1/4").clicked() {
                        app.ui_state.snap_division = 0.25; // Quarter beat
                    }
                    if ui.selectable_label(app.ui_state.snap_division == 0.125, "1/8").clicked() {
                        app.ui_state.snap_division = 0.125; // Eighth beat
                    }
                }
            });

            ui.with_layout(egui::Layout::right_to_left(Center), |ui| {
                if ui.button("settings").clicked() {
                    app.ui_state.is_settings_open = !app.ui_state.is_settings_open;
                }
            });
        });
        ui.add_space(12.0);
    });

}
