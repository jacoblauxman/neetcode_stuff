// # 58 Length of Last Word
// note: `daily` for 4/1/24

pub fn length_of_last_word(s: String) -> i32 {
    // s.trim().split(' ').last().unwrap().len() as i32 // not as efficient
    s.split_whitespace().last().unwrap().len() as i32
    // faster?
    // s.split_whitespace().next_back().map(|word| word.as_bytes().len() as i32).unwrap_or(0)
}

// alt solution:
pub fn length_of_last_word_two(s: String) -> i32 {
    let mut len = 0;
    for c in s.trim_end().chars().rev() {
        if c == ' ' {
            break;
        }
        len += 1;
    }

    len as i32
}
