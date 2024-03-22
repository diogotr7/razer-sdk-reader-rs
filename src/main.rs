#![allow(dead_code)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::color_provider::ColorProvider;
use crate::keyboard::ChromaKeyboard;
use signaled_reader::SignaledReader;
use std::sync::{Arc, Mutex};

mod appdata;
mod chroma_mutex;
mod color_provider;
mod common;
mod constants;
mod encryption;
mod keyboard;
mod reader;
mod signaled_reader;
mod utils;

use eframe::egui;
use eframe::egui::Rect;

fn main() -> Result<(), eframe::Error> {
    let _mutex = chroma_mutex::ChromaMutex::new();
    let initial_colors = [0xffffffff; 6 * 22];
    let colors = Arc::new(Mutex::new(initial_colors));
    let arc = Arc::clone(&colors);
    let keyboard_reader = SignaledReader::<ChromaKeyboard>::new(
        constants::KEYBOARD_FILE_NAME,
        constants::KEYBOARD_WAIT_HANDLE,
        Box::new(move |keyboard| {
            let mut colors = arc.lock().unwrap();
            for i in 0..6 * 22 {
                colors[i] = keyboard.get_color(i);
            }
        }),
    );

    std::thread::spawn(move || {
        keyboard_reader.run();
    });

    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(false)
            .with_inner_size([22.0 * 50.0, 6.0 * 50.0]),
        vsync: true,
        ..Default::default()
    };

    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("colors").num_columns(22).show(ui, |ui| {
                let colors = colors.lock().unwrap();
                for i in 0..6 {
                    for j in 0..22 {
                        let idx = i * 22 + j;
                        let color = colors[idx];
                        let clr = egui::Color32::from_rgb(
                            (color & 0xFF) as u8,
                            ((color >> 8) & 0xFF) as u8,
                            ((color >> 16) & 0xFF) as u8,
                        );
                        ui.allocate_space(egui::Vec2::new(50.0, 50.0));
                        ui.painter().rect_filled(
                            Rect::from_min_size(
                                egui::Pos2::new(j as f32 * 50.0, i as f32 * 50.0),
                                egui::Vec2::new(50.0, 50.0),
                            ),
                            0.0,
                            clr,
                        );
                    }

                    ui.end_row();
                }
                //todo: probably unnecessary
                ctx.request_repaint();
            });
        });
    })
}
