use std::sync::mpsc;

use crate::my::game::game_info::MyGameInfo;
use crate::my::performance_evaluation::PerformanceEvaluation;

#[derive(Debug)]
pub struct GLobalMessage {
    pub sender: mpsc::Sender<(String, i64)>,
    receiver: Option<mpsc::Receiver<(String, i64)>>,
}
impl Default for GLobalMessage {
    fn default() -> Self {
        Self::new()
    }
}
impl PartialEq for GLobalMessage {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
impl Clone for GLobalMessage {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            receiver: None,
        }
    }
}
impl GLobalMessage {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            sender,
            receiver: Some(receiver),
        }
    }

    #[allow(dead_code)]
    pub fn send0(&self, msg: String) {
        self.sender.send((msg, 0)).unwrap();
    }
    #[allow(dead_code)]
    pub fn send(&self, msg: String, time: u32) {
        self.sender.send((msg, time as i64)).unwrap();
    }

    pub fn recv(&self) -> Vec<String> {
        if let Some(receiver) = &self.receiver {
            let mut msgs = vec![];
            let mut repeated = vec![];
            loop {
                match receiver.try_recv() {
                    Ok(msg) => {
                        if msg.1 > 0 {
                            repeated.push((msg.0.clone(), msg.1 - 1));
                        }
                        msgs.push(msg.0);
                    }
                    Err(_) => break,
                }
            }
            for msg in repeated {
                self.sender.send(msg).unwrap();
            }
            msgs
        } else {
            vec![]
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MyGameOption {
    pub global_message: GLobalMessage,

    pub fullscreen: bool,
    pub screenshot: MyScreenShot,
    pub messages: MyMessage,
    pub time: std::time::Instant,
    pub dt: std::time::Duration,
    pub cpu_useage: f32,

    pub performance_evaluation: PerformanceEvaluation,

    pub events: MyEvents,

    pub game_library: my_levels_finder::CollectedGame,
    pub game_info: MyGameInfo,
}

impl Default for MyGameOption {
    fn default() -> Self {
        Self {
            global_message: GLobalMessage::new(),

            fullscreen: false,
            screenshot: Default::default(),
            messages: MyMessage::default(),
            time: std::time::Instant::now(),
            dt: std::time::Duration::from_millis(1),
            cpu_useage: 0.0,
            performance_evaluation: PerformanceEvaluation::new(),

            events: Default::default(),

            game_library: Self::load_levels(),
            game_info: MyGameInfo::NONE,
        }
    }
}

impl MyGameOption {
    fn load_levels() -> my_levels_finder::CollectedGame {
        let path = my_levels_finder::Link::new("");
        const FILENAME: &str = "levels.json";
        my_levels_finder::get_levels(path, FILENAME)
    }
}

#[derive(Debug, PartialEq)]
pub struct MyScreenShot {
    pub screen_shot: bool,
    pub screen_recording: bool,
    pub screen_recording_stop: bool,

    pub video_encoder: Option<media::Video>,
}

impl Clone for MyScreenShot {
    fn clone(&self) -> Self {
        Self {
            screen_shot: self.screen_shot,
            screen_recording: self.screen_recording,
            screen_recording_stop: self.screen_recording_stop,
            video_encoder: None,
        }
    }
}

impl Default for MyScreenShot {
    fn default() -> Self {
        Self {
            screen_shot: false,
            screen_recording: false,
            screen_recording_stop: false,
            video_encoder: None,
        }
    }
}

pub mod media {
    use std::{
        sync::mpsc::{self, Sender},
        thread::{self},
    };

    /// return None if it is saved successfully.
    /// or else return Some(error: String)
    pub fn save_image(path: &str, ext: &str, img: &eframe::egui::ColorImage) -> Option<String> {
        let path = format!("{path}.{ext}");
        if let Err(err) = image::save_buffer(
            path,
            img.as_raw(),
            img.width() as u32,
            img.height() as u32,
            image::ColorType::Rgba8,
        ) {
            return Some(err.to_string());
        }
        None
    }

    #[derive(Debug)]
    pub struct Video {
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

    pub struct VideoFrame {
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
            let lib =
                unsafe { libloading::Library::new(super::get_lib_name("videosaver")).unwrap() };
            
            let init: fn(usize, usize, String) =
                *unsafe { lib.get::<fn(usize, usize, String)>(b"new\0").unwrap() };
            let add_frame: fn(ndarray::Array3<u8>, f64) = *unsafe {
                lib.get::<fn(ndarray::Array3<u8>, f64)>(b"add_frame\0")
                    .unwrap()
            };
            let finish: fn() = *unsafe { lib.get::<fn()>(b"finish\0").unwrap() };
            let hello: fn() = *unsafe { lib.get::<fn()>(b"hello\0").unwrap() };

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
                    // println!("Sending {dt} frame: {:?}", frame);
                    add_frame(frame, dt);
                    // println!("ok");
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

        // pub fn done(self) {
        //     self.video_lib.close().unwrap();
        //     drop(self.sender);
        //     self.handler.join().unwrap();
        // }
    }
}

#[derive(Debug)]
pub struct MyMessage {
    pub msg: Vec<(String, std::time::Duration)>,
    // pub msg_old: Vec<String>,
    pub send: mpsc::Sender<(String, u64)>,
    pub recv: Option<mpsc::Receiver<(String, u64)>>,

    pub expanded: bool,

    shown_message: Vec<(String, std::time::Duration)>,
    is_empty: bool,
    has_updated: bool,
}

impl Default for MyMessage {
    fn default() -> Self {
        let (send, recv) = mpsc::channel();
        Self {
            msg: vec![],
            send,
            recv: Some(recv),
            expanded: false,
            is_empty: true,
            has_updated: false,
            shown_message: vec![],
        }
    }
}
impl PartialEq for MyMessage {
    fn eq(&self, other: &Self) -> bool {
        if self.msg.len() != other.msg.len() {
            return false;
        }
        let mut m = self.msg.iter();
        let mut o = other.msg.iter();
        while let (Some(l), Some(r)) = (m.next(), o.next()) {
            if l.0 != r.0 {
                return false;
            }
        }
        true
    }
}
impl Clone for MyMessage {
    fn clone(&self) -> Self {
        Self {
            msg: self.msg.clone(),
            send: self.send.clone(),
            recv: None,
            expanded: self.expanded,
            shown_message: self.shown_message.clone(),
            is_empty: self.is_empty,
            has_updated: self.has_updated,
        }
    }
}

impl MyMessage {
    pub fn recv(&mut self) {
        if let Some(recv) = &self.recv {
            loop {
                let msg = recv.try_recv();
                if let Ok(msg) = msg {
                    self.msg
                        .push((msg.0, std::time::Duration::from_millis(msg.1)));
                } else {
                    break;
                }
            }
            self.update();
        }
    }
    pub fn dt(&mut self, dt: std::time::Duration) {
        let mut to_update = false;
        for m in self.msg.iter_mut() {
            if !m.1.is_zero() {
                if let Some(t) = m.1.checked_sub(dt) {
                    m.1 = t;
                } else {
                    m.1 = std::time::Duration::ZERO;
                    to_update = true;
                }
            }
        }
        if to_update {
            self.update();
        }
    }
    fn update(&mut self) {
        self.shown_message = self
            .msg
            .iter()
            .filter_map(|m| {
                if m.1.is_zero() {
                    None
                } else {
                    Some(m.to_owned())
                }
            })
            .collect();
        self.is_empty = self.shown_message.is_empty();
    }
    pub fn get(&self) -> &Vec<(String, std::time::Duration)> {
        &self.shown_message
    }
    pub fn has_any(&self) -> bool {
        !self.is_empty
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MyEvents {
    /// keybord
    pub ctrl: bool,
    /// keybord
    pub shift_l: bool,
    /// keybord **NOT IMPL**
    pub shift_r: bool,
    /// keybord
    pub alt: bool,
    /// keybord **NOT IMPL**
    pub caps: bool,

    /// keybord
    pub tab: bool,
    /// keybord
    pub esc: bool,
    /// keybord
    pub enter: bool,
    /// keybord
    pub space: bool,
    /// keybord/game-movement
    pub left: bool,
    /// keybord/game-movement
    pub right: bool,
    /// mouse
    pub moved: (f32, f32),
    pub scrolled: (f32, f32),
    /// pointer state
    pub pos: (f32, f32),
    pub focused: bool,
    pub hovered: bool,

    /// mouse left button
    pub pressed_l: bool,
    /// mouse right button
    pub pressed_r: bool,
    /// mouse middle button
    pub pressed_m: bool,
}

impl MyEvents {
    #[allow(dead_code)]
    pub fn reset_all(&mut self) {
        // Copy
        *self = Self::default();
    }
    pub fn reset(&mut self) {
        self.ctrl = false;
        self.shift_l = false;
        self.shift_r = false;
        self.alt = false;
        self.caps = false;
        self.tab = false;
        self.esc = false;
        self.enter = false;
        self.space = false;
        // self.pressed_l = false;
        // self.pressed_m = false;
        // self.pressed_r = false;
        self.moved = (0.0, 0.0);
        self.scrolled = (0.0, 0.0);

        self.left = false;
        self.right = false;
    }
    pub fn get(&mut self, ctx: &eframe::egui::Context) {
        self.reset();
        ctx.input(|r| {
            for e in &r.events {
                match e {
                    eframe::egui::Event::WindowFocused(f) => self.focused = *f,
                    eframe::egui::Event::PointerGone => self.hovered = false,
                    eframe::egui::Event::PointerMoved(p) => {
                        self.moved = (p.x - self.pos.0, p.y - self.pos.1);
                        self.pos = (p.x, p.y);
                        self.hovered = true;
                    }
                    eframe::egui::Event::PointerButton {
                        button: eframe::egui::PointerButton::Primary,
                        pressed,
                        ..
                    } => self.pressed_l = *pressed,
                    eframe::egui::Event::PointerButton {
                        button: eframe::egui::PointerButton::Secondary,
                        pressed,
                        ..
                    } => self.pressed_r = *pressed,
                    eframe::egui::Event::PointerButton {
                        button: eframe::egui::PointerButton::Middle,
                        pressed,
                        ..
                    } => self.pressed_m = *pressed,
                    eframe::egui::Event::MouseWheel {
                        unit: eframe::egui::MouseWheelUnit::Point,
                        delta,
                        ..
                    } => self.scrolled = (delta.x, delta.y),
                    eframe::egui::Event::Key {
                        key,
                        physical_key: p_key,
                        modifiers,
                        pressed: true,
                        ..
                    } => {
                        match key {
                            eframe::egui::Key::Space => self.space = true,
                            eframe::egui::Key::Enter => self.enter = true,
                            eframe::egui::Key::Escape => self.esc = true,
                            eframe::egui::Key::Tab => self.tab = true,
                            // ArrowLeft ArrowRight
                            eframe::egui::Key::ArrowLeft => self.left = true,
                            eframe::egui::Key::ArrowRight => self.right = true,
                            // A D
                            eframe::egui::Key::A => self.left = true,
                            eframe::egui::Key::D => self.right = true,
                            _ => (),
                        }
                        // A and D of the keyboard
                        match p_key {
                            Some(eframe::egui::Key::A) => self.left = true,
                            Some(eframe::egui::Key::D) => self.right = true,
                            _ => (),
                        }
                        self.alt |= modifiers.alt;
                        self.ctrl |= modifiers.ctrl;
                        self.shift_l |= modifiers.shift;

                        // match physical_key {
                        //     eframe::egui::Key::Enter => (),
                        //     _ => (),
                        // }
                    }
                    // eframe::egui::Event::AccessKitActionRequest(_) => (),
                    // eframe::egui::Event::Screenshot { .. } => (),
                    // eframe::egui::Event::Copy => (),
                    // eframe::egui::Event::Cut => (),
                    // eframe::egui::Event::Paste(_) => (),
                    // eframe::egui::Event::Text(_) => (),
                    // eframe::egui::Event::MouseMoved(_) => (),
                    // eframe::egui::Event::Touch { .. } => (),
                    // eframe::egui::Event::Zoom(_) => (),
                    // eframe::egui::Event::CompositionStart => (),
                    // eframe::egui::Event::CompositionUpdate(_) => (),
                    // eframe::egui::Event::CompositionEnd(_) => (),
                    _ => (),
                }
            }
        });
    }
}

/// input: name of the library e.g.
/// output: name of the library with extension based on the OS
/// e.g. "libs/add" -> "libs/add.dll" (windows)
/// e.g. "libs/add" -> "libs/libadd.so" (linux)
/// e.g. "libs/add" -> "libs/libadd.dylib" (macos)
fn get_lib_name(name: &str) -> String {
    #[cfg(target_os = "windows")]
    const OS: &str = "windows";
    #[cfg(target_os = "linux")]
    const OS: &str = "linux";
    #[cfg(target_os = "macos")]
    const OS: &str = "macos";

    let path = name.split("/").collect::<Vec<_>>();
    if path.len() == 1 {
        match OS {
            "windows" => format!("{}.dll", name),
            "linux" => format!("lib{}.so", name),
            "macos" => format!("lib{}.dylib", name),
            _ => {
                println!("Unsupported OS: {:?}", OS);
                name.to_string()
            }
        }
    } else {
        let name = path[path.len() - 1];
        let prefix = path[0..path.len() - 1].join("/");
        match OS {
            "windows" => format!("{}/{}.dll", prefix, name),
            "linux" => format!("{}/lib{}.so", prefix, name),
            "macos" => format!("{}/lib{}.dylib", prefix, name),
            _ => {
                println!("Unsupported OS: {:?}", OS);
                name.to_string()
            }
        }
    }
}
