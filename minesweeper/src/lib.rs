pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let rows = minefield.len();
    if rows == 0 {
        return vec![];
    }
    let cols = minefield[0].len();
    if cols == 0 {
        return minefield.iter().map(|&row| row.to_string()).collect();
    }

    let mut result = vec![vec![' '; cols]; rows];

    for r in 0..rows {
        for c in 0..cols {
            if minefield[r].as_bytes()[c] == b'*' {
                result[r][c] = '*';
            } else {
                let count = count_adjacent_mines(minefield, r, c);
                if count > 0 {
                    result[r][c] = std::char::from_digit(count, 10).unwrap();
                }
            }
        }
    }

    result
        .into_iter()
        .map(|row| row.into_iter().collect())
        .collect()
}

fn count_adjacent_mines(minefield: &[&str], row: usize, col: usize) -> u32 {
    let rows = minefield.len();
    let cols = minefield[0].len();
    let mut count = 0;

    let row_range = row.saturating_sub(1)..=(row + 1).min(rows - 1);
    let col_range = col.saturating_sub(1)..=(col + 1).min(cols - 1);

    for r in row_range.clone() {
        for c in col_range.clone() {
            if minefield[r].as_bytes()[c] == b'*' {
                count += 1;
            }
        }
    }

    count
}
