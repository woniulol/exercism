pub fn reply(message: &str) -> &str {
    let message = message.trim_end();
    let is_question = message.chars().rev().next().unwrap_or_default() == '?';
    let mut is_all_capital = true;
    let mut is_empty = true;
    let mut is_all_puntuation = true;

    for c in message.chars() {
        if !c.is_whitespace() {
            is_empty = false;
            if !c.is_ascii_punctuation() {
                is_all_puntuation = false;
                if !c.is_ascii_uppercase() {
                    is_all_capital = false;
                }
            }
        }
        if !is_empty && !is_all_capital {
            break;
        }
    }

    match (is_question, is_all_puntuation, is_all_capital, is_empty) {
        (_, _, _, true) => "Fine. Be that way!",
        (true, true, _, _) => "Sure.",
        (true, false, false, _) => "Sure.",
        (true, false, true, _) => "Calm down, I know what I'm doing!",
        (false, _, true, _) => "Whoa, chill out!",
        (_, _, _, _) => "Whatever.",
    }
}
