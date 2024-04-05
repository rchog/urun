use eframe::egui;

mod backends;
use backends::CompletionBackend;
use backends::CompletionEntry;
use egui::Response;
use egui::Ui;

const WIDTH: f32 = 200.0;
const HEIGHT: f32 = 600.0;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([WIDTH, HEIGHT])
            .with_resizable(false)
            .with_active(true)
            .with_window_type(egui::X11WindowType::Dock),
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
}
impl Default for URun {
    fn default() -> Self {
        Self {
            input: "".to_string(),
            backend: Box::new(backends::by_env::Completions::new()),
        }
    }
}
impl eframe::App for URun {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let input_line = ui.text_edit_singleline(&mut self.input);
            if input_line.changed() {
                self.backend.generate(&self.input);
            }
            let area = ui.vertical(|vui| {
                for v in self.backend.all() {
                    v.ui(vui);
                }
            });

            input_line.request_focus();
        });
    }
}

impl CompletionEntry {
    fn ui(&self, ui: &mut Ui) -> Response {
        ui.vertical_centered_justified(|ui| {
            ui.add(egui::Label::new(
                egui::RichText::new(&self.filename)
                    .heading()
                    .background_color(egui::Color32::GRAY)
                    .color(egui::Color32::BLACK)
                    .size(16.0),
            ));
            ui.label(egui::RichText::new(&self.full_path));
        })
        .response
    }
}
