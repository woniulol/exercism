pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let chars: Vec<char> = digits.chars().collect();

    chars
        .windows(len)
        .for_each(|iter| res.push(String::from_iter(iter)));

    res
}
