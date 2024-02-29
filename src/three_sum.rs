// # 15 3Sum

use std::cmp::Ordering;

// pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {}
// not sure if fn signature change is allowed, for sorting input Vec
pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();
    nums.sort_unstable();

    // iterate with `i` maintaining initial 'slot' of 3 int sum
    for i in 0..nums.len() {
        // early termination of current iteration,
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
            // each `i` in initial 'slot' will be unique (to avoid duplicate return values in `res`)
        }

        // assign L + R pointers - L is 'off by 1' from `i`
        let (mut left, mut right) = (i + 1, nums.len() - 1);
        // iter through rest of nums
        while left < right {
            // to compare to 0
            let curr_sum = nums[i] + nums[left] + nums[right];

            match curr_sum.cmp(&0) {
                // sum is less than 0, incr. L pointer
                Ordering::Less => left += 1,
                // we've found "match" of 0, add to `res`
                Ordering::Equal => {
                    res.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    // move L pointer 'forward' to find next unique value
                    while nums[left] == nums[left - 1] && left < right {
                        left += 1;
                    }
                }
                // sum is greater than 0, decr. R pointer
                Ordering::Greater => right -= 1,
            }
        }
    }

    res
}
