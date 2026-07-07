use eframe::egui;
use std::sync::mpsc::Receiver;

pub fn main(_rx: Receiver<String>) {
    // Remove _ when communicating between GUI and network.rs (do same in network.rs) - tx is sender and rx is receiver
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "FOSSil Chat",
        native_options,
        Box::new(|cc| Ok(Box::new(FOSSilChat::new(cc)))),
    );
}

#[derive(Default)]
struct FOSSilChat {}

impl FOSSilChat {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        //persistence, set global style, set global font, graphics shaders/buffers

        Self::default()
    }
}

impl eframe::App for FOSSilChat {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Hello World!");
            // widgets
        });
    }
}
