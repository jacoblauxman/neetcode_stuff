// # 349 Intersection of Two Arrays
// note: `daily` for 3/10/24

use std::collections::HashSet;

// 1) Set + various Vec methods
// - with set, we can contain `nums1` duplicates and provide a check for `nums2` in one
// - filter `nums2` into `nums` (return) by whether `num` is in `num_set`
// - HELPER methods: Vec provides both `sort` (n log n) and `dedup` (requires a sorted Vec)

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let num_set = nums1.into_iter().collect::<HashSet<i32>>();
    let mut nums = nums2
        .into_iter()
        .filter(|num| num_set.contains(&num))
        .collect::<Vec<i32>>();
    nums.sort();
    nums.dedup();
    nums
}

// 2) Two Hash sets:
// - utilizing `intersection` method -> have to `map` results to deref i32 value re `&i32`:
// "a value of 'Vec<i32> cannot be built from an iterator over eles of '&i32'"
pub fn intersection_two_sets(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let num1_set = nums1.into_iter().collect::<HashSet<i32>>();
    let num2_set = nums2.into_iter().collect::<HashSet<i32>>();
    num1_set.intersection(&num2_set).map(|num| *num).collect()
}
