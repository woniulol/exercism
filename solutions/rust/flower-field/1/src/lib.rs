pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut res: Vec<String> = Vec::with_capacity(garden.len());
    for (i, line) in garden.iter().enumerate() {
        let mut line_res = String::new();
        let bytes = line.as_bytes();

        for (j, dot) in bytes.into_iter().enumerate() {
            if dot == &b"*"[0] {
                line_res.push_str("*");
                continue;
            }
            let mut none_empty_count = 0;
            // left
            if j > 0 && bytes[j - 1] == b"*"[0] {
                none_empty_count += 1;
            }
            // right
            if j + 1 < line.len() && bytes[j + 1] == b"*"[0] {
                none_empty_count += 1;
            }
            // up
            if i > 0 {
                let prev_line_bytes = garden[i - 1].as_bytes();
                if j > 0 && prev_line_bytes[j - 1] == b"*"[0] {
                    none_empty_count += 1
                }
                if prev_line_bytes[j] == b"*"[0] {
                    none_empty_count += 1
                }
                if j + 1 < line.len() && prev_line_bytes[j + 1] == b"*"[0] {
                    none_empty_count += 1;
                }
            }
            // down
            if i + 1 < garden.len() {
                let next_line_bytes = garden[i + 1].as_bytes();
                if j > 0 && next_line_bytes[j - 1] == b"*"[0] {
                    none_empty_count += 1
                }
                if next_line_bytes[j] == b"*"[0] {
                    none_empty_count += 1
                }
                if j + 1 < line.len() && next_line_bytes[j + 1] == b"*"[0] {
                    none_empty_count += 1;
                }
            }
            if none_empty_count == 0 {
                line_res.push_str(" ");
            } else {
                line_res.push_str(&none_empty_count.to_string());
            }
        }
        println!("{line_res}");
        res.push(line_res);
    }
    res
}
