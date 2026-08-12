const DIGIT_WORDS: [&str; 11] = [
    "no", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
];

const FIRST_CAP_DIGIT_WORDS: [&str; 11] = [
    "no", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
];

fn get_bottle_word(bottles: u32) -> String {
    if bottles == 1 {
        String::from("bottle")
    } else {
        String::from("bottles")
    }
}

fn recite_block(bottles: u32) -> String {
    let mut res = String::new();

    let mut line1 = String::from(FIRST_CAP_DIGIT_WORDS[bottles as usize]);
    line1.push_str(" green ");
    line1.push_str(&get_bottle_word(bottles));
    line1.push_str(" hanging on the wall,\n");

    res.push_str(&line1);
    res.push_str(&line1);

    res.push_str("And if one green bottle should accidentally fall,\n");

    let less_one = bottles - 1;
    let mut last_line = String::from("There'll be ");
    last_line.push_str(DIGIT_WORDS[less_one as usize]);
    last_line.push_str(" green ");
    last_line.push_str(&get_bottle_word(less_one));
    last_line.push_str(" hanging on the wall.");

    res.push_str(&last_line);

    res
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut res = String::new();
    let mut remaining_bottles = start_bottles;

    for _ in 1..take_down + 1 {
        res.push_str(&recite_block(remaining_bottles));
        remaining_bottles -= 1;
        if remaining_bottles != 0 {
            res.push_str("\n\n");
        }
    }

    res
}
