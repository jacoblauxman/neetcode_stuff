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

// optimized, no hash
pub fn minimum_pushes_no_hash(word: String) -> i32 {
    let mut char_counts = [0; 26]; // lowercase alpha count

    for c in word.bytes() {
        char_counts[(c - b'a') as usize] += 1;
    }

    char_counts.sort_unstable_by(|a, b| b.cmp(&a));

    char_counts
        .iter()
        .enumerate()
        .map(|(idx, count)| count * ((idx as i32 / 8) + 1))
        .sum()

    // --- Q: is this any faster? --- //

    // char_counts.sort_unstable();

    // char_counts
    //     .iter()
    //     .rev()
    //     .enumerate()
    //     .map(|(idx, count)| count * ((idx as i32 / 8) + 1))
    //     .sum()
}

// Q: is this any better?
pub fn minimum_pushes_no_hash_take_two(word: String) -> i32 {
    let mut char_counts = [0; 26]; // lowercase alpha count

    for c in word.bytes() {
        char_counts[(c - b'a') as usize] += 1;
    }

    let mut pushes = 0;
    char_counts.sort_unstable();

    for (idx, count) in char_counts.iter().rev().enumerate() {
        pushes += count * ((idx as i32 / 8) + 1);
    }

    pushes
}
