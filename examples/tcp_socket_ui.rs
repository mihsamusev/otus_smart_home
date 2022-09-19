#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use chrono::Utc;
use eframe::egui;
use smart_home::device::tcp_socket_client;

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.resizable = false;
    options.initial_window_size = Some(egui::Vec2::new(300.0, 600.0));

    eframe::run_native(
        "TCP Smart Socket controller",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    address: String,
    messages: Vec<String>,
    is_on: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            address: "127.0.0.1:8888".to_string(),
            messages: Vec::new(),
            is_on: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("IP:PORT ");
                ui.text_edit_singleline(&mut self.address);
            });

            ui.horizontal(|ui| {
                let switch_btn = ui.button("Turn on / off").clicked();
                if switch_btn {
                    self.is_on = !self.is_on;
                    let next_cmd = if self.is_on { "SET0" } else { "SET1" };

                    let mut message = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
                    message.push('\n');
                    message.push_str(
                        tcp_socket_client::query(&self.address, next_cmd)
                            .unwrap_or_else(|e| e.to_string())
                            .as_str(),
                    );
                    self.messages.push(message);
                }
                let status_btn = ui.button("Get status").clicked();
                if status_btn {
                    let mut message = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
                    message.push('\n');
                    message.push_str(
                        tcp_socket_client::query(&self.address, "GET")
                            .unwrap_or_else(|e| e.to_string())
                            .as_str(),
                    );
                    self.messages.push(message);
                }
            });
            ui.separator();
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        for item in self.messages.iter() {
                            ui.label(item);
                            ui.separator();
                        }
                    });
                });
        });
    }
}
