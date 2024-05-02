// # 2441 Largest Positive Integer That Exists With Its Negative
// note: `daily` for 5/2/24

// Constraints:

// 1 <= nums.length <= 1000
// -1000 <= nums[i] <= 1000
// nums[i] != 0

use std::collections::HashSet;

// first attempt:
pub fn find_max_k(nums: Vec<i32>) -> i32 {
    let mut max_k = -1;
    let mut check = HashSet::new();

    for num in nums {
        let curr_k = num.abs();
        let complement = num * -1;
        if check.contains(&complement) {
            max_k = max_k.max(curr_k);
        }

        check.insert(num);
    }

    max_k
}

//
//

// slightly optimized (from first):
pub fn find_max_k_lil_better(nums: Vec<i32>) -> i32 {
    let mut max_k = -1; // default return if no `k` match
    let mut check = HashSet::new();

    for num in nums {
        // abs value for `max_k` comparison
        let curr_k = num.abs();
        // `check` has the inverse of our current `num` AND the `k` value is greater
        if check.contains(&(-num)) && curr_k > max_k {
            max_k = curr_k;
        }

        // always insert `num` into check for 'complement' check (`-num`)
        check.insert(num);
    }

    max_k
}

//
//
//

// two pointer (adjusts fn signature):
pub fn find_max_k_pointers_mut(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable(); // 'faster'?
                          // nums.sort(); // this works, too

    // instantiate pointers + `k` return value
    let mut left = 0;
    let mut right = nums.len() - 1;
    let mut max_k = -1;

    while left < right {
        // since sorted, if either of pointers exceed the '0' median we found no `k` 'complements'
        if nums[left] > 0 || nums[right] < 0 {
            break;
        }

        // match found, iterating 'outside-in' on the sorted vec, we've found `max_k`
        if nums[left].abs() == nums[right] {
            max_k = nums[right];
            break;
        }

        // `left`'s absolute value is greater than `right`, increment to 'lower' its abs. value
        if nums[left].abs() > nums[right] {
            left += 1;
        // opposite, need to decr. `right` to get closer to `left`
        } else {
            right -= 1;
        }
    }

    max_k
}
