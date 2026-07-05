use eframe::egui;

pub fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("FOSSil Chat", native_options, Box::new(|cc| Ok(Box::new(FOSSilChat::new(cc)))));
}

#[derive(Default)]
struct FOSSilChat {}

impl FOSSilChat {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        
        //persistence, set global style, set global font, graphics shaders/buffers
        
        Self::default()
    }
}

impl eframe::App for FOSSilChat {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Hello World!");
            // widgets
        });
    }
}
