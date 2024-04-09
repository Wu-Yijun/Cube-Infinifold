#[cfg(test)]
pub mod my_test {
    #[test]
    pub fn test0() {
        // println!("Hello test!");
        let mut i = 10;
        let j = 1;
        let rj = &j;
        let k = loop {
            i -= 1;
            println!("{rj}");
            let rj = &i;
            if i < 0 {
                break rj;
            }
        };
        println!("{k}");
    }

    use std::thread;

    // use chrono;
    use crate::game_options::media;
    #[test]
    pub fn test_image_saver() {
        let t = chrono::offset::Local::now().to_string().replace(":", "_");
        let img = eframe::egui::ColorImage::example();
        thread::spawn(move || {
            if let Some(err) = media::save_image(&format!("output/img-{t}"), "png", &img) {
                println!("Cannot save image! Error: {err}");
            }
        });
    }

    pub fn test_time(){
        // let t = std::time::Instant::now();
        // let dt = std::time::
    }
}
