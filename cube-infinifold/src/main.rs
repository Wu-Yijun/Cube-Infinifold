use eframe::{egui, glow, icon_data};

mod game_options;
mod my;
mod test;
use game_options::{media, MyGameOption};
use my::{
    cube_infinifold_logo::MyInfinifoldLogo, game::MyGameView, gl_views::MyGLView,
    level_index::MyLevelIndex, load_fonts::load_fonts, menu::MyMenu, MyView, MyViewImpl,
};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_fullscreen(true)
            // .with_inner_size(egui::Vec2::new(800.0/1.25, 600.0/1.25))
            .with_resizable(false)
            .with_fullsize_content_view(false)
            .with_icon(
                icon_data::from_png_bytes(include_bytes!("../../assets/ferris.png"))
                    .expect("can not load file"),
            ),
        multisampling: 4,
        renderer: eframe::Renderer::Glow,
        depth_buffer: 24,
        ..Default::default()
    };

    // println!("{:#?}", options.viewport);

    eframe::run_native(
        "Custom 3D painting in eframe using glow",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}

struct MyApp {
    my_view: MyView,
    game_view: MyGLView,
    option: MyGameOption,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        load_fonts(&cc.egui_ctx);
        video_rs::init().unwrap();
        let mut sf = Self {
            my_view: MyView::None,
            game_view: MyGLView::new(cc),
            option: Default::default(),
        };
        sf.change_to(String::from("Menu"), &cc.egui_ctx);
        sf
    }

    fn change_to(&mut self, name: String, ctx: &eframe::egui::Context) {
        match name.as_str() {
            "Logo" => {
                self.my_view.destory();
                self.my_view =
                    MyView::MyLogo(MyInfinifoldLogo::new(self.game_view.lines.clone(), ctx));
            }
            "Menu" => {
                self.my_view.destory();
                self.my_view = MyView::MyMenu(MyMenu::new(self.game_view.basic.clone(), ctx));
            }
            "Select Group" => {
                self.my_view.destory();
                self.my_view = MyView::MyLevels(MyLevelIndex::new(ctx));
            }
            "Select Level" | "Start" | "Game" => {
                self.my_view.destory();
                let view = MyGameView::new(self.game_view.faces.clone(), ctx);
                if view.is_none() {
                    return self.change_to("Menu".to_string(), ctx);
                }
                self.my_view = MyView::MyGame(view.unwrap());
            }
            "Exit" => {
                self.my_view.destory();
                std::process::exit(0);
            }
            _ => (),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        {
            let t = std::time::Instant::now();
            let dt = t - self.option.time;
            let cpu_useage = frame.info().cpu_usage.unwrap_or(0.0);
            self.option.performance_evaluation.evaluate(dt, cpu_useage);
            self.option.time = t;
            self.option.cpu_useage = cpu_useage;
            self.option.dt = dt;

            self.option.messages.recv();
            self.option.messages.dt(self.option.dt);

            self.option.events.get(ctx);
        }
        // eve handler
        match ctx.input(|i| {
            for event in &i.events {
                match event {
                    egui::Event::Key {
                        key: egui::Key::F2,
                        pressed: true,
                        modifiers:
                            egui::Modifiers {
                                ctrl: true,
                                shift: false,
                                alt: false,
                                ..
                            },
                        repeat: _,
                        physical_key: _,
                    } => {
                        return "screen record";
                    }
                    egui::Event::Key {
                        key: egui::Key::F2,
                        pressed: true,
                        modifiers: egui::Modifiers::NONE,
                        repeat: _,
                        physical_key: _,
                    } => {
                        return "screen shot";
                    }
                    egui::Event::Key {
                        key: egui::Key::T,
                        pressed: true,
                        modifiers: egui::Modifiers::NONE,
                        repeat: false,
                        physical_key: _,
                    } => {
                        return "text input";
                    }
                    _ => (),
                }
            }
            ""
        }) {
            "screen record" => {
                let msg = if self.option.screenshot.screen_recording {
                    self.option.screenshot.screen_recording_stop = true;
                    "Stop screen recording.".to_string()
                } else {
                    self.option.screenshot.screen_recording = true;

                    // let rect = ctx
                    //     .input(|i| i.viewport().outer_rect)
                    //     .unwrap_or(ctx.screen_rect());
                    let rect = ctx.screen_rect();
                    self.option.screenshot.video_encoder = Some(media::Video::new(
                        (rect.height() * ctx.pixels_per_point()).round() as usize,
                        (rect.width() * ctx.pixels_per_point()).round() as usize,
                        "output/video.mp4",
                        self.option.messages.send.clone(),
                    ));
                    "Ready for screen recording...".to_string()
                };
                let _ = self.option.messages.send.send((msg, 2000));
            }
            "screen shot" => {
                self.option.screenshot.screen_shot = true;
            }
            "text input" => {
                self.option.messages.expanded = !self.option.messages.expanded;
            }
            _ => (),
        }

        // paint all
        egui::CentralPanel::default().show(ctx, |ui| {
            // View show
            match &mut self.my_view {
                MyView::MyMenu(v) => {
                    v.paint(ui, &self.option);
                    if let Some(aim) = v.to_change() {
                        self.change_to(aim, ctx);
                    }
                }
                MyView::MyLogo(v) => {
                    v.paint(ui, &self.option);
                    if let Some(aim) = v.to_change() {
                        self.change_to(aim, ctx);
                    }
                }
                MyView::MyGame(v) => {
                    v.paint(ui, &self.option);
                    if let Some(aim) = v.to_change() {
                        self.change_to(aim, ctx);
                    }
                }
                MyView::MyLevels(v) => {
                    v.paint(ui, &self.option);
                    if let Some(aim) = v.to_change() {
                        self.change_to(aim, ctx);
                    }
                }
                _ => todo!("未完成！！！"),
            };
            self.option.performance_evaluation.draw(ui);
        });
        ctx.request_repaint();

        // message
        if self.option.messages.has_any() || self.option.messages.expanded {
            egui::TopBottomPanel::bottom(egui::Id::new("msgbox"))
                .frame(egui::containers::Frame {
                    fill: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 100),
                    outer_margin: egui::Margin {
                        left: 10.0,
                        right: 10.0,
                        top: 20.0,
                        bottom: 100.0,
                    },
                    inner_margin: egui::Margin::same(10.0),
                    rounding: egui::Rounding::ZERO,
                    shadow: eframe::epaint::Shadow::NONE,
                    stroke: eframe::epaint::Stroke::NONE,
                })
                .show_separator_line(false)
                .min_height(1.0)
                .show(ctx, |ui| {
                    let msg = if self.option.messages.expanded {
                        &self.option.messages.msg
                    } else {
                        self.option.messages.get()
                    };
                    for s in msg {
                        ui.label(s.0.clone());
                    }
                });
        }

        if self.option.screenshot.screen_shot || self.option.screenshot.screen_recording {
            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot);
        }

        // screen shot and record
        if self.option.screenshot.screen_shot || self.option.screenshot.screen_recording {
            // notice that the command will not always be executed
            if let Some(image) = ctx.input(|i| {
                for event in &i.raw.events {
                    if let egui::Event::Screenshot { image, .. } = event {
                        return Some(image.clone());
                    }
                }
                None
            }) {
                if self.option.screenshot.screen_shot {
                    self.option.screenshot.screen_shot = false;
                    let t = chrono::offset::Local::now().to_string().replace(":", "_");
                    let sender = self.option.messages.send.clone();
                    let name = format!("output/img-{t}");
                    sender
                        .clone()
                        .send((format!("{name}.png captured. Saving"), 500))
                        .unwrap();
                    std::thread::spawn(move || {
                        if let Some(err) = media::save_image(&name, "png", &image) {
                            println!("Cannot save image! Error: {err}");
                        } else {
                            let _ = sender.send((format!("{name}.png saved successfully"), 2000));
                        }
                    });
                } else if self.option.screenshot.screen_recording {
                    // todo!("Add image");
                    let img: &eframe::egui::ColorImage = &image;
                    if let Some(s) = &self.option.screenshot.video_encoder {
                        let frame = media::VideoFrame {
                            image: img.clone(),
                            audio: (),
                            time_stamp: std::time::Instant::now(),
                        };
                        s.sender.send(frame).unwrap();

                        if self.option.screenshot.screen_recording_stop {
                            self.option.screenshot.screen_recording = false;
                            self.option.screenshot.screen_recording_stop = false;
                            // s.done();
                            self.option.screenshot.video_encoder = None;
                        }
                    }
                }
            }
        }
    }

    fn on_exit(&mut self, gl: Option<&glow::Context>) {
        if let Some(gl) = gl {
            self.game_view.destroy_all(gl);
        }
    }
}
