// # 165 Compare Version Numbers
// note: `daily` for 5/3/24

// Return the following:
// If version1 < version2, return -1.
// If version1 > version2, return 1.
// Otherwise, return 0.

use std::cmp::Ordering::{Greater, Less};

// vector allocation
pub fn compare_version(version1: String, version2: String) -> i32 {
    let mut ver_nums1 = version1
        .split('.')
        .map(|chunk| chunk.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut ver_nums2 = version2
        .split('.')
        .map(|chunk| chunk.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    while ver_nums1.len() < ver_nums2.len() {
        ver_nums1.push(0);
    }

    while ver_nums2.len() < ver_nums1.len() {
        ver_nums2.push(0);
    }

    match ver_nums1.cmp(&ver_nums2) {
        Greater => 1,
        Less => -1,
        _ => 0,
    }
}

// no vec allocation
pub fn _compare_version(version1: String, version2: String) -> i32 {
    let mut ver_nums1 = version1.split('.').map(|n| n.parse().unwrap());
    let mut ver_nums2 = version2.split('.').map(|n| n.parse().unwrap());

    loop {
        match (ver_nums1.next(), ver_nums2.next()) {
            (Some(ver_nums1), ver_nums2) if ver_nums1 > ver_nums2.unwrap_or(0) => return 1,
            (ver_nums1, Some(ver_nums2)) if ver_nums2 > ver_nums1.unwrap_or(0) => return -1,
            (None, None) => return 0,
            _ => continue,
        }
    }
}
