use std::path::PathBuf;
use eframe::epaint::text::FontDefinitions;

/// FontDefinition constructor called on app init.
/// Prepares the proper FontDefinitions with a ttf font file.
pub fn prepare_fonts() -> FontDefinitions {

    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "app_font".to_owned(),
        std::sync::Arc::from(egui::FontData::from_static(include_bytes!("../assets/AzeretMono-Regular.ttf")))
    );

    // Set as primary font for proportional and mono text
    fonts.families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "app_font".to_owned());

    fonts.families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .insert(0, "app_font".to_owned());
    fonts
}

pub fn get_file_name(file: &PathBuf) -> String {
    file.file_name()
    .and_then(|n| n.to_str())
    .unwrap_or("Unknown").to_owned()
}