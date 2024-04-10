use eframe::{
    egui::{self, mutex::Mutex},
    egui_glow,
};
use std::sync::Arc;

use super::{
    gl_views::{GLFacesView, GLGameBase, GlPaintOptions},
    performance_evaluation::PerformanceEvaluation,
    MyViewImpl, UIWidget,
};

mod penrose_triangle;

pub struct MyGameView {
    game_view: Arc<Mutex<GLFacesView>>,
    angle: f32,
    perf: PerformanceEvaluation,
    btns: Vec<UIWidget>,
    change_to: Option<String>,
    // faces: Vec<items::Face>,
    level: penrose_triangle::PenroseTriangle,
}

impl MyGameView {
    pub fn new(game_view: Arc<Mutex<GLFacesView>>, ctx: &eframe::egui::Context) -> MyGameView {
        let btns = vec![UIWidget::new(vec![
            "file://assets/ui/unselected.png",
            "file://assets/ui/selected.png",
        ])
        .with_font(egui::Color32::GREEN, 28.0, egui::FontFamily::Proportional)
        .with_size(200.0, 50.0)
        .load(ctx)];
        let level = penrose_triangle::PenroseTriangle::new();
        game_view.lock().set_faces(level.get().clone());
        Self {
            game_view,
            angle: 0_f32.to_radians(),
            perf: PerformanceEvaluation::new(),
            btns: btns,
            change_to: None,
            level,
        }
    }

    fn paint_opengl(&mut self, ui: &mut egui::Ui) {
        let (rect, response) = ui.allocate_exact_size(ui.max_rect().size(), egui::Sense::drag());
        self.angle += response.drag_delta().x * 0.01;

        // if self.angle > std::f32::consts::PI * 2.0 {
        //     self.angle -= std::f32::consts::PI * 2.0;
        //     self.box_num += 1;
        //     self.game_view
        //         .lock()
        //         .set_lines(Self::get_box(self.box_num, 0.8, 0.8, 0.6));
        // } else if self.angle < 0.0 {
        //     self.angle += std::f32::consts::PI * 2.0;
        //     self.box_num -= if self.box_num <= 1 { 0 } else { 1 };
        //     self.game_view
        //         .lock()
        //         .set_lines(Self::get_box(self.box_num, 0.8, 0.8, 0.6));
        // }

        // Clone locals so we can move them into the paint callback:
        let angle = self.angle;
        if self.level.when_angled(angle) {
            self.game_view.lock().set_faces(self.level.get().clone());
        }
        // let angle = self.angle - (45.0 as f32).to_radians();
        // self.game_view
        //     .lock()
        //     .set_musk_enabled(angle < (45.0 as f32).to_radians());

        let game_view = self.game_view.clone();
        let option = GlPaintOptions {
            angle,
            scale: 0.05,
            aspect_ratio: rect.size().y / rect.size().x,
            ..Default::default()
        };

        let callback = egui::PaintCallback {
            rect,
            callback: std::sync::Arc::new(egui_glow::CallbackFn::new(move |_info, painter| {
                game_view.lock().paint(painter.gl(), &option);
            })),
        };
        ui.painter().add(callback);
    }
}

impl MyViewImpl for MyGameView {
    fn destory(&mut self) {
        // nothing todo!()
        self.btns.clear();
    }

    fn to_change(&self) -> Option<String> {
        match self.change_to.clone()?.as_str() {
            "Logo" | "Menu" => self.change_to.clone(),
            "Start" | "Game" => self.change_to.clone(),
            _ => None,
        }
    }

    fn paint(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        let gl_layer = egui::containers::Frame {
            fill: egui::Color32::WHITE,
            ..Default::default()
        };
        let layout_layers = egui::containers::Frame {
            fill: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 0),
            ..Default::default()
        };
        egui::CentralPanel::default()
            .frame(gl_layer)
            .show(ctx, |ui| {
                self.paint_opengl(ui);
            });
        egui::CentralPanel::default()
            .frame(layout_layers)
            .show(ctx, |ui| {
                if self.btns[0].button(ui, "返回", 0, 1).clicked() {
                    println!("返回");
                    self.change_to = Some(String::from("Menu"));
                }
                self.perf.performance_evaluation(ui, &frame);
            });
        ctx.input(|k| {
            if k.key_pressed(egui::Key::Escape) {
                std::process::exit(0);
            }
        });
        ctx.request_repaint();
    }
}
