#![allow(dead_code)]

use signaled_reader::SignaledReader;
use crate::appdata::ChromaAppData;
use crate::color_provider::ColorProvider;
use crate::keyboard::ChromaKeyboard;

mod chroma_mutex;
mod color_provider;
mod utils;
mod common;
mod keyboard;
mod appdata;
mod reader;
mod signaled_reader;
mod constants;

fn main() {
    let _chroma_mutex = chroma_mutex::ChromaMutex::new();
    let _keyboard_reader = SignaledReader::<ChromaKeyboard>::new(
        constants::KEYBOARD_FILE_NAME,
        constants::KEYBOARD_WAIT_HANDLE,
        |keyboard| {
            let clr = keyboard.get_color(0, 0);
            let r = clr & 0xFF;
            let g = (clr >> 8) & 0xFF;
            let b = (clr >> 16) & 0xFF;
            println!("Keyboard color: r: {}, g: {}, b: {}", r, g, b);
        },
    );
    let _app_data_reader = SignaledReader::<ChromaAppData>::new(
        constants::APP_DATA_FILE_NAME,
        constants::APP_DATA_WAIT_HANDLE,
        |app_data| {
            println!("App data: {}", app_data.get_current_app_name());
        },
    );

    _app_data_reader.run();
}
