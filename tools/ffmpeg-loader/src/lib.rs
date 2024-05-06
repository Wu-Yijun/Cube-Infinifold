use ndarray::Array3;
use std::env;
use video_rs::encode::{Encoder, Settings};

static mut ENCODER: Option<&mut Encoder> = None;

#[no_mangle]
pub fn new(width: usize, height: usize, name: String) {
    let path_exe = env::current_exe().unwrap();
    let path = path_exe.ancestors().nth(1).unwrap();
    let out_path = path.join(&name);
    env::set_current_dir(path).unwrap();

    println!("Output path: {:?}", out_path);

    video_rs::init().unwrap();

    let settings = Settings::preset_h264_yuv420p(width, height, false);
    let encoder = Box::new(Encoder::new(out_path, settings).expect("failed to create encoder"));
    unsafe { ENCODER = Some(Box::leak(encoder)) };
}

#[no_mangle]
pub fn add_frame(frame: Array3<u8>, dt: f64) {
    println!("Adding frame at time: {dt}");
    let source_timestamp = video_rs::Time::from_secs_f64(dt);
    unsafe {
        match ENCODER {
            Some(ref mut encoder) => encoder
                .encode(&frame, &source_timestamp)
                .expect("failed to encode frame"),
            None => println!("No encoder found"),
        }
    };
}

#[no_mangle]
pub fn finish() {
    println!("Finishing encoder");
    unsafe {
        match ENCODER {
            Some(ref mut encoder) => encoder.finish().expect("failed to finish encoder"),
            None => println!("No encoder found"),
        }
    };
}

#[no_mangle]
pub fn hello() {
    println!("Hello ffmpeg-loader(video-encoder)!");
}