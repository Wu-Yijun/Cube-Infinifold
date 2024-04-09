use std::sync::mpsc;

#[derive(Debug, PartialEq, Clone)]
pub struct MyGameOption {
    pub fullscreen: bool,
    pub screenshot: MyScreenShot,
    pub messages: MyMessage,
    pub time: std::time::Instant,
    pub dt: std::time::Duration,
}

impl Default for MyGameOption {
    fn default() -> Self {
        Self {
            fullscreen: false,
            screenshot: Default::default(),
            messages: MyMessage::default(),
            time: std::time::Instant::now(),
            dt: std::time::Duration::from_millis(1),
        }
    }
}

impl MyGameOption {}

#[derive(Debug, PartialEq, Clone)]
pub struct MyScreenShot {
    pub screen_shot: bool,
    pub screen_recording: bool,
    pub screen_recording_stop: bool,
}

impl Default for MyScreenShot {
    fn default() -> Self {
        Self {
            screen_shot: false,
            screen_recording: false,
            screen_recording_stop: false,
        }
    }
}

pub mod media {

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
        self.update();
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
