// # 58 Length of Last Word
// note: `daily` for 4/1/24

pub fn length_of_last_word(s: String) -> i32 {
    // s.trim().split(' ').last().unwrap().len() as i32 // not as efficient
    s.split_whitespace().last().unwrap().len() as i32
}
