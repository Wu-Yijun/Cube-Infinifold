use std::sync::Arc;

use eframe::{
    egui::{self, mutex::Mutex},
    egui_glow,
};

use crate::game_options::MyGameOption;

use super::{
    gl_views::{GLGameBase, GLGameView, GlPaintOptions},
    // performance_evaluation::PerformanceEvaluation,
    MyViewImpl,
    UIWidget,
};

pub struct MyMenu {
    game_view: Arc<Mutex<GLGameView>>,
    angle: f32,

    btns: Vec<UIWidget>,

    change_to: Option<String>,
}

impl MyMenu {
    pub fn new(game_view: Arc<Mutex<GLGameView>>, _ctx: &eframe::egui::Context) -> MyMenu {
        let btns = vec![
            UIWidget::new(vec![
                "file://assets/ui/unselected.png",
                "file://assets/ui/selected.png",
            ]),
            // .load(ctx),
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
            // perf: PerformanceEvaluation::new(),
            btns: btns,
            change_to: None,
        }
    }

    fn paint_opengl(&mut self, ui: &mut egui::Ui, option: &MyGameOption) {
        self.angle += if option.events.pressed_l {
            option.events.moved.0 * 0.01
        } else {
            0.0
        };
        let angle = self.angle;
        let game_view = self.game_view.clone();
        let option = GlPaintOptions {
            angle,
            scale: 1.0,
            aspect_ratio: ui.max_rect().aspect_ratio(),
            ..Default::default()
        };

        // Clone locals so we can move them into the paint callback:
        let callback = egui::PaintCallback {
            rect: ui.max_rect(),
            callback: std::sync::Arc::new(egui_glow::CallbackFn::new(move |_info, painter| {
                game_view.lock().paint(painter.gl(), &option);
            })),
        };
        ui.painter().add(callback);
    }
}

impl MyViewImpl for MyMenu {
    fn destory(&mut self) {
        // nothing todo!()
        self.btns.clear();
    }

    fn to_change(&self, _option: &mut MyGameOption) -> Option<String> {
        match self.change_to.clone()?.as_str() {
            "Logo" | "Start" | "Select Group" => self.change_to.clone(),
            "Exit" => self.change_to.clone(),
            s => {
                println!("Undefined Command:{s}");
                None
            }
        }
    }

    fn paint(&mut self, ui: &mut egui::Ui, option: &MyGameOption) {
        self.paint_opengl(ui, option);
        if self.btns[0].button(ui, "Cube Infinifold", 0, 1).clicked() {
            self.change_to = Some(String::from("Logo"));
        }
        if self.btns[1].button(ui, "加载游戏", 0, 1).clicked() {
            self.change_to = Some(String::from("Start"));
            println!("开始游戏");
        }
        if self.btns[2].button(ui, "选择关卡", 0, 1).clicked() {
            self.change_to = Some(String::from("Select Group"));
            println!("进入选择关卡菜单");
        }
        // event handler
        if option.events.esc {
            self.change_to = Some(String::from("Exit"));
        }
    }
}
