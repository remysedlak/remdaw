pub fn render(ui: &mut egui::Ui, app: &mut crate::models::MyApp) {
    ui.horizontal(|ui| {
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
        ui.checkbox(&mut app.ui_state.snap_to_grid, "Snap to Grid");
    });
}