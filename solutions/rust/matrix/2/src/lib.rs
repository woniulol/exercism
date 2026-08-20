pub struct Matrix {
    lines: Vec<Vec<u32>>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let input: Vec<Vec<u32>> = input
            .lines()
            .map(|s| s.split_whitespace().map(|d| d.parse().unwrap()).collect())
            .collect();

        Self { lines: input }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        self.lines.get(row_no.checked_sub(1)?).cloned()
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        let col = col_no.checked_sub(1)?;
        self.lines.iter().map(|row| row.get(col).cloned()).collect()
    }

    // Difference between clone(), cloned(), and copy()
    //
    // clone() is a method on the value itself, from the Clone() trait;
    //
    // cloned() is an adapter fro when the thing you have is a reference wrapped in
    // a container;
    //
    // e.g. Option<&Vec<u32>> -> Option<Vec<u32>>
    // e.g. Iterator<Item = &u32> -> Iterator<Item = u32>
    // It is essentially map(|x| x.clone)
    //
    // copied() similar to cloned() but requires T: Copy and only does a bitwise copy
    // instead of calling clone()
}
