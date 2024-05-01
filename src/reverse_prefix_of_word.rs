// # 2000 Reverse Prefix of Word
// note: `daily` for 5/1/24

pub fn reverse_prefix(word: String, ch: char) -> String {
    match word.find(ch) {
        Some(idx) => word[..=idx].chars().rev().collect::<String>() + &word[idx + 1..],
        None => word,
    }
}

// alternatives
pub fn reverse_prefix_for(word: String, ch: char) -> String {
    let mut adjusted_word = String::new();
    let mut reversed = false;

    for c in word.chars() {
        adjusted_word.push(c);
        if c == ch && !reversed {
            adjusted_word = adjusted_word.chars().rev().collect::<String>();
            reversed = true;
        }
    }

    adjusted_word
}

pub fn reverse_prefix_fold(word: String, ch: char) -> String {
    let mut reversed = false;

    word.chars().fold(String::new(), |mut acc, c| {
        acc += &c.to_string();

        if !reversed && c == ch {
            reversed = true;
            acc = acc.chars().rev().collect::<String>();
        }

        acc
    })
}

pub fn reverse_prefix_bytes(word: String, ch: char) -> String {
    let mut word = word.into_bytes();
    let ch = ch as u8;

    if let Some(idx) = word.iter().position(|&c| c == ch) {
        word[..=idx].reverse();
    }

    // String::from_utf8(word).unwrap()
    // alt:
    unsafe { String::from_utf8_unchecked(word) }
}

pub fn reverse_prefix_drain(mut word: String, ch: char) -> String {
    word.drain(..word.find(ch).unwrap_or(0) + 1) // drain only takes up to "0th" idx if ch not found
        .rev() // only called on the drained chars from `word` (if not found, ONLY first char rev'd)
        .collect::<String>() // collects drained + rev'd chars into new String to concat
        + &word // remaining chars/str from original `word`
}
