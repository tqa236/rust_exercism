pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut result = Vec::with_capacity(garden.len());
    let rows = garden.len();
    if rows == 0 {
        return result;
    }
    let cols = garden[0].len();

    for i in 0..rows {
        let mut new_row = String::with_capacity(cols);
        for j in 0..cols {
            let current = garden[i].as_bytes()[j];
            if current == b'*' {
                new_row.push('*');
                continue;
            }

            let mut count = 0;
            for di in -1..=1 {
                for dj in -1..=1 {
                    if di == 0 && dj == 0 {
                        continue;
                    }
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    if ni >= 0
                        && (ni as usize) < rows
                        && nj >= 0
                        && (nj as usize) < cols
                        && garden[ni as usize].as_bytes()[nj as usize] == b'*'
                    {
                        count += 1;
                    }
                }
            }

            if count > 0 {
                new_row.push(char::from_digit(count as u32, 10).unwrap());
            } else {
                new_row.push(' ');
            }
        }
        result.push(new_row);
    }
    result
}
