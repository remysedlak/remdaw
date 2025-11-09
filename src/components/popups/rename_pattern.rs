use crate::models::MyApp;

pub fn render(app: &mut MyApp, ctx: &egui::Context, idx: usize) {
    let mut is_open = true;
    egui::Window::new("Rename Pattern")
        .open(&mut is_open)
        .show(ctx, |ui| {
            ui.label("Pattern name:");
            ui.text_edit_singleline(&mut app.ui_state.rename_buffer);

            ui.horizontal(|ui| {
                if ui.button("OK").clicked() {
                    let mut state = app.audio_state.lock().unwrap();
                    if idx < state.patterns.len() {
                        state.patterns[idx].name = app.ui_state.rename_buffer.clone();
                    }
                    app.ui_state.pattern_rename_popup = None;
                }
                if ui.button("Cancel").clicked() {
                    app.ui_state.pattern_rename_popup = None;
                }
            });
        });

    if !is_open {
        app.ui_state.pattern_rename_popup = None;
    }
}