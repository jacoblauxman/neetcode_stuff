// # 27 Remove Element

// 'retain' method
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|&num| num != val);
    nums.len() as i32
}

// 'for in'
pub fn remove_element_for(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut k = 0;
    for idx in 0..nums.len() {
        if nums[idx] != val {
            nums[k] = nums[idx];
            k += 1;
        }
    }

    k as i32
}
