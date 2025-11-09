use crate::models::MyApp;


pub(crate) fn render(app: &mut MyApp, ctx: &egui::Context) {
    egui::Window::new("Settings")
        .min_width(400.0)
        .resizable(false)
        .collapsible(false)
        .open(&mut app.is_settings_open)
        .show(&ctx, |ui| {
            ui.label(egui::RichText::new("Settings").strong().size(20.0));
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("File Path:");
                ui.text_edit_singleline(&mut app.config.file_path);

                if ui.button("Browse...").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        app.config.file_path = path.display().to_string();
                    }
                }
            });

            if ui.button("Save").clicked() {
                app.config.save(); // Save immediately when user clicks
                // Optionally show a confirmation message
            }
        });
}