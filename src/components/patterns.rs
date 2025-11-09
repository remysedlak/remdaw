use eframe::emath;
use crate::model::MyApp;

pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    egui::SidePanel::left("patterns")
        .resizable(true)
        .default_width(120.0)
        .max_width(150.0)
        .show(ctx, |ui| {
            ui.add_space(10.0);
            ui.label(egui::RichText::new("Patterns").strong().size(20.0));
            ui.separator();
            ui.vertical_centered(|ui|{
                let add_button = egui::Button::new("+").min_size(emath::vec2(100.0, 20.0));
                if ui.add(add_button).clicked() {
                    // Your code here
                }
            });


        });
}