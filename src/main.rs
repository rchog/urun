use std::process::exit;

use eframe::egui;

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
    eframe::run_native("uRun", options, Box::new(|_cc| Box::<URun>::default()))
}

#[allow(dead_code)]
struct URun {
    input: String,
    backend: Box<dyn CompletionBackend>,
    displayed: Vec<(Response, CompletionEntry)>,
    focused: usize,
}
impl Default for URun {
    fn default() -> Self {
        Self {
            input: "".to_string(),
            backend: Box::new(backends::by_env::Completions::new()),
            displayed: vec![],
            focused: 0,
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

            // Down
            if ui.input(|i| i.key_released(egui::Key::N) && i.modifiers.ctrl)
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
                    let _ = self.backend.execute(&task);
                } else if self.input.len() > 0 {
                    let _ = self.backend.command(&self.input);
                }
            }
            // Exit
            else if ui.input(|i| i.key_released(egui::Key::Escape)) {
                exit(0);
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
            let filename_lbl = egui::Label::new(
                egui::RichText::new(&self.filename)
                    .heading()
                    .background_color(egui::Color32::GRAY)
                    .color(egui::Color32::BLACK)
                    .size(16.0),
            );
            let path_lbl = egui::Label::new(egui::RichText::new(&self.path));

            surround.content_ui.add(filename_lbl);
            surround.content_ui.add(path_lbl);
        }
        return surround.end(ui).interact(Sense::click());
    }
}
