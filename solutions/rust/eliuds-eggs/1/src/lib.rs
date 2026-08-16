pub fn egg_count(display_value: u32) -> usize {
    // display_value -> binary
    // count 1 in binary

    let mut v: Vec<usize> = Vec::new();
    let mut remaining = display_value;
    loop {
        if remaining == 0 {
            break;
        }

        if remaining % 2 == 1 {
            v.push(1 as usize)
        }
        remaining = remaining / 2;
        println!("{remaining}");

        if remaining == 1 {
            v.push(1);
            break;
        }
    }
    v.iter().sum()
}
