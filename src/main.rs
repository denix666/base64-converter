#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use base64::{Engine as _, engine::general_purpose};

fn load_icon() -> eframe::IconData {
    eframe::IconData::try_from_png_bytes(&include_bytes!("../assets/base64.png")[..]).unwrap()
}

fn main() {
    let mut title = String::from("Base64 converter v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(600.0, 600.0)),
        icon_data: Some(load_icon()),
        ..Default::default()
    };

    let mut plain_text_content = String::new();
    let mut encrypted_content = String::new();

    eframe::run_simple_native(&title, options, move |ctx, _frame| {
        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.vertical(|ui| {
                ui.group(|ui| {
                    ui.set_height(270.0);
                    ui.label("Plain text:");
                    egui::ScrollArea::vertical().id_source("plain").show(ui, |ui| {
                        let plain_text = egui::TextEdit::multiline(&mut plain_text_content)
                            .code_editor()
                            .font(egui::FontId::new(14.0, egui::FontFamily::Monospace))
                            .text_color(egui::Color32::GREEN)
                            .desired_rows(15)
                            .desired_width(f32::INFINITY)
                            .lock_focus(true);
                        if ui.add(plain_text).changed() {
                            encrypted_content = general_purpose::STANDARD.encode(plain_text_content.to_owned());
                        }
                    });
                });
                ui.group(|ui| {
                    ui.set_height(270.0);
                    ui.label("Decoded base64 text:");
                    egui::ScrollArea::vertical().id_source("encrypted").show(ui, |ui| {
                        let encrypted_text = egui::TextEdit::multiline(&mut encrypted_content)
                            .code_editor()
                            .font(egui::FontId::new(14.0, egui::FontFamily::Monospace))
                            .text_color(egui::Color32::GREEN)
                            .desired_rows(15)
                            .desired_width(f32::INFINITY)
                            .lock_focus(true);
                        if ui.add(encrypted_text).changed() {
                            match general_purpose::STANDARD.decode(&encrypted_content) {
                                Ok(bytes) => {
                                    match std::str::from_utf8(&bytes) {
                                        Ok(res) => {
                                            plain_text_content = res.to_string();
                                        },
                                        Err(_) => {
                                            plain_text_content = "Wrong input...".to_string();
                                        },
                                    }
                                },
                                Err(_) => {
                                    plain_text_content = "Invalid input...".to_string();
                                }
                            }
                        }
                    });
                });
            });
        });  
    }).unwrap();
}