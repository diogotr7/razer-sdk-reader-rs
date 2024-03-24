#![allow(unused)]
use std::sync::{Arc, Mutex};

use eframe::egui;
use eframe::egui::Rect;

use signaled_reader::SignaledReader;

use crate::color_provider::ColorProvider;
use crate::keyboard::ChromaKeyboard;

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

const PIXEL: f32 = 50.0;
fn main() -> Result<(), eframe::Error> {
    const WIDTH: usize = ChromaKeyboard::WIDTH;
    const HEIGHT: usize = ChromaKeyboard::HEIGHT;
    const COUNT: usize = WIDTH * HEIGHT;

    let _mutex = chroma_mutex::ChromaMutex::new();
    let initial_colors = [0xffffffff; COUNT];
    let colors = Arc::new(Mutex::new(initial_colors));
    let arc = Arc::clone(&colors);
    let keyboard_reader = SignaledReader::<ChromaKeyboard>::new(
        constants::KEYBOARD_FILE_NAME,
        constants::KEYBOARD_WAIT_HANDLE,
        Box::new(move |keyboard| {
            let mut colors = arc.lock().unwrap();
            keyboard.get_colors(&mut colors[..]);
        }),
    );

    std::thread::spawn(move || {
        keyboard_reader.run();
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(false)
            .with_inner_size([WIDTH as f32 * PIXEL, HEIGHT as f32 * PIXEL]),
        ..Default::default()
    };

    eframe::run_simple_native("razer-sdk-reader-rs", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("colors").show(ui, |ui| {
                for i in 0..HEIGHT {
                    for j in 0..WIDTH {
                        let size = egui::Vec2::new(PIXEL, PIXEL);
                        let color = colors.lock().unwrap()[i * WIDTH + j];

                        ui.allocate_space(size);
                        ui.painter().rect_filled(
                            Rect::from_min_size(
                                egui::Pos2::new(j as f32 * PIXEL, i as f32 * PIXEL),
                                size,
                            ),
                            0.0,
                            egui::Color32::from_rgba_premultiplied(
                                (color & 0xFF) as u8,
                                ((color >> 8) & 0xFF) as u8,
                                ((color >> 16) & 0xFF) as u8,
                                ((color >> 24) & 0xFF) as u8,
                            ),
                        );
                    }

                    ui.end_row();
                }
                //razer chroma runs at 30fps
                ctx.request_repaint_after(std::time::Duration::from_millis(1000 / 30));
            });
        });
    })
}
