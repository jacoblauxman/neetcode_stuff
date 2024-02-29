// # 704 Binary Search
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.is_empty() {
        return -1;
    }
    if nums.len() == 1 {
        return if nums[0] == target { 0 } else { -1 };
    }
    // init pointers
    let (mut lo, mut hi) = (0, (nums.len() - 1) as i32);
    while lo <= hi {
        // midpoint between current `lo` + `hi` values
        let mid = (lo + hi) / 2;
        if nums[mid as usize] == target {
            return mid as i32;
        } else if nums[mid as usize] > target {
            // shorten search to `lo - mid` range -> move `hi` to midpoint ('off by 1')
            hi = mid - 1;
        } else {
            // shorten search to `mid - hi` range -> move `lo` to midpoint ('off by 1')
            lo = mid + 1;
        }
    }

    -1 // target not found
}

// 2) using `match` and `Ordering`
use std::cmp::Ordering;

pub fn search_with_match(nums: Vec<i32>, target: i32) -> i32 {
    let (mut lo, mut hi) = (0, nums.len() as i32 - 1);

    while lo < hi {
        let mid = (lo + hi) / 2;
        match nums[mid as usize].cmp(&target) {
            Ordering::Equal => return mid,
            Ordering::Less => lo = mid + 1,
            Ordering::Greater => hi = mid - 1,
        }
    }

    -1
}

// 3) don't forget the std/core-lib!
pub fn search_with_method(nums: Vec<i32>, target: i32) -> i32 {
    // built in method on Vec (from core::alloc!) -- only works on sorted slice (can be used via Deref trait on Vec)
    nums.binary_search(&target).map(|x| x as i32).unwrap_or(-1)
}
