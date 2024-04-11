#[allow(dead_code)]
#[warn(unused_must_use)]
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

    use std::sync::mpsc;
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

    #[test]
    pub fn test_vec() {
        let mut img = vec![0; 20];
        img[1] = 10;
        img[2] = 20;
        println!("{:#?}", img);
    }

    use ndarray::Array3;
    use video_rs::encode::{Encoder, Settings};
    use video_rs::time::Time;
    #[test]
    pub fn test_mp4_encoder() {
        video_rs::init().unwrap();

        // let set2 = Settings::preset_h264_custom(
        //     1280,
        //     720,
        //     video_rs::ffmpeg::format::Pixel::RGBA,
        //     Options::preset_h264_realtime(),
        // );
        let settings = Settings::preset_h264_yuv420p(1280, 720, false);
        // let mut encoder = Encoder::new(std::path::Path::new("output/rainbow2.mp4"), settings)
        //     .expect("failed to create encoder");
        let mut encoder = Encoder::new(std::path::Path::new("output/rainbow3.mp4"), settings)
            .expect("Cannot create");

        let mut position = Time::zero();
        for i in 0..100 {
            // This will create a smooth rainbow animation video!
            let frame = rainbow_frame(i as f32 / 100.0);

            encoder
                .encode(&frame, &position)
                .expect("failed to encode frame");

            let duration: Time = Time::from_secs_f64(4.0 / (5.0 + i as f64));
            // Update the current position and add the inter-frame duration to it.
            position = position.aligned_with(&duration).add();

            // sleep(Duration::from_millis(1000 / 60));
            println!("{i}");
        }

        encoder.finish().expect("failed to finish encoder");
    }

    fn rainbow_frame(p: f32) -> Array3<u8> {
        // This is what generated the rainbow effect! We loop through the HSV color spectrum and convert
        // to RGB.
        let rgb = hsv_to_rgb(p * 360.0, 100.0, 100.0);

        // This creates a frame with height 720, width 1280 and three channels. The RGB values for each
        // pixel are equal, and determined by the `rgb` we chose above.
        Array3::from_shape_fn((720, 1280, 3), |(_y, _x, c)| rgb[c])
    }

    fn hsv_to_rgb(h: f32, s: f32, v: f32) -> [u8; 3] {
        let s = s / 100.0;
        let v = v / 100.0;
        let c = s * v;
        let x = c * (1.0 - (((h / 60.0) % 2.0) - 1.0).abs());
        let m = v - c;
        let (r, g, b) = if (0.0..60.0).contains(&h) {
            (c, x, 0.0)
        } else if (60.0..120.0).contains(&h) {
            (x, c, 0.0)
        } else if (120.0..180.0).contains(&h) {
            (0.0, c, x)
        } else if (180.0..240.0).contains(&h) {
            (0.0, x, c)
        } else if (240.0..300.0).contains(&h) {
            (x, 0.0, c)
        } else if (300.0..360.0).contains(&h) {
            (c, 0.0, x)
        } else {
            (0.0, 0.0, 0.0)
        };
        [
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
        ]
    }

    #[test]
    fn sender_recver() {
        let (sender, receiver) = mpsc::channel();
        sender.send(123).unwrap();
        sender.send(456).unwrap();
        sender.send(789).unwrap();

        println!("{:?},{:?}", receiver.recv(), receiver.recv());
    }

    use level_interface;
    #[test]
    fn lib_loader() {
        let p = level_interface::MyInterface::from_lib_safe("testlevel.dll".to_string());
        println!("{:#?}", p);
        let p =p.unwrap();
        let s = (p.new)();
        println!("{:#?}", s);
        (p.show)(s);
        (p.append)(s, "，");
        (p.show)(s);
        (p.append)(s, "世界！");
        (p.show)(s);
        (p.destory)(s);
    }
}
