pub const fn to_read_index(write_index: u32) -> u32 {
    if write_index == 0 {
        return 9;
    }
    write_index - 1
}
