use crate::model::{MyApp};
pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            let (bpm,sample_rate, samples_per_beat) = {
                let state = app.audio_state.lock().unwrap();
                (state.bpm, state.sampling_rate, state.samples_per_beat)
            };
            ui.label(format!("BPM: {}",bpm.to_string()));
            ui.label(format!("Sampling Rate: {}",sample_rate.to_string()));
            ui.label(format!("Samples per beat: {}",samples_per_beat.to_string()));
            let label = if app.audio_state.lock().unwrap().is_playing {"pause"} else {"play"};
            if ui.button(label).clicked() {
                let mut state = app.audio_state.lock().unwrap();
                state.is_playing = !state.is_playing;
                state.metronome_counter = 0.0;
            }
            if ui.button("stop").clicked(){
                app.audio_state.lock().unwrap().is_playing = false;
                app.audio_state.lock().unwrap().metronome_counter = 0.0;
            }

            if ui.button("metro").clicked(){
                let mut state = app.audio_state.lock().unwrap();
                state.is_metronome = !state.is_metronome;
            }
            if ui.button("rack").clicked(){
                app.is_channel_rack_open = !app.is_channel_rack_open;
            }
        })
    });
}