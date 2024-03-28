// # 53 Maximum Subarray
// note: `Kadane's Algo` ref'd (from neetcode)

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max_sum = nums[0];
    let mut curr_sum = 0;

    nums.iter().for_each(|num| {
        curr_sum = std::cmp::max(curr_sum, 0);
        curr_sum += num;
        max_sum = std::cmp::max(max_sum, curr_sum);
    });

    max_sum
}

// alt: `fold` use

pub fn max_sub_array_fold(nums: Vec<i32>) -> i32 {
    let mut curr_sum = 0;

    nums.iter().fold(nums[0], |acc, num| {
        curr_sum = std::cmp::max(curr_sum, 0);
        curr_sum += num;
        std::cmp::max(acc, curr_sum)
    })
}

// classic loop

pub fn max_sub_array_loop(nums: Vec<i32>) -> i32 {
    let mut curr_sum = 0;
    let mut max_sum = nums[0];

    for num in nums {
        curr_sum = std::cmp::max(curr_sum, 0);
        curr_sum += num;
        max_sum = std::cmp::max(max_sum, curr_sum);
    }

    max_sum
}
