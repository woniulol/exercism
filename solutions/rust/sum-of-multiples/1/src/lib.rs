pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut v: Vec<u32> = Vec::new();
    for i in 1..limit {
        let mut matched: bool = false;
        for f in factors.iter() {
            if *f == 0 {
                continue;
            }
            if i % f == 0 {
                v.push(i);
                matched = true;
            }
            if matched {
                break;
            }
        }
    }
    v.iter().sum()
}
