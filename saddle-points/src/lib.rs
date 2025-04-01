pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points = Vec::new();

    for (i, row) in input.iter().enumerate() {
        if let Some(max_in_row) = row.iter().max() {
            for (j, &value) in row.iter().enumerate() {
                if value == *max_in_row {
                    let is_min_in_col = input.iter().all(|r| r[j] >= value);
                    if is_min_in_col {
                        saddle_points.push((i, j));
                    }
                }
            }
        }
    }

    saddle_points
}
