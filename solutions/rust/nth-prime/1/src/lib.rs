pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    loop {
        for i in 1.. {
            let mut divisiable_count = 0;
            for j in 1..i {
                if i % j == 0 {
                    divisiable_count += 1;
                }
            }
            if divisiable_count == 1 {
                count += 1
            }
            if count == n + 1 {
                return i;
            }
        }
    }
}
