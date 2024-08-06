// 3016. Minimum Number of Pushes to Type Word II
// note: `daily` for 8/6/24

// first attempt
pub fn minimum_pushes(word: String) -> i32 {
    let mut char_counts: Vec<(char, i32)> = word
        .chars()
        .fold(std::collections::HashMap::new(), |mut acc, ch| {
            *acc.entry(ch).or_insert(0) += 1;
            acc
        })
        .iter()
        .map(|(&key, &val)| (key, val))
        .collect();

    char_counts.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    char_counts
        .iter()
        .enumerate()
        .map(|(idx, val)| val.1 * ((idx as i32 / 8) + 1))
        .sum()
}

// slightly better
pub fn minimum_pushes_take_two(word: String) -> i32 {
    let mut char_counts = word
        .chars()
        .fold(std::collections::HashMap::new(), |mut acc, ch| {
            *acc.entry(ch).or_insert(0) += 1;
            acc
        })
        .iter()
        .map(|(_, &val)| val)
        .collect::<Vec<i32>>();

    char_counts.sort_unstable_by(|a, b| b.cmp(&a));

    char_counts
        .iter()
        .enumerate()
        .map(|(idx, val)| val * ((idx as i32 / 8) + 1))
        .sum()
}
