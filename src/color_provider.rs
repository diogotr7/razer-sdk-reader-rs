pub trait ColorProvider {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn get_color(&self, x: u32, y: u32) -> u32;
}
