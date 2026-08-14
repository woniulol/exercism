pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let names: Vec<&str> = "Alice, Bob, Charlie, David, Eve, Fred, Ginny, Harriet, Ileana, Joseph, Kincaid, Larry".split(", ").collect();
    let index = names.iter().position(|&name| name == student).unwrap();

    diagram
        .lines()
        .flat_map(|line| line.chars().skip(index * 2).take(2))
        .map(|r| match r {
            'G' => "grass",
            'V' => "violets",
            'C' => "clover",
            'R' => "radishes",
            _ => "",
        })
        .collect()
}
