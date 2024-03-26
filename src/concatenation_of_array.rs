// # 1929 Concatenation of Array
// note: neetcode 'dynamic arrays' course problem

pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let offset = nums.len();
    let mut concat = vec![0; offset * 2];

    nums.iter().enumerate().for_each(|(i, _num)| {
        concat[i] = nums[i];
        concat[i + offset] = nums[i];
    });

    concat
}

// 'one pass'

pub fn get_concatentation_one_pass(nums: Vec<i32>) -> Vec<i32> {
    let offset = nums.len();
    let mut concat = vec![0; offset * 2];

    (0..offset).for_each(|i| {
        concat[i] = nums[i];
        concat[i + offset] = nums[i];
    });

    concat
}

// 'methods'

pub fn get_concatenation_concat_method(nums: Vec<i32>) -> Vec<i32> {
    [nums.clone(), nums].concat()
}

pub fn get_concatenation_extend_within_method(mut nums: Vec<i32>) -> Vec<i32> {
    // let mut nums = nums.clone(); // used if not adjusting fn signature to `mut nums` arg
    nums.extend_from_within(0..);

    nums
}

pub fn get_concatenation_extend_slice_method(nums: Vec<i32>) -> Vec<i32> {
    let mut concat = nums.clone();
    concat.extend_from_slice(&nums);

    concat
}
