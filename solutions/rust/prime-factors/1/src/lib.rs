pub fn factors(n: u64) -> Vec<u64> {
    let mut res: Vec<u64> = Vec::new();

    let mut divided_by = 2;
    let mut remainder = n;

    while divided_by <= remainder {
        while remainder % divided_by == 0 {
            res.push(divided_by);
            remainder /= divided_by
        }
        divided_by += 1
    }

    res
}
