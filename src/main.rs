use eframe::{
    egui,
    glow, icon_data,
};

mod my;
use my::{
    cube_infinifold_logo::MyInfinifoldLogo,
    gl_views::MyGLView,
    load_fonts::load_fonts,
    menu::MyMenu,
    MyView, MyViewImpl,
};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_fullscreen(true)
            .with_resizable(false)
            .with_fullsize_content_view(false)
            .with_icon(
                icon_data::from_png_bytes(include_bytes!("../assets/ferris.png"))
                    .expect("can not load file"),
            ),
        multisampling: 4,
        renderer: eframe::Renderer::Glow,
        depth_buffer: 24,
        ..Default::default()
    };

    eframe::run_native(
        "Custom 3D painting in eframe using glow",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}

struct MyApp {
    my_view: MyView,
    game_view: MyGLView,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        load_fonts(&cc.egui_ctx);
        let mut sf = Self {
            my_view: MyView::None,
            game_view: MyGLView::new(cc),
        };
        sf.change_to(String::from("Menu"), &cc.egui_ctx);
        sf
    }

    fn change_to(&mut self, name: String, ctx: &eframe::egui::Context) {
        match name.as_str() {
            "Logo" => {
                self.my_view.destory();
                self.my_view = MyView::MyLogo(MyInfinifoldLogo::new(self.game_view.lines.clone(), ctx));
            }
            "Menu" => {
                self.my_view.destory();
                self.my_view = MyView::MyMenu(MyMenu::new(self.game_view.basic.clone(), ctx));
            }
            "Start" => {
                self.my_view.destory();
                self.my_view = MyView::None;
                // self.my_view = MyView::MyLogo(MyInfinifoldLogo::new(self.game_view.clone(), ctx));
            }
            _ => (),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // self.menu_view(ctx, frame);
        match &mut self.my_view {
            MyView::MyMenu(v) => {
                v.paint(ctx, frame);
                if let Some(aim) = v.to_change() {
                    self.change_to(aim, ctx);
                }
            }
            MyView::MyLogo(v) => {
                v.paint(ctx, frame);
                if let Some(aim) = v.to_change() {
                    self.change_to(aim, ctx);
                }
            }
            _ => todo!("未完成！！！"),
        }
    }

    fn on_exit(&mut self, gl: Option<&glow::Context>) {
        if let Some(gl) = gl {
            self.game_view.destroy_all(gl);
        }
    }
}
