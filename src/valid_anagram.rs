// # 242 Valid Anagram
use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    let mut char_counts = HashMap::new();

    for c in s.chars() {
        *char_counts.entry(c).or_insert(0) += 1;
    }

    for c in t.chars() {
        if let Some(c_count) = char_counts.get_mut(&c) {
            *c_count -= 1;

            if *c_count == 0 {
                char_counts.remove(&c);
            }
        } else {
            return false;
        }
    }

    char_counts.is_empty()
}
