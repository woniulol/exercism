pub fn abbreviate(phrase: &str) -> String {
    let mut s = String::new();

    phrase.split(" ").into_iter().for_each(|word| {
        if word.chars().map(|c| c.is_uppercase()).all(|b| b == true) {
            s.push(word.chars().next().unwrap());
        } else if word.chars().map(|c| c.is_uppercase()).any(|b| b == true) {
            for c in word.chars() {
                if c.is_uppercase() {
                    s.push(c);
                }
            }
        } else {
            word.split("-").into_iter().for_each(|sep| {
                if let Some(c) = sep.chars().next() {
                    s.push(c.to_ascii_uppercase());
                }
            })
        }
    });
    s
}
