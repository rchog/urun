use std::process::exit;

use eframe::egui;

mod backends;
use backends::CompletionBackend;
use backends::CompletionEntry;
use egui::InputState;
use egui::Response;
use egui::Sense;
use egui::Stroke;
use egui::Ui;

const WIDTH: f32 = 200.0;
const HEIGHT: f32 = 600.0;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([WIDTH, HEIGHT])
            .with_resizable(false)
            .with_active(true)
            .with_window_type(egui::X11WindowType::Splash),
        centered: true,
        follow_system_theme: true,
        ..Default::default()
    };
    eframe::run_native(
        "uRun",
        options,
        Box::new(|cc| {
            // let style = eframe::egui::Style {
            //     eframe::egui::style::TextStyle
            //     ..eframe::egui::Style::default()
            // };
            // cc.egui_ctx.set_style(style)
            Box::<URun>::default()
        }),
    )
}

#[allow(dead_code)]
struct URun {
    input: String,
    backend: Box<dyn CompletionBackend>,
    displayed: Vec<(Response, String)>,
}
impl Default for URun {
    fn default() -> Self {
        Self {
            input: "".to_string(),
            backend: Box::new(backends::by_env::Completions::new()),
            displayed: vec![],
        }
    }
}
impl eframe::App for URun {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let input_line = ui.text_edit_singleline(&mut self.input);
            if input_line.changed() {
                self.backend.generate(&self.input);
            }
            for (resp, full_path) in &self.displayed {
                dbg!(resp.sense);
                if ui.input(|i| i.pointer.any_click()) && resp.hovered() {
                    println!("clicked {}", full_path);
                    std::process::Command::new(full_path).spawn().unwrap();
                    exit(0);
                }
            }
            self.displayed = vec![];
            let area = ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |vui| {
                for v in self.backend.all() {
                    self.displayed.push((v.ui(vui), v.full_path.to_owned()));
                }
            });
            input_line.request_focus();
        });
    }
}
impl CompletionEntry {
    // At present, apparently, we can either show a hover effect here on mouse
    // hover OR respond to clicks in update(..) but not both
    fn ui(&self, ui: &mut Ui) -> Response {
        let mut surround = egui::Frame::default().inner_margin(5.0).begin(ui);
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
        // let mut resp = surround.allocate_space(ui);
        // let mut resp = surround.end(ui);
        // resp.sense = Sense::click();
        // if resp.hovered() {
        // surround.frame.fill = egui::Color32::from_gray(69);
        // }
        return surround.end(ui).interact(Sense::click());
        // surround.paint(ui);
        // resp
    }
}
