pub fn is_valid(code: &str) -> bool {
    if code.len() < 2 {
        return false;
    }

    let mut blocks: Vec<&str> = Vec::new();

    for block in code.split(" ") {
        if block == "" {
            continue;
        } else {
            blocks.push(block)
        }
    }

    if (blocks.len() == 1)
        && (blocks[0].len() == 1)
        && (blocks[0].bytes().next().unwrap().is_ascii_digit())
        && ((blocks[0].bytes().next().unwrap() - b'0') as u32 == 0)
    {
        return false;
    }

    let mut sum = 0;
    let mut count = 0;

    for block in blocks.iter().rev() {
        for b in block.bytes().rev() {
            if !b.is_ascii_digit() {
                return false;
            }

            let mut i = (b - b'0') as u32;

            if count % 2 != 0 {
                i = if i * 2 > 9 { i * 2 - 9 } else { i * 2 };
            }
            sum += i;
            count += 1;
        }
    }

    sum % 10 == 0
}
