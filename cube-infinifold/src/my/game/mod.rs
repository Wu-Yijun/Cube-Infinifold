use eframe::{
    egui::{self, mutex::Mutex},
    egui_glow,
};
use std::sync::Arc;

use crate::game_options::MyGameOption;

use super::{
    gl_views::{GLFacesView, GLGameBase, GlPaintOptions},
    MyViewImpl, UIWidget,
};

// mod penrose_triangle;
pub mod game_info;
mod load_level;

pub struct MyGameView {
    game_view: Arc<Mutex<GLFacesView>>,
    angle: f32,
    btns: Vec<UIWidget>,
    change_to: Option<String>,
    // faces: Vec<items::Face>,
    // level: penrose_triangle::PenroseTriangle,
    level: load_level::Level,
}

impl MyGameView {
    pub fn new(
        game_view: Arc<Mutex<GLFacesView>>,
        ctx: &eframe::egui::Context,
        option: &MyGameOption,
    ) -> Option<MyGameView> {
        let btns = vec![UIWidget::new(vec![
            "file://assets/ui/unselected.png",
            "file://assets/ui/selected.png",
        ])
        .with_font(egui::Color32::GREEN, 28.0, egui::FontFamily::Proportional)
        .with_size(200.0, 50.0)
        .load(ctx)];
        // let level = penrose_triangle::PenroseTriangle::new();
        let level = load_level::Level::new(option)?;
        game_view.lock().set_faces(level.get_faces().clone());
        Some(Self {
            game_view,
            angle: 0_f32.to_radians(),
            btns: btns,
            change_to: None,
            level,
        })
    }

    fn paint_opengl(&mut self, ui: &mut egui::Ui, option: &MyGameOption) {
        self.calc_angle(option);

        if self.level.when_angled(self.angle) {
            self.game_view
                .lock()
                .set_faces(self.level.get_faces().clone());
        }

        let game_view = self.game_view.clone();
        let option = GlPaintOptions {
            angle: self.angle,
            scale: 0.05,
            aspect_ratio: ui.max_rect().aspect_ratio(),
            ..Default::default()
        };

        let callback = egui::PaintCallback {
            rect: ui.max_rect(),
            callback: std::sync::Arc::new(egui_glow::CallbackFn::new(move |_info, painter| {
                game_view.lock().paint(painter.gl(), &option);
            })),
        };
        ui.painter().add(callback);
    }

    fn calc_angle(&mut self, option: &MyGameOption) {
        // if we use the mouse to control the angle
        // drag distance * 0.01 is the angle
        // however, we can use the keyboard to control the angle
        // when we press the key 'L', the angle will snap to previous 45 * n degree
        // when we press the key 'R', the angle will snap to next 45 * n degree
        let angle = self.angle.to_degrees();
        let angle = if option.events.pressed_l {
            angle + option.events.moved.0 * 0.05
        } else if option.events.left {
            ((angle - 23.0) / 45.0).round() * 45.0
        } else if option.events.right {
            ((angle + 23.0) / 45.0).round() * 45.0
        } else {
            angle
        };
        self.angle = angle.to_radians();
    }
}

impl MyViewImpl for MyGameView {
    fn destory(&mut self) {
        // nothing todo!()
        self.btns.clear();
    }

    fn to_change(&self, _option: &mut MyGameOption) -> Option<String> {
        match self.change_to.clone()?.as_str() {
            "Logo" | "Menu" | "Exit" => self.change_to.clone(),
            "Start" | "Game" => self.change_to.clone(),
            "Error" => Some("Menu".to_string()),
            s => {
                println!("Undefined Command:{s}");
                None
            }
        }
    }

    fn paint(&mut self, ui: &mut egui::Ui, option: &MyGameOption) {
        if !self.level.is_ok() {
            let _ = option
                .messages
                .send
                .send(("Errors in level!".to_string(), 2500));
            self.change_to = Some(String::from("Error"));
            return;
        }
        self.paint_opengl(ui, option);
        if self.btns[0].button(ui, "返回", 0, 1).clicked() {
            println!("返回");
            self.change_to = Some(String::from("Menu"));
        }
        // event handler
        if option.events.esc {
            self.change_to = Some(String::from(if option.events.shift_l {
                "Exit"
            } else {
                "Menu"
            }))
        }
    }
}
