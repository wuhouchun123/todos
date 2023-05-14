use std::println;

use todos::todolist::{Enum, TodoItem};
#[derive(Default, serde::Serialize, serde::Deserialize)] //翻译为序列化和反序列化
pub struct MyApp {
    addtodo: String,
    task: usize,
    radio: Enum,
    data: Vec<TodoItem>,
    #[serde(skip)]
    del: i32,
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let mut data = Self::default();
        if let Some(storage) = cc.storage {
            data = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        data.del = -1;
        data
    }

    fn show_content(&mut self, ui: &mut eframe::egui::Ui) {
        match self.radio {
            Enum::All => {
                self.show_all(ui);
            }
            Enum::Active => {
                self.show_active(ui, false);
            }
            Enum::Completed => {
                self.show_active(ui, true);
            }
        }
    }

    fn show_all(&mut self, ui: &mut eframe::egui::Ui) {
        if self.data.len() > 0 {
            for (idx, todoitem) in &mut self.data.iter_mut().enumerate() {
                // 这个todoitem.singleitem(ui)执行了，所以生成了singleitem
                if todoitem.singleitem(ui) {
                    self.del = idx as i32;
                    // println!("all: {:?}", self.del);
                }
            }
        } else {
            ui.label("you have not create task yet");
        }
    }

    fn show_active(&mut self, ui: &mut eframe::egui::Ui, active: bool) {
        // println!("aa: {:?}", self.del);
        let datae = self
            .data
            .iter()
            .filter(|u| u.active == active)
            .collect::<Vec<_>>();
        if datae.len() > 0 {
            for (idx, todoitem) in &mut self.data.iter_mut().enumerate() {
                if todoitem.active == active {
                    if todoitem.singleitem(ui) {
                        self.del = idx as i32;
                        // println!("bb: {:?}", self.del);
                    }
                }
            }
        } else {
            if active {
                ui.label("you have not task todo yet!");
            } else {
                ui.label("you have not complete yet!");
            }
        }
    }

    fn dataupdate(&mut self) {
        let datae = self
            .data
            .iter()
            .filter(|u| u.active == false)
            .collect::<Vec<_>>();
        self.task = datae.len(); // todo的数量
        if self.del > -1 {
            // 如果del>-1，删除对应
            self.data.remove(self.del as usize);
            self.del = -1;
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        // println!("{:?}", self.del);
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("todos");
            let p = ui.add(
                eframe::egui::TextEdit::singleline(&mut self.addtodo).hint_text("write something"),
            );
            // 失去焦点并且不是因为点击其他地方，也就是说失去焦点是因为按了回车
            if p.lost_focus() & !p.clicked_elsewhere() {
                if self.addtodo.is_empty() {
                } else {
                    self.data.push(TodoItem {
                        title: self.addtodo.clone(),
                        active: false,
                        edit: false,
                    });
                    self.addtodo.clear();
                }
            }
            ui.horizontal(|ui| {
                ui.label(&self.task.to_string());
                ui.label("task left");
                ui.add_space(44.0);

                ui.selectable_value(&mut self.radio, Enum::All, "All");
                ui.selectable_value(&mut self.radio, Enum::Active, "Active");
                ui.selectable_value(&mut self.radio, Enum::Completed, "Completed");
            });
            ui.add_space(11.0);
            self.show_content(ui);
        });
        self.dataupdate();
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        eframe::set_value(_storage, eframe::APP_KEY, self);
    }
}
