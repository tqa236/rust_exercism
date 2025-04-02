pub struct Matrix {
    rows: Vec<Vec<u32>>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let rows = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect()
            })
            .collect();
        Matrix { rows }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        self.rows.get(row_no - 1).cloned()
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        if col_no == 0 || self.rows.is_empty() || col_no > self.rows[0].len() {
            return None;
        }
        Some(self.rows.iter().map(|row| row[col_no - 1]).collect())
    }
}
