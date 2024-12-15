pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        if self.row_count == 0 {
            return vec![];
        }

        let mut triangle = vec![vec![1]];

        for i in 1..self.row_count {
            let mut row = vec![1];
            let prev_row = &triangle[(i - 1) as usize];

            for j in 1..i {
                row.push(prev_row[(j - 1) as usize] + prev_row[j as usize]);
            }

            row.push(1);
            triangle.push(row);
        }

        triangle
    }
}
