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
}

impl MyInfinifoldLogo {
    pub fn new(game_view: Arc<Mutex<GLGameView>>, ctx: &eframe::egui::Context) -> Self {
        let btns = vec![
            UIWidget::new(vec![
                "file://assets/ui/unselected.png",
                "file://assets/ui/selected.png",
            ])
            .load(ctx),
            UIWidget::new(vec![
                "file://assets/ui/unselected.png",
                "file://assets/ui/selected.png",
            ]),
            UIWidget::new(vec![
                "file://assets/ui/unselected.png",
                "file://assets/ui/selected.png",
            ]),
        ];
        Self {
            game_view: game_view,
            angle: 0.0,
            perf: PerformanceEvaluation::new(),
            btns: btns,
        }
    }

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
    fn destory(self) {
        todo!()
    }

    fn to_change(&self) -> Option<i32> {
        None
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
                if self.btns[0].button(ui, "开始游戏", 0, 1).clicked() {
                    println!("开始游戏")
                }
                if self.btns[1].button(ui, "Test1", 0, 1).clicked() {
                    println!("Tst 1 clked");
                }
                if self.btns[2].button(ui, "Test2", 0, 1).double_clicked() {
                    println!("Tst 2 db clked");
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
