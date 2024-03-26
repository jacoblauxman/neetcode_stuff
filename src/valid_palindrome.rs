// # 125 Valid Palindrome

// 'best' - minimal / no allocations
pub fn is_palindrome(s: String) -> bool {
    let s_iter = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase());
    let s_iter_rev = s_iter.clone().rev();

    s_iter.eq(s_iter_rev)
}

//
//

// Alt

// pub fn is_palindrome(s: String) -> bool {
//     let s = s
//         .chars()
//         .filter(|c| c.is_alphanumeric())
//         .map(|c| c.to_ascii_lowercase())
//         .collect::<String>();

//     match s {
//         s if s.chars().rev().collect::<String>() == s => true,
//         _ => false,
//     }
// }

// Alt #2

// pub fn is_palindrome(s: String) -> bool {
//     let mut s = s;

//     if s.is_empty() {
//         return true;
//     }

//     s.retain(|c| c.is_alphanumeric());
//     s.to_lowercase() == s.chars().rev().collect::<String>().to_lowercase()
// }
