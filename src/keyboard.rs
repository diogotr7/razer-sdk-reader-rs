use crate::color_provider::ColorProvider;
use crate::common::ChromaDevice;
use crate::encryption::decrypt;
use crate::utils;

#[repr(C, packed)]
pub struct ChromaKeyboard {
    write_index: u32,
    _padding: u32,
    data: [ChromaKeyboardData; 10],
    device: [ChromaDevice; 10],
}

#[repr(C, packed)]
struct ChromaKeyboardData {
    flag: u32,
    effect_type: i32,
    effect: KeyboardEffect,
    _padding: u32,
    timestamp: u64,
}

#[repr(C, packed)]
struct KeyboardEffect {
    _padding: [u8; 60], //60 bytes are effects we do not care about
    static_param: u32,
    static_color: u32,
    custom1: [u32; 132],
    custom2_color: [u32; 132],
    custom2_key: [u32; 132],
    custom3_color: [u32; 192],
    custom3_key: [u32; 132],
}

impl ColorProvider for ChromaKeyboard {
    fn width(&self) -> u32 {
        22
    }

    fn height(&self) -> u32 {
        6
    }

    fn get_color(&self, i: usize) -> u32 {
        let idx = utils::to_read_index(self.write_index) as usize;
        let data = &self.data[idx];
        let effect_type = data.effect_type;

        match effect_type {
            6 => decrypt(data.effect.static_color, data.timestamp),
            8 => decrypt(data.effect.custom2_color[i], data.timestamp),
            _ => decrypt(data.effect.custom1[i], data.timestamp),
        }
    }
}
