use core::convert::Into;
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use eframe::egui;

#[derive(Debug, PartialEq, Clone)]
struct Time(Instant);

impl Default for Time {
    fn default() -> Self {
        Time(Instant::now())
    }
}

impl Into<Instant> for Time {
    fn into(self) -> Instant {
        self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Fps99Finder {
    pub n: usize,
    pub index_99: usize,
    fps: Vec<f32>,
    fps2: Vec<f32>,
}

impl Fps99Finder {
    fn new(n: usize) -> Self {
        let size: usize = ((n + 150) / 100).max(2) as usize;
        let fps = Vec::with_capacity(size);
        let fps2 = Vec::new();
        Self {
            n,
            index_99: size,
            fps,
            fps2,
        }
    }
    fn push(&mut self, number: f32) {
        self.fps.push(number);
        if self.fps.len() >= self.n {
            // move the vec from fps to fps2 so that fps is cleared
            self.fps2 = self.fps.clone();
            self.fps.clear();
            // sort the fps2
            self.fps2.sort_by(|a, b| a.partial_cmp(b).unwrap());
        }
    }
    pub fn fps99(&self) -> f32 {
        *self.fps2.get(self.index_99).unwrap_or(&std::f32::NAN)
    }
    // a% of fps
    pub fn fps_a(&self, a: f32) -> f32 {
        let index: usize = (self.n as f32 * (1.0 - a / 100.0)) as usize;
        *self.fps2.get(index).unwrap_or(&std::f32::NAN)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PerformanceEvaluation {
    pub enable_calc: bool,
    pub cpu_useage: f32,
    pub frame_time: u128,
    pub fps: f32,
    pub fps_stable_ratio: f32,
    pub fps_stable: f32,
    pub fps99: f32,

    pub fps99_finder: Fps99Finder,

    pub enable_draw: bool,
    pub max_shown_fps: f32,
    pub fps_list_size: usize,
    pub fps_list: VecDeque<f32>,
}

impl PerformanceEvaluation {
    pub fn new() -> Self {
        let fps_list_size = 500;
        Self {
            enable_calc: false,
            frame_time: 0,
            fps: 0.0,
            fps99: 0.0,
            cpu_useage: 0.0,
            fps_stable_ratio: 0.05,
            fps_stable: 0.0,

            fps99_finder: Fps99Finder::new(500),

            enable_draw: false,
            max_shown_fps: 150.0,
            fps_list_size,
            fps_list: VecDeque::with_capacity(fps_list_size),
        }
    }
    pub fn evaluate(&mut self, dt: Duration, cpu_useage: f32) {
        self.cpu_useage = cpu_useage;
        self.frame_time = dt.as_nanos();
        self.fps = 1000000000.0 / (self.frame_time as f32);
        self.fps_stable =
            self.fps_stable * (1.0 - self.fps_stable_ratio) + self.fps * self.fps_stable_ratio;

        self.fps99_finder.push(self.fps);
        self.fps99 = self.fps99_finder.fps99();
    }
    pub fn draw(&mut self, ui: &mut egui::Ui) {
        // draw at right top
        let rect = ui.max_rect();
        let rect = egui::Rect::from_min_max(
            [rect.right() - 250.0, rect.top()].into(),
            rect.right_bottom(),
        );
        let ui = &mut ui.child_ui(rect, ui.layout().clone());
        ui.group(|ui| {
            ui.label(format!(
                "CPU cost:  {:0} ns ",
                (self.cpu_useage * 1_000_000_000.0) as u64
            ));
            ui.label(format!("Frame time:{:0} ns ", self.frame_time as u64));
            ui.label(format!("FPS:         {:.2} ", self.fps));
            ui.label(format!("FPS(stable): {:.2} ", self.fps_stable));
            ui.label(format!("FPS-99:      {:.2} ", self.fps99));
            ui.label(format!(
                "FPS-90:      {:.2} ",
                self.fps99_finder.fps_a(90.0)
            ));
            // let wid = ui.min_rect().width();

            let (width1, width2, height) = (30.0, 180.0, 90.0);
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.set_width(width1);
                    ui.set_height(height);
                    // ui.label("High");
                    // ui.label("Mid");
                    // ui.label("Low");
                });
                egui::Frame::canvas(ui.style())
                    .fill(egui::Color32::from_rgba_unmultiplied(255, 255, 255, 50))
                    .show(ui, |ui| {
                        ui.set_width(width2);
                        ui.set_height(height);
                        let scale_x = width2 / (self.fps_list_size as f32);
                        let offset_x = ui.min_rect().left();
                        let scale_y = height / self.max_shown_fps;
                        let offset_y = height + ui.min_rect().top();

                        self.fps_list.push_back(self.fps);
                        if self.fps_list.len() > self.fps_list_size {
                            self.fps_list.pop_front();
                        }
                        let mut avg = 0.0;
                        let points = self
                            .fps_list
                            .clone()
                            .into_iter()
                            .enumerate()
                            .map(|i| {
                                avg += i.1;
                                egui::pos2(
                                    offset_x + scale_x * i.0 as f32,
                                    offset_y - scale_y * i.1,
                                )
                            })
                            .collect();

                        ui.painter().add(egui::epaint::Shape::line(
                            points,
                            egui::Stroke::new(
                                1.0,
                                egui::Color32::from_rgba_unmultiplied(10, 10, 255, 150),
                            ),
                        ));
                        avg /= self.fps_list.len() as f32;

                        ui.painter().add(egui::epaint::Shape::line(
                            vec![
                                egui::pos2(offset_x + 0.0, offset_y - avg * scale_y),
                                egui::pos2(offset_x + width2, offset_y - avg * scale_y),
                            ],
                            egui::Stroke::new(
                                1.0,
                                egui::Color32::from_rgba_unmultiplied(255, 0, 0, 100),
                            ),
                        ));
                        ui.painter().add(egui::epaint::Shape::line(
                            vec![
                                egui::pos2(offset_x + 0.0, offset_y - self.fps99 * scale_y),
                                egui::pos2(offset_x + width2, offset_y - self.fps99 * scale_y),
                            ],
                            egui::Stroke::new(
                                1.0,
                                egui::Color32::from_rgba_unmultiplied(0, 255, 0, 100),
                            ),
                        ));

                        let font: egui::FontId = egui::FontId {
                            size: 10.0,
                            family: egui::FontFamily::Proportional,
                        };

                        ui.painter().text(
                            egui::pos2(offset_x - 5.0, offset_y - height),
                            egui::Align2::RIGHT_TOP,
                            format!("fps {}", self.max_shown_fps as i32),
                            font.clone(),
                            egui::Color32::from_rgba_unmultiplied(0, 0, 0, 130),
                        );
                        ui.painter().text(
                            egui::pos2(offset_x - 5.0, offset_y - avg * scale_y),
                            egui::Align2::RIGHT_CENTER,
                            format!("avg {}", avg as i32),
                            font.clone(),
                            egui::Color32::from_rgba_unmultiplied(100, 0, 0, 130),
                        );
                        ui.painter().text(
                            egui::pos2(offset_x - 5.0, offset_y - self.fps99 * scale_y),
                            egui::Align2::RIGHT_TOP,
                            format!("99% {}", self.fps99 as i32),
                            font.clone(),
                            egui::Color32::from_rgba_unmultiplied(0, 100, 0, 130),
                        );
                        ui.painter().text(
                            egui::pos2(offset_x - 5.0, offset_y),
                            egui::Align2::RIGHT_BOTTOM,
                            "0",
                            font,
                            egui::Color32::from_rgba_unmultiplied(0, 0, 0, 130),
                        );

                        // ui.painter().
                    });
            });
        });
    }
}
