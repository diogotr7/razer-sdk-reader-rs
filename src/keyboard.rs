use crate::color_provider::ColorProvider;
use crate::common::ChromaDevice;
use crate::encryption::{decrypt, decrypt_with_key, get_key};
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
    //60 bytes are effects we do not care about
    _padding: [u8; 60],
    static_param: u32,
    static_color: u32,
    custom1: [u32; 132],
    custom2_color: [u32; 132],
    custom2_key: [u32; 132],
    custom3_color: [u32; 192],
    custom3_key: [u32; 132],
}

impl ColorProvider for ChromaKeyboard {
    const WIDTH: usize = 22;
    const HEIGHT: usize = 6;

    fn get_color(&self, i: usize) -> u32 {
        let read_index = utils::to_read_index(self.write_index) as usize;
        let data = &self.data[read_index];

        let encrypted = match data.effect_type {
            6 => data.effect.static_color,
            8 => data.effect.custom2_color[i],
            _ => data.effect.custom1[i],
        };

        decrypt(encrypted, data.timestamp)
    }

    fn get_colors(&self, colors: &mut [u32]) {
        assert_eq!(colors.len(), Self::WIDTH * Self::HEIGHT);

        let read_index = utils::to_read_index(self.write_index) as usize;
        let data = &self.data[read_index];
        let key = get_key(data.timestamp);

        //doing the whole thing here because of wonky behavior when borrowing packed structs.
        //if we call the "correct" decrypt functions we need to copy 500 or so bytes of data
        if data.effect_type == 6 {
            let color = decrypt_with_key(data.effect.static_color, key);
            colors.fill(color);
        } else if data.effect_type == 8 {
            for i in 0..colors.len() {
                colors[i] = decrypt_with_key(data.effect.custom2_color[i], key);
            }
        } else {
            for i in 0..colors.len() {
                colors[i] = decrypt_with_key(data.effect.custom1[i], key);
            }
        }
    }
}
