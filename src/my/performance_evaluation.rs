use core::convert::Into;
use std::{collections::VecDeque, time::Instant};

use eframe::egui;

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

#[derive(Default)]
pub struct PerformanceEvaluation {
    count: i32,
    last_time: Time,
    fps: f32,
    frame100: u128,
    frame99: u128,
    frames: i32,
    fps99: f32,
    fps100: f32,

    fps_list_size: usize,
    fps_list: VecDeque<f32>,
    max_shown_fps: f32,
}

impl PerformanceEvaluation {
    pub fn new() -> Self {
        Self {
            count: 100,
            fps_list_size: 500,
            max_shown_fps: 150.0,
            // max_shown_fps: 150.0,
            ..Default::default()
        }
    }
    pub fn performance_evaluation(self: &mut Self, ui: &mut egui::Ui) {
        let now = Instant::now();
        let dt = now - self.last_time.0;
        self.last_time = Time(now);

        let fm_time = dt.as_nanos();
        let fps = 1000000000.0 / (fm_time as f32);
        self.fps = self.fps * 0.9 + fps * 0.1;

        if self.frames < self.count {
            self.frames += 1;
            if fm_time > self.frame100 {
                self.frame99 = self.frame100;
                self.frame100 = fm_time;
            } else if fm_time > self.frame99 {
                self.frame99 = fm_time;
            }
        } else {
            self.frames = 0;
            self.fps100 = 1000000000.0 / (self.frame100 as f32);
            self.fps99 = 1000000000.0 / (self.frame99 as f32);
            self.frame100 = 0;
            self.frame99 = 0;
        }

        ui.group(|ui| {
            ui.label(format!("Frame time:{:0} ns ", fm_time));
            ui.label(format!("FPS:         {:.2} ", fps));
            ui.label(format!("FPS(stable): {:.2} ", self.fps));
            ui.label(format!("FPS-99:      {:.2} ", self.fps99));
            ui.label(format!("FPS-100:     {:.2} ", self.fps100));
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

                        self.fps_list.push_back(fps);
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
