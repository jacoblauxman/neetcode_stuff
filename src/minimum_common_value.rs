// # 2540 Minimum Common Value
// note: `daily` for 3/9/24

use std::collections::HashSet;

// using set and method chaining

pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let nums1 = nums1.into_iter().collect::<HashSet<i32>>();
    let nums2 = nums2.into_iter().collect::<HashSet<i32>>();

    *nums1.intersection(&nums2).min().unwrap_or(&-1)
}

// two pointer

pub fn get_common_two_pointer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let (mut i, mut j) = (0, 0);

    while i < nums1.len() && j < nums2.len() {
        if nums1[i] == nums2[j] {
            return nums1[i];
        } else if nums1[i] < nums2[j] {
            i += 1;
        } else {
            j += 1;
        }
    }

    -1
}

// two pointer 'match' style using `cmp()` + `Ordering`
use std::cmp::Ordering;

pub fn get_common_two_pt_match(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    // init pointers
    let (mut i, mut j) = (0, 0);

    while i < nums1.len() && j < nums2.len() {
        // use `Ordering` to compare values at each iter. step
        match nums1[i].cmp(&nums2[j]) {
            Ordering::Equal => return nums1[i],
            //incr. `i` to catch up to `j` value
            Ordering::Less => i += 1,
            //incr. `j` to catch up to `i` value
            Ordering::Greater => j += 1,
        }
    }
    // no match found
    -1
}
