#![windows_subsystem = "windows"]
#![allow(unused)]

use eframe::egui;
use razer_sdk_reader_lib::chroma_mutex::ChromaMutex;
use razer_sdk_reader_lib::color_provider::ColorProvider;
use razer_sdk_reader_lib::constants;
use razer_sdk_reader_lib::keyboard::ChromaKeyboard;
use razer_sdk_reader_lib::signaled_reader::SignaledReader;
use std::sync::{Arc, Mutex};

const PIXEL: f32 = 50.0;

fn main() -> Result<(), eframe::Error> {
    const WIDTH: usize = ChromaKeyboard::WIDTH;
    const HEIGHT: usize = ChromaKeyboard::HEIGHT;
    const COUNT: usize = WIDTH * HEIGHT;

    let _mutex = ChromaMutex::new();
    let colors = Arc::new(Mutex::new([0xff000000; COUNT]));
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
                        let [r, g, b, _] = u32::to_le_bytes(colors.lock().unwrap()[i * WIDTH + j]);

                        ui.allocate_space(size);
                        ui.painter().rect_filled(
                            egui::Rect::from_min_size(
                                egui::Pos2::new(j as f32 * PIXEL, i as f32 * PIXEL),
                                size,
                            ),
                            0.0,
                            egui::Color32::from_rgb(r, g, b),
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
