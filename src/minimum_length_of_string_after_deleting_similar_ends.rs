// # 1750 Minimum Length of String After Deleting Similar Ends
// note: `daily` for 3/5/24

// ex: `abbacab` -> can take as many from `either` side as long as pre/suffix char matches in both positions
// -- GREEDY
// - any time we shift L/R pointers we want to continue the shift until chars no longer match
// - when do we stop? L + R CANNOT be at same point

// O(n) and O(1)

pub fn minimum_length(s: String) -> i32 {
    // early return if our string has no way to "match" pre/suffixes
    if s.len() < 2 {
        return s.len() as i32;
    }
    // redeclare as slice (&[u8]) for direct idx'ing  (avoids timeout in tests from LC specifically)
    let s = s.as_bytes();
    // define 'L + R' pointers
    let (mut left, mut right) = (0, s.len() - 1);
    // `the prefix and suffix should not intersect at any index` + confirming matched chars at ptr pos's
    while left < right && s[left] == s[right] {
        // store matched char "temp" for inner iteration
        let matched_char = s[left];
        // incr. / decr. "inwards" to center of input String while either ptr pos matches "temp" variable
        while left <= right && s[left] == matched_char {
            left += 1;
        }
        while right >= left && s[right] == matched_char {
            right -= 1;
        }
    }

    return (right - left) as i32 + 1; // note: `off by one` due to right init. as "len() - 1"
}

// This kept the allocated String -- TIMED OUT!
// pub fn minimum_length(s: String) -> i32 {
//     // early return if our string has no way to "match" pre/suffixes
//     if s.len() < 2 {
//         return s.len() as i32;
//     }
//     // define 'L + R' pointers
//     let (mut left, mut right) = (0, s.len() - 1);
//     // `the prefix and suffix should not intersect at any index` + confirming matched chars at ptr pos's
//     while left < right && s.chars().nth(left) == s.chars().nth(right) {
//         // store matched char "temp" for inner iteration
//         let matched_char = s.chars().nth(left);
//         // incr. / decr. "inwards" to center of input String while either ptr pos matches "temp" variable
//         while left <= right && s.chars().nth(left) == matched_char {
//             left += 1;
//         }
//         while right >= left && s.chars().nth(right) == matched_char {
//             right -= 1;
//         }
//     }

//     return (right - left) as i32 + 1; // note: `off by one` due to right init. as "len() - 1"
// }

// ALT METHOD: utilize "trim_matches" method provided
pub fn minimum_length_with_methods(s: String) -> i32 {
    // convert to str (for trim_matches() method)
    let mut s = s.as_str();

    // length greater than minimum (no 'match' available with 1 char) and both first and last chars match
    while s.len() > 1 && s.as_bytes().first() == s.as_bytes().last() {
        // get
        let c = s.chars().next().unwrap();
        s = s.trim_matches(c); // cannot pass reference to this closure (`c` needs to be dereferenced to `char` it is for trimming)
    }

    s.len() as i32
}
