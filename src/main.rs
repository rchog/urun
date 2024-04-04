use eframe::egui;

mod backends;
use backends::CompletionBackend;
use egui::TextBuffer;
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([200.0, 600.0]),
        centered: true,
        follow_system_theme: false,
        ..Default::default()
    };
    eframe::run_native(
        "uRun",
        options,
        Box::new(|cc| {
            // let style = eframe::egui::Style {
            //     visuals: eframe::egui::Visuals::light(),
            //     ..eframe::egui::Style::default()
            // };
            // cc.egui_ctx.set_style(style);
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
            ui.text_edit_singleline(&mut self.input);
            self.backend.generate(&self.input);

            let area = ui.vertical(|vui| {
                for v in self.backend.all() {
                    vui.label(v.clone().as_str());
                }
            });
            let lab = ui.label("...");
        });
    }
}
