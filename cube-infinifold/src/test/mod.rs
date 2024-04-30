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

    #[test]
    fn sender_recver() {
        let (sender, receiver) = mpsc::channel();
        sender.send(123).unwrap();
        sender.send(456).unwrap();
        sender.send(789).unwrap();

        println!("{:?},{:?}", receiver.recv(), receiver.recv());
    }

    
    // // now we have chaned to levels_interface
    // use level_interface;
    // #[test]
    // fn lib_loader() {
    //     // panic in outer thread can not be catched

    //     // let q = std::panic::catch_unwind(||level_interface::MyInterface::from_lib_safe("testlevel.dll".to_string()));
    //     // if q.is_err(){
    //     //     return;
    //     // }
    //     // let p = q.unwrap();
    //     let p = level_interface::MyInterface::from_lib_safe("testlevel.dll".to_string());
    //     println!("{:#?}", p);
    //     if p.is_err(){
    //         return;
    //     }
    //     let p =p.unwrap();
    //     let s = (p.new)();
    //     println!("{:#?}", s);
    //     (p.show)(s);
    //     (p.append)(s, "，");
    //     (p.show)(s);
    //     (p.append)(s, "世界！");
    //     (p.show)(s);
    //     (p.destory)(s);
    // }

    #[test]
    fn thread_test(){
        let hand = thread::spawn(||{
            panic!("I hate it.");
        });
        if hand.join().is_err() {
            return;
        }
        println!("Success")
    }
}
