use eframe::epaint::text::FontDefinitions;

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