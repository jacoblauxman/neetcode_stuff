// # 167 Two Sum II: Input Array is Sorted

// Original Logic:

// pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
//     let mut i = 0;
//     let mut j = 0;
//     let mut res: Vec<i32> = Vec::new();

//     for i in 0..numbers.len() {
//         for j in (i + 1)..numbers.len() {
//             let curr_sum = numbers[i] + numbers[j];

//             if curr_sum == target {
//                 res.push((i + 1) as i32);
//                 res.push((j + 1) as i32);
//             }
//         }
//     }

//     res
// }

// ALTERNATE LOGIC:

// - we know there's a result of [X, Y] (being idx's within input array)
// - imagine L + R pointers - L should reach X and R should reach Y (never crossing!)
// if L finds position first ie L = X, R = Y + K (K being random num)
// - SORTED so the sum of two points is > Target - decrease R and vice versa in scenario with < Target and L
// Note: remember the base-1 indexing (+ 1 to L and R return values)
//
// need `Ordering` trait
use std::cmp::Ordering;

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    // assign L + R pointers
    let (mut left, mut right) = (0, numbers.len() - 1);
    // iterate through `numbers`
    while left < right {
        // value to compare vs target
        let curr_sum = numbers[left] + numbers[right];

        match curr_sum.cmp(&target) {
            // target is greater than L+R sum, increment left pointer
            Ordering::Less => left += 1,
            // match found, return
            Ordering::Equal => return vec![(left + 1) as i32, (right + 1) as i32],
            // target is less than L+R sum, decrement right pointer
            Ordering::Greater => right -= 1,
        }
    }
    // should be unreachable as each input is given to have EXACTLY ONE solution
    vec![]
}
