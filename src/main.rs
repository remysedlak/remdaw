mod ui;
mod audio;
mod model;
mod components;
mod config;
mod font_loader;

use model::MyApp;
use eframe::egui;
use crate::font_loader::prepare_fonts;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1500.0, 1000.0]),
        ..Default::default()
    };
    eframe::run_native(
        "remdaw",
        options,
        Box::new(|cc| {
            // This gives us image support:
            cc.egui_ctx.set_fonts(prepare_fonts());
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<MyApp>::default())
        }),
    )
}