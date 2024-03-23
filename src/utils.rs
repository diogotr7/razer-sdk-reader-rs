use eframe::egui;

pub fn to_read_index(write_index: u32) -> u32 {
    if write_index == 0 {
        return 9;
    }
    return write_index - 1;
}

pub fn convert_color(color: u32) -> egui::Color32 {
    egui::Color32::from_rgba_premultiplied(
        (color & 0xFF) as u8,
        ((color >> 8) & 0xFF) as u8,
        ((color >> 16) & 0xFF) as u8,
        ((color >> 24) & 0xFF) as u8,
    )
}
