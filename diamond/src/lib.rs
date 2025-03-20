pub fn get_diamond(c: char) -> Vec<String> {
    let c = c.to_ascii_uppercase();

    let size = (c as u8 - b'A') as usize;
    let width = 2 * size + 1;
    let total_rows = 2 * size + 1;

    let mut result: Vec<String> = Vec::with_capacity(total_rows);

    for i in 0..=size {
        let current_char = (b'A' + i as u8) as char;
        let mut row = vec![' '; width];

        row[size - i] = current_char;
        if i > 0 {
            row[size + i] = current_char;
        }

        result.push(row.into_iter().collect());
    }

    for i in (0..size).rev() {
        result.push(result[i].clone());
    }

    result
}
