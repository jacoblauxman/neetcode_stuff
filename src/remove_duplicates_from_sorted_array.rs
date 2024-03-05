// # 26 Remove Duplicates from Sorted Array

// note: need to mutate input array + return num unique values

// two pointer - 'L + R'

// each unique value found, place to `left` pointer
// need variable to compare to prev value 'seen' in iteration
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut uniques_ptr = 1; // init AFTER 0th val (since by default unique) -- our return `count` of uniques (via idx)
    let mut seen = nums[0]; // init as first ele in `nums` -> keeps track of whether value has already been "seen"

    for i in 1..nums.len() {
        // "off by one" start since 0th is unique by default
        if nums[i] != seen {
            // haven't seen sorted input value yet
            seen = nums[i]; // mark value as `seen`
            nums[uniques_ptr] = nums[i]; // replace `nums` value with unique val
            uniques_ptr += 1; // incr. to point to next "spot" in `nums` for unique element
        }
    }

    uniques_ptr as i32
}
