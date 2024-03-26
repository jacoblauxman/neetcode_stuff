// # 217 Contains Duplicate
use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    if nums.len() <= 1 {
        return false;
    }

    let mut dupes_check = HashSet::new();

    for num in nums {
        if dupes_check.contains(&num) {
            return true;
        }
        dupes_check.insert(num);
    }

    false
}

// older ver, two pointer (not necessary):
// pub fn contains_duplicate(nums: Vec<i32>) -> bool {
//     if nums.len() <= 1 {
//         return false;
//     }

//     let mut dupes_check = HashSet::new();
//     let (mut i, mut j) = (0, nums.len() - 1);

//     while i <= j {
//         if !dupes_check.insert(nums[i]) {
//             return true;
//         } else if i != j && !dupes_check.insert(nums[j]) {
//             return true;
//         }

//         i += 1;
//         j -= 1;
//     }

//     false
// }
