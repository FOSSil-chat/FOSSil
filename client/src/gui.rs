use eframe::egui::CentralPanel;

#[derive(Default)]

struct App {}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| ui.heading("Hello World"));
    }
}

pub fn main() -> Result<(), eframe::Error> {
    println!("Hello World!");
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_resizable(true)
            .with_inner_size([1366.0, 768.0]),
        ..Default::default()
    };
    eframe::run_native(
        "FOSSil Chat",
        options,
        Box::new(|_cc| Ok(Box::<App>::default())),
    )
}
