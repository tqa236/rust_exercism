pub fn egg_count(display_value: u32) -> usize {
    let mut count = 0;
    let mut value = display_value;

    while value > 0 {
        if value % 2 == 1 {
            count += 1;
        }
        value /= 2;
    }

    count
}
