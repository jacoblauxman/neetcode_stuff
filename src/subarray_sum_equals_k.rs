// # 560 Subarray Sum Equals K
// note: "Prefix Sum"

use std::collections::HashMap;

pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut total_sum = 0;
    let mut prefix_sum = 0;
    let mut prefix_sums_freq: HashMap<i32, i32> = HashMap::new();
    prefix_sums_freq.insert(0, 1);

    for num in nums {
        prefix_sum += num;
        let k_diff = prefix_sum - k;

        if let Some(&freq) = prefix_sums_freq.get(&k_diff) {
            total_sum += freq;
        }

        *prefix_sums_freq.entry(prefix_sum).or_insert(0) += 1;
    }

    total_sum
}
