use std::default;

use eframe::{
    self,
    egui::{vec2, Context, TextureId},
};
pub mod menu;
pub mod cube_infinifold_logo;
pub trait MyViewImpl {
    // fn new() -> Self;
    fn destory(self);
    fn paint(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame);
    fn to_change(&self) -> Option<i32>;
}

pub enum MyView {
    MyMenu(menu::MyMenu),
}

pub struct UIWidget {
    pub default_img: String,
    imgs: Vec<String>,
    imgs_ids: Vec<eframe::egui::TextureId>,
    pub x: f32,
    pub y: f32,
    pub font_color: eframe::egui::Color32,
    pub font_id: eframe::egui::FontId,
}

impl UIWidget {
    fn new(imgs: Vec<&'_ str>) -> Self {
        // initial imgs
        Self {
            default_img: String::from("file://assets/ui/selected.png"),
            imgs: imgs.iter().map(|s| String::from(*s)).collect(),
            x: 100.0,
            y: 50.0,
            font_color: eframe::egui::Color32::BLACK,
            font_id: eframe::egui::FontId {
                size: 24.0,
                family: eframe::egui::FontFamily::Proportional,
            },
            imgs_ids: vec![],
        }
    }
    // eframe:
    fn load(mut self, ctx: &Context) -> Self {
        self.imgs_ids = self
            .imgs
            .iter()
            .filter_map(|s| {
                eframe::egui::Image::new(s)
                    .load_for_size(ctx, vec2(self.x, self.y))
                    .ok()
                    .and_then(|ld| ld.texture_id())
            })
            .collect();
        self
    }

    fn with_size(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }
    fn with_font(
        mut self,
        color: eframe::egui::Color32,
        size: f32,
        family: eframe::egui::FontFamily,
    ) -> Self {
        self.font_color = color;
        self.font_id = eframe::egui::FontId { size, family };
        self
    }

    fn button(
        &self,
        ui: &mut eframe::egui::Ui,
        text: &str,
        img_id: usize,
        hov_id: usize,
    ) -> eframe::egui::Response {
        let (rect, response) = ui.allocate_exact_size(
            eframe::egui::Vec2 { x: 100.0, y: 50.0 },
            eframe::egui::Sense {
                click: true,
                drag: false,
                focusable: false,
            },
        );
        if self.imgs_ids.is_empty() {
            let img_str = if response.hovered() {
                self.imgs.get(hov_id).unwrap_or(&self.default_img)
            } else {
                self.imgs.get(img_id).unwrap_or(&self.default_img)
            };
            eframe::egui::Image::new(img_str).paint_at(ui, rect);
        } else {
            let default_img = eframe::egui::TextureId::default();
            let texture = if response.hovered() {
                self.imgs_ids
                    .get(hov_id)
                    .unwrap_or(&default_img)
            } else {
                self.imgs_ids
                    .get(img_id)
                    .unwrap_or(&default_img)
            };
            ui.painter().image(
                *texture,
                rect,
                eframe::egui::Rect::from_min_max(
                    eframe::egui::Pos2::new(0.0, 0.0),
                    eframe::egui::Pos2::new(1.0, 1.0),
                ),
                eframe::egui::Color32::WHITE,
            );
        }
        ui.painter().text(
            rect.center(),
            eframe::egui::Align2::CENTER_CENTER,
            text,
            self.font_id.clone(),
            self.font_color,
        );
        response
    }
}

pub mod gl_game_view;
pub mod load_fonts;
pub mod performance_evaluation;
