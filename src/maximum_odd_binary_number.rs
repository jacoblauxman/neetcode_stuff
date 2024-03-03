// # 2864 Maximum Odd Binary Number
// note: `daily` for 3/1/24

pub fn maximum_odd_binary_number(s: String) -> String {
    let ones = s.chars().filter(|c| *c == '1').count() - 1;
    let zeroes = s.chars().filter(|c| *c == '0').count();
    "1".repeat(ones) + &"0".repeat(zeroes) + "1"
}

// alt: use pointer + iterative based approach

// pub fn maximum_odd_binary_number(s: String) -> {
//     let mut s = s.chars().collect::<Vec<_>>();
//     let mut ptr = 0;
//     let must_be_one = s.len() - 1;

//     for i in 0..s.len() {
//         // ensures we swap a '1' value to `ptr` (regardless of whether 1 or not) and then incr `ptr` to next idx in return String
//         if s[i] == '1' {
//             s.swap(ptr, i);
//             ptr += 1;
//         }
//     }
//     // swap last found `1` to the end (guaranteed 'odd')
//     s.swap(ptr - 1, must_be_one);

//     s.into_iter().collect()

// }

// 3rd) found from another submission - `iter hell`

// pub fn maximum_odd_binary_number(s: String) -> String {
//     let ones = s.as_bytes().iter().filter(|c| *c == &b'1').count() - 1;
//     std::iter::repeat('1')
//         .take(ones)
//         .chain(std::iter::repeat('0').take(s.len() - ones - 1))
//         .chain(std::iter::once('1'))
//         .collect()
// }
