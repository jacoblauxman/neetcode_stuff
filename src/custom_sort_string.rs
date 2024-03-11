// # 791 Custom Sort String
// note: `daily` for 3/11/24

use std::collections::HashMap;

pub fn custom_sort_string(order: String, s: String) -> String {
    // HashMap `char_counts` to collect frequency of `char`s within `s`
    let mut char_counts = HashMap::new();
    for c in s.chars() {
        *char_counts.entry(c).or_insert(0) += 1;
    }

    // create "new" return String -> to append ordered `char`s to
    let mut sorted_s = String::new();

    // iter through `order`, appending each `char` in `char_count` to return String `count` times (if found)
    for c in order.chars() {
        if let Some(&c_count) = char_counts.get(&c) {
            for _ in 0..c_count {
                sorted_s.push(c);
            }
            char_counts.remove(&c);
        }
    }

    // append remaing `char`s from `char_freq` (ordering doesn't matter here)
    for (c, c_count) in char_counts {
        for _ in 0..c_count {
            sorted_s.push(c);
        }
    }

    sorted_s
}
