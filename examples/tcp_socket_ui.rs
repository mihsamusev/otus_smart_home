#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "TCP Smart Socket controller",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    ip: String,
    port: String,
    is_on: bool
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            ip: "127.0.0.1".to_string(),
            port: "8000".to_string(),
            is_on: false
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("IP: ");
                ui.text_edit_singleline(&mut self.ip);
            });
            ui.horizontal(|ui| {
                ui.label("PORT: ");
                ui.text_edit_singleline(&mut self.port);
            });
            if ui.button("Turn on / off").clicked() {
                self.is_on = !self.is_on;
            }
            if ui.button("Get status").clicked() {
                self.is_on = !self.is_on;
            }
            ui.label(format!("Listennig on {}:{} -> {}", self.ip, self.port, self.is_on));
            ui.label("Big canvas here");
        });
    }
}