mod ui;
mod audio;
mod models;
mod components;
mod config;
mod utils;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "remdaw", // app title
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_fonts(utils::prepare_fonts()); // font support
            egui_extras::install_image_loaders(&cc.egui_ctx); // image support
            Ok(Box::<models::MyApp>::default())
        }),
    )
}