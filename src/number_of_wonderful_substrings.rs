// # 1915 Number of Wonderful Substrings
// note: `daily` for 4/30/24
//
// CONSTRAINTS:
// 1 <= word.length <= 105
// word consists of lowercase English letters from 'a' to 'j'.

// pure bitmask:
pub fn wonderful_substrings_bitmask(word: String) -> i64 {
    let mut count = [0; 1024];

    count[0] = 1;
    let mut wonderful_count = 0;
    let mut curr_mask = 0;

    for c in word.chars() {
        let c_idx = (c as u8 - b'a') as usize;
        curr_mask ^= 1 << c_idx;
        wonderful_count += count[curr_mask];
        for i in 0..10 {
            let new_mask = curr_mask ^ (1 << i);
            wonderful_count += count[new_mask];
        }
        count[curr_mask] += 1;
    }

    wonderful_count
}
