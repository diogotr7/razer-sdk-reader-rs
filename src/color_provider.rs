pub trait ColorProvider {
    const WIDTH: usize;
    const HEIGHT: usize;
    fn get_color(&self, i: usize) -> u32;
    fn get_colors(&self, colors: &mut [u32]);
    //if we had genetic trait params,
    //restricting the array to be of size WIDTH * HEIGHT would be cool
}
