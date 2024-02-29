// #1 Two Sum
use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_complements = HashMap::new();

    for (idx, &num) in nums.iter().enumerate() {
        let num_complement = target - num;

        if let Some(&num_complement_idx) = num_complements.get(&num_complement) {
            return vec![num_complement_idx, idx as i32];
        } else {
            num_complements.insert(num, idx as i32);
        }
    }
    // empty return if no sum found - should be unreachable given input should have EXACTLY ONE solution
    vec![]
}
