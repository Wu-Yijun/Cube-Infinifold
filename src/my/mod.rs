use eframe::{
    self,
    egui,
};

// use self::menu::MyMenu;
pub mod cube_infinifold_logo;
pub mod gl_game_view;
pub mod load_fonts;
pub mod menu;
pub mod performance_evaluation;

pub trait MyViewImpl {
    fn destory(&mut self);
    fn paint(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame);
    fn to_change(&self) -> Option<String>;
}

pub enum MyView {
    MyMenu(menu::MyMenu),
    MyLogo(cube_infinifold_logo::MyInfinifoldLogo),
    None,
}

impl MyView {
    pub fn destory(&mut self) {
        match self {
            MyView::MyMenu(m) => m.destory(),
            MyView::MyLogo(m) => m.destory(),
            _ => (),
        }
    }
}

pub struct UIWidget {
    pub default_img: String,
    imgs: Vec<String>,
    imgs_ids: Vec<egui::TextureId>,
    pub x: f32,
    pub y: f32,
    pub font_color: egui::Color32,
    pub font_id: egui::FontId,
}

impl UIWidget {
    fn new(imgs: Vec<&'_ str>) -> Self {
        // initial imgs
        Self {
            default_img: String::from("file://assets/ui/selected.png"),
            imgs: imgs.iter().map(|s| String::from(*s)).collect(),
            x: 100.0,
            y: 50.0,
            font_color: egui::Color32::BLACK,
            font_id: egui::FontId {
                size: 24.0,
                family: egui::FontFamily::Proportional,
            },
            imgs_ids: vec![],
        }
    }
    // eframe:
    fn load(mut self, ctx: &egui::Context) -> Self {
        self.imgs_ids = self
            .imgs
            .iter()
            .filter_map(|s| {
                egui::Image::new(s)
                    .load_for_size(ctx, egui::vec2(self.x, self.y))
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
        color: egui::Color32,
        size: f32,
        family: egui::FontFamily,
    ) -> Self {
        self.font_color = color;
        self.font_id = egui::FontId { size, family };
        self
    }

    fn button(
        &self,
        ui: &mut egui::Ui,
        text: &str,
        img_id: usize,
        hov_id: usize,
    ) -> egui::Response {
        let (rect, response) = ui.allocate_exact_size(
            egui::Vec2 { x: 100.0, y: 50.0 },
            egui::Sense {
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
            egui::Image::new(img_str).paint_at(ui, rect);
        } else {
            let default_img = egui::TextureId::default();
            let texture = if response.hovered() {
                self.imgs_ids.get(hov_id).unwrap_or(&default_img)
            } else {
                self.imgs_ids.get(img_id).unwrap_or(&default_img)
            };
            ui.painter().image(
                *texture,
                rect,
                egui::Rect::from_min_max(
                    egui::Pos2::new(0.0, 0.0),
                    egui::Pos2::new(1.0, 1.0),
                ),
                egui::Color32::WHITE,
            );
        }
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            self.font_id.clone(),
            self.font_color,
        );
        response
    }
}
