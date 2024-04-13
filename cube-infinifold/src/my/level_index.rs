use eframe::egui;

use crate::game_options::MyGameOption;

use super::{
    // performance_evaluation::PerformanceEvaluation,
    MyViewImpl,
    UIWidget,
};

#[derive(Debug, Clone)]
struct SelectedLevel {
    pub level: i64,
    pub group: i64,
}
#[derive(Debug, Clone)]
enum Selector {
    Select(SelectedLevel),
    State(String),
}

pub struct MyLevelIndex {
    btns: Vec<UIWidget>,

    change_to: Option<Selector>,
}

impl MyLevelIndex {
    pub fn new(_ctx: &eframe::egui::Context) -> Self {
        let btns = vec![UIWidget::new(vec![
            "file://assets/ui/unselected.png",
            "file://assets/ui/selected.png",
        ])];
        Self {
            // perf: PerformanceEvaluation::new(),
            btns: btns,
            change_to: None,
        }
    }
}

impl MyViewImpl for MyLevelIndex {
    fn destory(&mut self) {
        // nothing todo!()
        self.btns.clear();
    }

    fn to_change(&self, option: &mut MyGameOption) -> Option<String> {
        // match self.change_to.clone()?.as_str() {
        //     "Menu" | "Exit" => self.change_to.clone(),
        //     "Selected" => Some(String::from("Select Level")),
        //     s => {
        //         println!("Undefined Command:{s}");
        //         None
        //     }
        // }
        match self.change_to.clone()? {
            Selector::State(s) => match s.as_str() {
                "Menu" | "Exit" => Some(s),
                s => {
                    println!("Undefined Command:{s}");
                    None
                }
            },
            Selector::Select(s) => {
                option.game_info.current_group_id = Some(s.group);
                option.game_info.current_level_id = Some(s.level);
                Some(String::from("Select Level"))
            }
        }
    }

    fn paint(&mut self, ui: &mut egui::Ui, option: &MyGameOption) {
        if self.btns[0].button(ui, "返回", 0, 1).clicked() {
            println!("返回");
            self.change_to = Some(Selector::State(String::from("Menu")));
        }
        let ui = &mut ui.child_ui(
            ui.max_rect(),
            egui::Layout::left_to_right(egui::Align::Center),
        );
        let frame = egui::Frame {
            inner_margin: 12.0.into(),
            outer_margin: 40.0.into(),
            rounding: 10.0.into(),
            shadow: egui::epaint::Shadow {
                offset: [0.0, 0.0].into(),
                blur: 16.0,
                spread: 0.0,
                color: egui::Color32::from_black_alpha(50),
            },
            fill: egui::Color32::from_rgba_unmultiplied(200, 200, 255, 128),
            stroke: egui::Stroke::new(1.0, egui::Color32::GRAY),
        };
        for (i, g) in &option.game_library.groups {
            frame.show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new(format!("第 {i} 组"))
                            .size(30.0)
                            .color(egui::Color32::GRAY),
                    )
                    .on_hover_text_at_pointer(&g.name);
                    ui.horizontal(|ui| {
                        for (j, l) in &g.levels {
                            if ui
                                .button(
                                    egui::RichText::new(format!("第 {j} 关\n{}", &l.name))
                                        .extra_letter_spacing(1.0)
                                        .size(20.0),
                                )
                                .clicked()
                            {
                                self.change_to = Some(Selector::Select(SelectedLevel {
                                    level: *j,
                                    group: *i,
                                }));
                                println!("第 {i} 组 第 {j} 关");
                            }
                        }
                    })
                })
            });
        }

        // event handler
        if option.events.esc {
            self.change_to = Some(Selector::State(String::from(if option.events.shift_l {
                "Exit"
            } else {
                "Menu"
            })))
        }
    }
}
