// # 347 Top K Frequent Elements
// note: guaranteed that answer is unique

// BUCKET SORT(of)! for linear O(n) time (vs max heap O(k log n) or an O(n log n))

// track idx for 'counts' of values within `nums` -> place given counted values within `nums` at matching idx point in new `frequency` array
// `frequency` array will be the LENGTH of `nums`, as worst case of each 'count' occuring once OR only one single value for entire length of `nums` present
// -- will utilize 'dummy' "0th" idx point (since no elements have a k frequency of 0)

// HashMap utilize to count occurences of each value to then place in `frequency` array

use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut nums_count = HashMap::new();
    let mut frequency = vec![vec![]; nums.len() + 1]; // account for 'dummy' 0th position
    let mut k_frequent = vec![];

    for num in nums {
        *nums_count.entry(num).or_insert(0) += 1;
    }

    for (num, &num_count) in &nums_count {
        frequency[num_count as usize].push(*num);
    }

    for i in (0..frequency.len()).rev() {
        for num in &frequency[i] {
            k_frequent.push(*num);

            if k_frequent.len() as i32 == k {
                return k_frequent;
            }
        }
    }

    k_frequent
}

//
//

// OLD, lots of issues (for reviewing with above re: borrow / ownership / reference rules + others)
// pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
//     let mut nums_count = HashMap::new();
//     let mut frequency = vec![vec![]; nums.len() + 1];
//     let mut k_frequent = vec![];

//     for num in nums {
//         *nums_count.entry(&num).or_insert(0) += 1;
//     }

//     for (num, num_count) in nums_count.iter() {
//         frequency[*num_count].push(num);
//     }

//     for i in frequency.iter().rev() {
//         for num in frequency[*i] {
//             k_frequent.push(num);
//             if k_frequent.len() == k {
//                 break;
//             }
//         }
//     }

//     k_frequent
// }
