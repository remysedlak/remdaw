use egui::Frame;
use crate::model::MyApp;

pub fn render(app: &MyApp, ctx: &egui::Context) {
    egui::SidePanel::left("files")
        .frame(Frame::new().inner_margin(12.0))
        .resizable(true)
        .max_width(500.0)
        .show(ctx, |ui| {
            ui.label(egui::RichText::new("Files").strong().size(20.0));
            ui.separator();
        });
}