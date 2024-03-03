// # 977
// note: `daily` for 3/2/24

// exploring various methods / method chaining
// pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
pub fn sorted_squares_one(mut nums: Vec<i32>) -> Vec<i32> {
    nums = nums.iter().map(|num| num.pow(2)).collect();
    // let mut nums = nums.iter().map(|num| num.pow(2)).collect(); // before changing fn signature to allow `mut nums`
    nums.sort_unstable();
    nums
}

// two pointer
// pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
//     let (mut left, mut right, mut ptr) = (0, nums.len() - 1, nums.len() - 1);
//     // let mut res = Vec::with_capacity(nums.len()); // we HAVE to initialize with elements (`with_capacity` only allocates the memory - can't index into!)
//     let mut squares = vec![0; nums.len()];

//     while left <= right {
//         let left_val = nums[left].abs();
//         let right_val = nums[right].abs();

//         if left_val < right_val {
//             squares[ptr] = right_val.pow(2);
//             right -= 1;
//         } else {
//             squares[ptr] = left_val.pow(2);
//             left += 1;
//         }
//         ptr -= 1;
//     }

//     squares
// }

// Two pointer no `ptr`:
pub fn sorted_squares_two(nums: Vec<i32>) -> Vec<i32> {
    let (mut left, mut right) = (0, nums.len() - 1);
    let mut squares = vec![0; nums.len()];

    for i in (0..nums.len()).rev() {
        let left_val = nums[left].abs();
        let right_val = nums[right].abs();

        if left_val < right_val {
            squares[i] = right_val.pow(2);
            right -= 1;
        } else {
            squares[i] = left_val.pow(2);
            left += 1;
        }
    }

    squares
}

// another, `mut nums` / in place? panics
// pub fn sorted_squares(nums: &mut Vec<i32>) {
//     let (mut left, mut right) = (0, nums.len() - 1);

//     for idx in (0..nums.len()).rev() {
//         let left_val = nums[left].abs();
//         let right_val = nums[right].abs();

//         if left_val > right_val {
//             nums[idx] = left_val.pow(2);
//             left += 1;
//         } else {
//             nums[idx] = right_val.pow(2);
//             right -= 1;
//         }
//     }
// }
