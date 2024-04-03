use eframe::egui::mutex::Mutex;
use eframe::{egui, glow, icon_data};
use my::menu::MyMenu;
use my::MyView;
use my::MyViewImpl;
use std::sync::Arc;

mod my;
use my::{load_fonts::load_fonts,gl_game_view::GLGameView};

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
    game_view: Arc<Mutex<GLGameView>>,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        load_fonts(&cc.egui_ctx);
        let gl = cc
            .gl
            .as_ref()
            .expect("You need to run eframe with the glow backend");
        let game_view:Arc<Mutex<GLGameView>>= Mutex::new(GLGameView::new(gl)).into();
        Self {
            my_view: MyView::MyMenu(MyMenu::new(game_view.clone(), &cc.egui_ctx)),
            game_view: game_view,
        }
    }

}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // self.menu_view(ctx, frame);
        match &mut self.my_view {
            MyView::MyMenu(v) =>{
                v.paint(ctx, frame);
                if let Some(i) = v.to_change() {
                    // TODO change view
                }
            }
        }
    }

    fn on_exit(&mut self, gl: Option<&glow::Context>) {
        if let Some(gl) = gl {
            self.game_view.lock().destroy(gl);
        }
    }
}

impl MyApp {
    fn change_to(id: i32){
        
    }
}
