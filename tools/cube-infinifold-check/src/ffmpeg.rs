pub fn main() {
    println!("Checking ffmpeg-loader...");

    let (msg_sender, msg_receiver) = std::sync::mpsc::channel::<(String, u64)>();
    let (info_sender, info_receiver) = std::sync::mpsc::channel::<(String, i64)>();
    let video = Video::new(1080, 1920, "test.mp4", msg_sender, info_sender);

    for i in 0..100 {
        let l = i as f32 / 100.0;
        let mut pixels = vec![];
        for y in 0..1080 {
            for x in 0..1920 {
                let a = x as f32 / 1920.0;
                let b = y as f32 / 1080.0;
                let r = l * a + (1.0 - l) * b;
                let g = l * b + (1.0 - l) * a;
                let b = l * (a + b) / 2.0;
                pixels.push(eframe::egui::Color32::from_rgb(
                    (r * 255.0) as u8,
                    (g * 255.0) as u8,
                    (b * 255.0) as u8,
                ));
            }
        }
        video
            .sender
            .send(VideoFrame {
                info: "frame".to_string(),
                image: eframe::egui::ColorImage {
                    size: [1920, 1080],
                    pixels: pixels,
                },
                audio: (),
                time_stamp: std::time::Instant::now()
                    .checked_add(std::time::Duration::from_millis(50 * i))
                    .unwrap(),
            })
            .unwrap();

        if let Ok(msg) = msg_receiver.try_recv() {
            println!("Msg: {:?}", msg);
        }
    }
    video.fin();

    let msg = info_receiver.recv().unwrap();
    println!("Info: {:?}", msg);

    println!("Checking ffmpeg-loader... OK");
}

use libloading::Library;
use std::sync::mpsc::{self, Sender};
use std::thread;

#[allow(dead_code)]
#[derive(Debug)]
struct Video {
    pub width: usize,
    pub height: usize,
    pub path: String,
    pub sender: Sender<VideoFrame>,
    pub ready: bool,
}
impl PartialEq for Video {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

#[allow(dead_code)]
struct VideoFrame {
    pub info: String,
    pub image: eframe::egui::ColorImage,
    pub audio: (),
    pub time_stamp: std::time::Instant,
}

impl Video {
    pub fn new(
        height: usize,
        width: usize,
        path: &str,
        msg_sender: Sender<(String, u64)>,
        info_sender: Sender<(String, i64)>,
    ) -> Self {
        let lib = load_lib("videosaver").unwrap();
        let (init, add_frame, finish, hello): (
            fn(usize, usize, String),
            fn(ndarray::Array3<u8>, f64),
            fn(),
            fn(),
        ) = unsafe {
            (
                *lib.get::<fn(usize, usize, String)>(b"new\0").unwrap(),
                *lib.get::<fn(ndarray::Array3<u8>, f64)>(b"add_frame\0")
                    .unwrap(),
                *lib.get::<fn()>(b"finish\0").unwrap(),
                *lib.get::<fn()>(b"hello\0").unwrap(),
            )
        };
        init(width, height, path.to_string());
        hello();

        let (sender, receiver) = mpsc::channel::<VideoFrame>();
        let path_s = path.to_string();
        thread::spawn(move || {
            let start_from = std::time::Instant::now();
            while let Ok(msg) = receiver.recv() {
                if msg.info == "stop" {
                    break;
                }
                let dt = (msg.time_stamp - start_from).as_secs_f64();

                let shape = (height, width, 3);
                let data = msg.image.as_raw();
                let default = &0;
                let frame: ndarray::Array3<u8> =
                    ndarray::Array3::from_shape_fn(shape, |(y, x, c)| {
                        *data.get((y * shape.1 + x) * 4 + c).unwrap_or(default)
                    });
                add_frame(frame, dt);
            }

            println!("Finish writing!");
            finish();
            msg_sender
                .send((
                    format!("Screen Record {path_s} has been written into file successfully!"),
                    5000,
                ))
                .unwrap();
            lib.close().unwrap();
            info_sender
                .send(("recording ready".to_string(), 0))
                .unwrap();
        });
        Self {
            width,
            height,
            path: path.to_string(),
            sender,
            ready: false,
        }
    }

    pub fn fin(&self) {
        self.sender
            .send(VideoFrame {
                info: "stop".to_string(),
                image: eframe::egui::ColorImage::default(),
                audio: (),
                time_stamp: std::time::Instant::now(),
            })
            .unwrap();
    }
}

#[cfg(target_os = "windows")]
const PLATFORM: &str = "Windows";
#[cfg(target_os = "linux")]
const PLATFORM: &str = "Linux";
#[cfg(target_os = "macos")]
const PLATFORM: &str = "MacOS";

fn load_lib(name: &str) -> Option<Library> {
    let libname = match PLATFORM {
        "Windows" => format!("{}.dll", name),
        "Linux" => format!("lib{}.so", name),
        "MacOS" => format!("lib{}.dylib", name),
        _ => return None,
    };
    let cur_path = std::env::current_dir().ok()?;
    let path_exe = std::env::current_exe().ok()?;
    let path = path_exe.ancestors().nth(1)?;
    let path = format!("{}/libs", path.display());
    println!("Loading lib from: {}", path);
    std::env::set_current_dir(path).ok()?;
    let lib = unsafe { libloading::Library::new(libname).ok() };
    std::env::set_current_dir(cur_path).ok()?;
    lib
}
