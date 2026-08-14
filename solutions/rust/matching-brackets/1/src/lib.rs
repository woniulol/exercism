pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for c in string.chars() {
        if c == '(' || c == '[' || c == '{' {
            stack.push(c);
            continue;
        }

        if c == ')' || c == ']' || c == '}' {
            if let Some(last) = stack.pop() {
                if c == ')' && last != '(' {
                    return false;
                }
                if c == ']' && last != '[' {
                    return false;
                }
                if c == '}' && last != '{' {
                    return false;
                }
            } else {
                return false;
            }
        }
    }

    return stack.is_empty();
}
