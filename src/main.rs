use std::process::exit;

use eframe::egui;

mod config;
use config::Config;

mod backends;
use backends::CompletionBackend;
use backends::CompletionEntry;
use egui::Color32;
use egui::Response;
use egui::Sense;
use egui::Ui;

const WIDTH: f32 = 200.0;
const HEIGHT: f32 = 600.0;

fn main() -> Result<(), eframe::Error> {
    let config = Config::from_disc().unwrap_or_else(|_| Config::default());

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_inner_size([WIDTH, HEIGHT])
            .with_resizable(false)
            .with_active(true)
            .with_window_type(egui::X11WindowType::Splash),
        centered: true,
        follow_system_theme: true,
        ..Default::default()
    };

    let model = URun::new(config);

    eframe::run_native("uRun", options, Box::new(|_cc| Box::new(model)))
}

#[allow(dead_code)]
struct URun {
    input: String,
    backend: Box<dyn CompletionBackend>,
    displayed: Vec<(Response, CompletionEntry)>,
    focused: usize,
    history_idx: usize,
    config: Config,
}

impl URun {
    fn new(config: Config) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    fn exit(&self, code: i32) {
        match self.config.to_disc() {
            Err(_) => println!("Error writing config to disc. Closing urun."),
            Ok(_) => println!("Closing urun."),
        }
        exit(code);
    }
}

impl Default for URun {
    fn default() -> Self {
        Self {
            input: "".to_string(),
            backend: Box::new(backends::launcher::Completions::new()),
            displayed: vec![],
            focused: 0,
            history_idx: 0,
            config: Default::default(),
        }
    }
}

impl eframe::App for URun {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let input_line = ui.text_edit_singleline(&mut self.input);
            if input_line.changed() {
                self.backend.generate(&self.input);
                self.focused = 0;
            }

            for (resp, item) in &self.displayed {
                if ui.input(|i| i.pointer.any_click()) && resp.hovered() {
                    let _ = self.backend.execute(item);
                }
            }

            // History previous
            if ui.input(|i| i.key_released(egui::Key::P) && i.modifiers.alt)
                || ui.input(|i| i.key_released(egui::Key::ArrowUp))
            {
                let history_len = self.config.history.len();
                if self.history_idx < history_len {
                    self.history_idx += 1;
                    self.input = self.config.history[history_len - self.history_idx].clone();
                }
            }
            // History next
            else if ui.input(|i| i.key_released(egui::Key::N) && i.modifiers.alt)
                || ui.input(|i| i.key_released(egui::Key::ArrowDown))
            {
                let history_len = self.config.history.len();
                if self.history_idx > 0 {
                    self.input = self.config.history[history_len - self.history_idx].clone();
                    self.history_idx -= 1;
                }

                if self.history_idx == 0 {
                    self.input = "".to_string();
                }
            }
            // Down
            else if ui.input(|i| i.key_released(egui::Key::N) && i.modifiers.ctrl)
                || ui.input(|i| {
                    i.key_released(egui::Key::Tab) && !i.modifiers.ctrl && !i.modifiers.shift
                })
            {
                if self.backend.all().len() - 1 > self.focused {
                    self.focused += 1;
                }
            }
            // Up
            else if ui.input(|i| i.key_released(egui::Key::P) && i.modifiers.ctrl)
                || ui.input(|i| {
                    i.key_released(egui::Key::Tab) && (i.modifiers.ctrl || i.modifiers.shift)
                })
            {
                if self.focused > 0 {
                    self.focused -= 1;
                }
            }
            // Execute
            else if ui.input(|i| i.key_pressed(egui::Key::Enter))
                || ui.input(|i| i.key_released(egui::Key::Y) && i.modifiers.ctrl)
            {
                if self.backend.len() > 0 {
                    let (_, task) = &self.displayed[self.focused];
                    match self.backend.execute(&task) {
                        backends::Exec::Continue => {
                            self.config.history.push(task.action.clone());
                            self.input = "".to_string();
                        }
                        backends::Exec::Exit(code) => {
                            self.config.history.push(task.action.clone());
                            self.input = "".to_string();
                            self.exit(code);
                        }
                    }
                } else if self.input.len() > 0 {
                    match self.backend.command(&self.input) {
                        backends::Exec::Continue => {
                            self.config.history.push(self.input.clone());
                            self.input = "".to_string();
                        }
                        backends::Exec::Exit(code) => {
                            self.config.history.push(self.input.clone());
                            self.input = "".to_string();
                            self.exit(code);
                        }
                    }
                }
            }
            // Exit
            else if ui.input(|i| i.key_released(egui::Key::Escape)) {
                self.exit(0);
            }

            self.displayed = vec![];
            let _area =
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |vui| {
                    for v in self.backend.all() {
                        let highlight = self.displayed.len() == self.focused;

                        self.displayed.push((v.ui(vui, highlight), v.clone()));
                    }
                });
            input_line.request_focus();
        });
    }
}
impl CompletionEntry {
    // At present, apparently, we can either show a hover effect here on mouse
    // hover OR respond to clicks in update(..) but not both
    fn ui(&self, ui: &mut Ui, highlight: bool) -> Response {
        let mut surround = egui::Frame::default()
            .inner_margin(5.0)
            .fill(if highlight {
                Color32::DARK_GRAY
            } else {
                Default::default()
            })
            .begin(ui);
        {
            let title_lbl = egui::Label::new(
                egui::RichText::new(&self.title)
                    .heading()
                    .background_color(egui::Color32::GRAY)
                    .color(egui::Color32::BLACK)
                    .size(16.0),
            );
            let path_lbl = egui::Label::new(egui::RichText::new(&self.subtitle));

            surround.content_ui.add(title_lbl);
            surround.content_ui.add(path_lbl);
        }
        return surround.end(ui).interact(Sense::click());
    }
}
