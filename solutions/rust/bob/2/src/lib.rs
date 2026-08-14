pub fn reply(message: &str) -> &str {
    let message = message.trim_end();

    let is_silence = message.is_empty();
    let is_question = message.ends_with('?');
    let is_yell =
        message.chars().any(char::is_alphabetic) && message == message.to_uppercase();

    match (is_silence, is_yell, is_question) {
        (true, _, _) => "Fine. Be that way!",
        (_, true, true) => "Calm down, I know what I'm doing!",
        (_, true, false) => "Whoa, chill out!",
        (_, false, true) => "Sure.",
        _ => "Whatever.",
    }
}
