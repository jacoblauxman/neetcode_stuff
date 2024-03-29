// # 3005 Count Elements with Maximum Frequency
// note: `daily` for 3/8/24

use std::collections::HashMap;

// note: all eles will be POSITIVE (fn sig from LC)
pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    let mut num_freqs = HashMap::new();
    // base case -> `worst case` is all eles have freq of 1
    let mut max_freq = 1;

    nums.iter().for_each(|num| {
        // insert each num as key - count init to 0 if not found then immediately += 1
        *num_freqs.entry(num).or_insert(0) += 1;
        // determines most frequent value in `nums` -> set as `max_freq`
        max_freq = max_freq.max(*num_freqs.get(num).unwrap());
    });

    // value of `max freq` multiplied by count of values found in `num_freqs` that have the same frequency as `max_freq`
    max_freq * num_freqs.values().filter(|&val| *val == max_freq).count() as i32
}

// ALT attempt to try and use `fold`

pub fn max_frequency_elements_with_fold(nums: Vec<i32>) -> i32 {
    let mut max_freq = 1;

    let num_freqs = nums.iter().fold(HashMap::new(), |mut nums_hash, num| {
        *nums_hash.entry(num).or_insert(0) += 1;
        max_freq = max_freq.max(*nums_hash.get(num).unwrap());
        nums_hash
    });

    max_freq * num_freqs.values().filter(|&val| *val == max_freq).count() as i32
}
