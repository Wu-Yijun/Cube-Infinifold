pub fn main() {
    println!("Checking font loading...");
    load_fonts();
    println!("Checking font loading... OK");
}

use eframe::egui;

fn load_fonts() {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "font_key".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../../../assets/ui/AlibabaPuHuiTi-2-55-Regular.otf"
        )),
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
}
