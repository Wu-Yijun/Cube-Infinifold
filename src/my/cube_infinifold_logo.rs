use std::sync::Arc;

use eframe::{
    egui::{self, mutex::Mutex},
    egui_glow,
};
use rand::random;

use super::{
    gl_views::{items, GLGameBase, GLLinesView, GlPaintOptions},
    performance_evaluation::PerformanceEvaluation,
    MyViewImpl, UIWidget,
};

pub struct MyInfinifoldLogo {
    /// Behind an `Arc<Mutex<…>>` so we can pass it to [`egui::PaintCallback`] and paint later.
    // game_view: Arc<Mutex<GLLinesView>>,
    game_view: Arc<Mutex<GLLinesView>>,
    angle: f32,

    perf: PerformanceEvaluation,

    btns: Vec<UIWidget>,

    change_to: Option<String>,

    box_num: usize,
}

impl MyInfinifoldLogo {
    pub fn new(
        game_view: Arc<Mutex<GLLinesView>>,
        ctx: &eframe::egui::Context,
    ) -> MyInfinifoldLogo {
        let btns = vec![UIWidget::new(vec![
            "file://assets/ui/unselected.png",
            "file://assets/ui/selected.png",
        ])
        .with_font(egui::Color32::GREEN, 28.0, egui::FontFamily::Proportional)
        .with_size(200.0, 50.0)
        .load(ctx)];
        game_view.lock().set_lines(Self::get_box(3, 0.8, 0.8, 0.6));
        // game_view
        //     .lock()
        //     .add_lines(items::Line::default(0.0, 0.0, 0.0, 0.5, 0.5, 0.5));
        // game_view
        //     .lock()
        //     .add_lines(items::Line::default(0.0, 0.0, 0.0, 0.5, 0.0, 0.0));
        // game_view
        //     .lock()
        //     .add_lines(items::Line::default(0.0, 0.0, 0.0, 0.0, 0.5, 0.0));
        // game_view
        //     .lock()
        //     .add_lines(items::Line::default(0.0, 0.0, 0.0, 0.0, 0.0, 0.5));
        Self {
            game_view: game_view,
            angle: 0.0,
            perf: PerformanceEvaluation::new(),
            btns: btns,
            change_to: None,
            box_num: 3,
        }
    }

    fn paint_opengl(&mut self, ui: &mut egui::Ui) {
        let (rect, response) = ui.allocate_exact_size(ui.max_rect().size(), egui::Sense::drag());
        self.angle += response.drag_delta().x * 0.01;

        if self.angle > std::f32::consts::PI * 2.0 {
            self.angle -= std::f32::consts::PI * 2.0;
            self.box_num += 1;
            self.game_view
                .lock()
                .set_lines(Self::get_box(self.box_num, 0.8, 0.8, 0.6));
        } else if self.angle < 0.0 {
            self.angle += std::f32::consts::PI * 2.0;
            self.box_num -= if self.box_num <= 1 { 0 } else { 1 };
            self.game_view
                .lock()
                .set_lines(Self::get_box(self.box_num, 0.8, 0.8, 0.6));
        }

        // Clone locals so we can move them into the paint callback:
        let angle = self.angle - (45.0 as f32).to_radians();
        self.game_view
            .lock()
            .set_musk_enabled(angle < (45.0 as f32).to_radians());

        let game_view = self.game_view.clone();
        let option = GlPaintOptions {
            angle,
            scale: 0.1,
            aspect_ratio: rect.size().y / rect.size().x,
            ..Default::default()
        };

        let callback = egui::PaintCallback {
            rect,
            callback: std::sync::Arc::new(egui_glow::CallbackFn::new(move |_info, painter| {
                game_view.lock().paint(painter.gl(), &option);
            })),
        };
        // std::thread::spawn(f)
        ui.painter().add(callback);
    }

    fn get_box(dim: usize, scale: f32, init_offest: f32, offset_scale: f32) -> Vec<items::Line> {
        let mut ori: Vec<(Vec<f32>, Vec<isize>)> = vec![(vec![], vec![])];
        let mut times = 0;
        let mut len = 1;
        while times < dim {
            let mut res = ori.clone();
            if times >= 3 {
                for i in res.iter_mut() {
                    i.0.push(1.0);
                    i.0 = i.0.iter().map(|x| x * scale).collect();

                    i.1.push(-len);
                }
                for i in ori.iter_mut() {
                    i.0.push(0.0);
                }
            } else {
                for i in res.iter_mut() {
                    i.0.push(-1.0);

                    i.1.push(-len);
                }
                for i in ori.iter_mut() {
                    i.0.push(1.0);
                }
            }
            // res.append(&mut ori);
            ori.append(&mut res);

            times += 1;
            len *= 2;
        }

        let mut of_scale = init_offest;
        let mut dir: Vec<(f32, f32, f32)> = vec![
            (of_scale, 0.0, 0.0),
            (0.0, of_scale, 0.0),
            (0.0, 0.0, of_scale),
        ];
        while times > 3 {
            times -= 1;
            of_scale *= offset_scale;
            dir.push(loop {
                let x: f32 = random::<f32>() * 2.0 - 1.0;
                let y: f32 = random::<f32>() * 2.0 - 1.0;
                let z: f32 = random::<f32>() * 2.0 - 1.0;
                let r = (x * x + y * y + z * z).sqrt();
                if r <= 1.0 {
                    break (x / r * of_scale, y / r * of_scale, z / r * of_scale);
                }
            });
        }
        let proj = move |v: &Vec<f32>| -> Vec<f32> {
            let mut res = vec![0.0, 0.0, 0.0];
            let mut cnt = 0;
            for i in v.iter() {
                res[0] += i * dir[cnt].0;
                res[1] += i * dir[cnt].1;
                res[2] += i * dir[cnt].2;
                cnt += 1;
            }
            res
        };

        // Projection
        for i in ori.iter_mut() {
            i.0 = proj(&i.0);
        }

        // transform
        let mut res: Vec<items::Line> = vec![];
        let musk = Some(items::Musk {
            pos: items::V3 {
                x: init_offest + 0.01,
                y: 1.0,
                z: init_offest + 0.01,
            },
            dir: items::V3 {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
        });
        let mut times = 0;
        let cfun = items::Colored::Fun(Arc::new(|x| items::Color {
            r: 0.5,
            g: 0.2 * x as f32,
            b: 0.8,
            a: 1.0
        }));
        for i in ori.iter() {
            for j in i.1.iter() {
                res.push(items::Line {
                    pos1: items::V3 {
                        x: i.0[0],
                        y: i.0[1],
                        z: i.0[2],
                    },
                    pos2: items::V3 {
                        x: ori[(times + *j) as usize].0[0],
                        y: ori[(times + *j) as usize].0[1],
                        z: ori[(times + *j) as usize].0[2],
                    },
                    msk: if times < len / 2 { None } else { musk.clone() },
                    // color: items::Colored::Default,
                    color: cfun.clone(),
                })
            }
            times += 1;
        }

        // println!("{:#?}", res);

        res
    }
}

impl MyViewImpl for MyInfinifoldLogo {
    fn destory(&mut self) {
        // nothing todo!()
        self.btns.clear();
    }

    fn to_change(&self) -> Option<String> {
        match self.change_to.clone()?.as_str() {
            "Logo" | "Menu" => self.change_to.clone(),
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
