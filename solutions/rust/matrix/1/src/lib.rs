pub struct Matrix {
    lines: Vec<Vec<u32>>,
    n_rows: usize,
    n_columns: usize,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let input: Vec<Vec<u32>> = input
            .lines()
            .map(|s| s.split(" ").map(|d| d.parse().unwrap()).collect())
            .collect();

        let n_rows: usize = input.len();
        let mut n_columns: usize = 0;

        if let Some(n1) = input.iter().next() {
            n_columns = n1.len()
        }

        Self {
            lines: input,
            n_rows: n_rows,
            n_columns: n_columns,
        }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        if row_no == 0 || row_no > self.n_rows {
            None
        } else {
            Some(self.lines[row_no - 1].to_owned())
        }
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        if col_no == 0 || col_no > self.n_columns {
            None
        } else {
            let mut res: Vec<u32> = Vec::new();
            for line in self.lines.iter() {
                res.push(line[col_no - 1]);
            }
            Some(res)
        }
    }
}
