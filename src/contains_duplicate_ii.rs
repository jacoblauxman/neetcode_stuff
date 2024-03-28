// # 219 Contains Duplicate II
use std::collections::HashSet;

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    if k == 0 {
        return false;
    }

    let mut window = HashSet::new();
    let (mut left, mut right) = (0, 0);

    while right <= nums.len() {
        if right - left > k as usize {
            window.remove(&nums[left]);
            left += 1;
        }

        if let Some(num) = window.get(&nums[right]) {
            return true;
        }

        window.insert(nums[right]);
        right += 1;
    }

    false
}
