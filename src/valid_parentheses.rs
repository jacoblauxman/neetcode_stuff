// # 20 Valid Parentheses

pub fn is_valid(s: String) -> bool {
    // additional, maybe unnecessary guard check
    if s.len() < 2 {
        return false;
    }

    let mut stack = Vec::new();

    for c in s.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            ')' | ']' | '}' if Some(c) != stack.pop() => return false,
            _ => (), // in the case of allowing extraneous `char`s
        }
    }

    stack.is_empty()
}
