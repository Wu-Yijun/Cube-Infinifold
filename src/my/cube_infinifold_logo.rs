use std::sync::Arc;

use eframe::{
    egui::{self, mutex::Mutex},
    egui_glow,
};

use super::{
    gl_game_view::GLGameView, performance_evaluation::PerformanceEvaluation, MyViewImpl, UIWidget,
};

pub struct MyInfinifoldLogo {
    /// Behind an `Arc<Mutex<…>>` so we can pass it to [`egui::PaintCallback`] and paint later.
    game_view: Arc<Mutex<GLGameView>>,
    angle: f32,

    perf: PerformanceEvaluation,

    btns: Vec<UIWidget>,

    change_to: Option<String>,
}

impl MyInfinifoldLogo {

    fn paint_opengl(&mut self, ui: &mut egui::Ui) {
        let (rect, response) = ui.allocate_exact_size(ui.max_rect().size(), egui::Sense::drag());
        self.angle += response.drag_delta().x * 0.01;

        // Clone locals so we can move them into the paint callback:
        let angle = self.angle;
        let game_view = self.game_view.clone();

        let callback = egui::PaintCallback {
            rect,
            callback: std::sync::Arc::new(egui_glow::CallbackFn::new(move |_info, painter| {
                game_view.lock().paint(painter.gl(), &rect, angle);
            })),
        };
        ui.painter().add(callback);
    }
}

impl MyViewImpl for MyInfinifoldLogo {
    fn new(game_view: Arc<Mutex<GLGameView>>, ctx: &eframe::egui::Context) -> MyInfinifoldLogo {
        let btns = vec![UIWidget::new(vec![
            "file://assets/ui/unselected.png",
            "file://assets/ui/selected.png",
        ])
        .load(ctx)];
        Self {
            game_view: game_view,
            angle: 0.0,
            perf: PerformanceEvaluation::new(),
            btns: btns,
            change_to: None,
        }
    }

    fn destory(&mut self) {
        // nothing todo!()
        self.btns.clear();
    }

    fn to_change(&self) -> Option<String> {
        match self.change_to.clone()?.as_str() {
            "Logo"|"Menu" => self.change_to.clone(),
            _ => None,
        }
    }

    fn paint(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
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
                self.perf.performance_evaluation(ui);
            });
        ctx.input(|k| {
            if k.key_pressed(egui::Key::Escape) {
                std::process::exit(0);
            }
        });
        ctx.request_repaint();
    }
}
