//  # 918 Maximum Sum Circular Subarray
// note: `circular array` means end connects to beginning:
// next ele for `nums[i]` is `nums[(i + 1) % n]`
// previous ele for `nums[i]` is `nums[(i - 1 + n) % n]`
// where `n` is `nums` length

// can ONLY inlcude each ele from buffer once
// offshoot of maximum_subarray
// -- utilize max AND minimum sums:
// -> by using minimum (contiguous) subarray, we can compare `global` max (contiguous found subarray) to the 'total' (sum of all nums) less the 'global' min

// EDGE CASE: all vals negative -- global max will be 'negative'
// -> so take global negative

pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
    let (mut curr_max, mut curr_min) = (0, 0);
    let (mut max, mut min) = (nums[0], nums[0]);
    let mut total = 0;

    for num in nums {
        curr_max = std::cmp::max(curr_max + num, num);
        curr_min = std::cmp::min(curr_min + num, num);
        total += num;

        max = std::cmp::max(max, curr_max);
        min = std::cmp::min(min, curr_min);
    }

    match max {
        max if max < 0 => max,
        _ => std::cmp::max(max, total - min),
    }
}
