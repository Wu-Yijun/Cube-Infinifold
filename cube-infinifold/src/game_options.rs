use std::sync::mpsc;

use crate::my::performance_evaluation::PerformanceEvaluation;

#[derive(Debug, PartialEq, Clone)]
pub struct MyGameOption {
    pub fullscreen: bool,
    pub screenshot: MyScreenShot,
    pub messages: MyMessage,
    pub time: std::time::Instant,
    pub dt: std::time::Duration,
    pub cpu_useage: f32,
    
    pub performance_evaluation: PerformanceEvaluation,

    pub events: MyEvents,

    pub game_library: my_levels_finder::CollectedGame,
}

impl Default for MyGameOption {
    fn default() -> Self {
        Self {
            fullscreen: false,
            screenshot: Default::default(),
            messages: MyMessage::default(),
            time: std::time::Instant::now(),
            dt: std::time::Duration::from_millis(1),
            cpu_useage: 0.0,
            performance_evaluation: PerformanceEvaluation::new(),

            events: Default::default(),

            game_library: Self::load_levels(),
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
        // handler: JoinHandle<()>,
        // recvier: Receiver<Vec<u8>>,
        // handler: Receiver<Vec<u8>>,
        // / we will save the un encoded frames
        // pub video: VecDeque<Vec<u8>>,
        // pub audio: (),
    }
    impl PartialEq for Video {
        fn eq(&self, _: &Self) -> bool {
            true
        }
    }

    pub struct VideoFrame {
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
        ) -> Self {
            let (sender, receiver) = mpsc::channel::<VideoFrame>();

            // println!("{width},{height}");
            let settings = video_rs::encode::Settings::preset_h264_yuv420p(width, height, false);
            let mut encoder = video_rs::encode::Encoder::new(std::path::Path::new(path), settings)
                .expect("Cannot create");
            let path_s = path.to_string();
            thread::spawn(move || {
                let start_from = std::time::Instant::now();
                while let Ok(msg) = receiver.recv() {
                    let dt = msg.time_stamp - start_from;
                    let source_timestamp = video_rs::Time::from_secs_f64(dt.as_secs_f64());
                    // let rgb_data = msg.image.pixels.iter().map(|c|)
                    let shape = (msg.image.height(), msg.image.width(), 3);
                    let data = msg.image.as_raw();
                    let default = &0;
                    let frame: ndarray::Array3<u8> =
                        ndarray::Array3::from_shape_fn(shape, |(x, y, c)| {
                            *data.get((x * shape.1 + y) * 4 + c).unwrap_or(default)
                        });
                    encoder
                        .encode(&frame, &source_timestamp)
                        .expect("failed to encode frame");
                }
                encoder.finish().expect("failed to finish encoder");
                // println!("Finish writing!");
                msg_sender
                    .send((
                        format!("Screen Record {path_s} has been written into file successfully!"),
                        5000,
                    ))
                    .unwrap();
            });
            Self {
                width,
                height,
                path: path.to_string(),
                sender,
            }
        }

        // pub fn done(self) {
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
    /// mouse
    pub moved: (f32, f32),
    pub scrolled: (f32, f32),
    /// pointer state
    pub pos: (f32, f32),
    pub focused: bool,
    pub hovered: bool,

    pub pressed_l: bool,
    pub pressed_r: bool,
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
                        physical_key: _,
                        modifiers,
                        pressed: true,
                        ..
                    } => {
                        match key {
                            eframe::egui::Key::Space => self.space = true,
                            eframe::egui::Key::Enter => self.enter = true,
                            eframe::egui::Key::Escape => self.esc = true,
                            eframe::egui::Key::Tab => self.tab = true,
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
