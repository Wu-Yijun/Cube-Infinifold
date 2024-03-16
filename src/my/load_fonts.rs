use eframe::egui;

pub fn load_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "font_key".to_owned(),
        egui::FontData::from_static(include_bytes!("../../assets/AlibabaPuHuiTi-2-55-Regular.otf")),
    );
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "font_key".to_owned());
    fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .push("font_key".to_owned());
    ctx.set_fonts(fonts);
}
