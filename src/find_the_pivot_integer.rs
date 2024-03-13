// # 2485 Find the Pivot Integer
// note: `daily` for 3/13/24

// arithmetic sum:
// arithmetic_sum(num) = `num * (num + 1) / 2`

// arithmetic_sum(n) for `n` input
// our goal is to find `x` in `n` where each 'left' + 'right' side of `x` sum to the same value
// if pivot (`x`) exists then its arithmetic_sum EXCLUDING itself should be able to be added together to equal input's
// arithmetic_sum(x) + arithmetic_sum(x) - x = arithmetic_sum(n)

// long form:

// (x * (x + 1) / 2) + (x * (x + 1) / 2) - x == (n * (n + 1) / 2)
// -> distributive (`a(b + c) = ab + ac` where `a` is the 'outer' `x`) :: "(2(x) * (x + 1))"
// (2x * (x + 1)) / 2  - x = ...
// -> ..again "2x * x + 2x + 1"
// (2x^2 + 2x) / 2 - x
// -> x^2 + x - x
// x^2 = arithmetic_sum(n)

pub fn pivot_integer(n: i32) -> i32 {
    let sum = (n * (n + 1) / 2) as f64;
    let pivot = sum.sqrt();

    match pivot {
        // rounding errors prior with `powi` and simple mult. ->
        // compare and ensure `pivot` is actually an integer instead
        pivot if pivot == pivot.floor() => pivot as i32,
        _ => -1,
    }
}

//

//

// old, doesn't pass a few tests due to rounding errors:

// pub fn pivot_integer(n: i32) -> i32 {
//     let sum = (n * (n + 1) / 2) as f64;
//     let pivot = sum.sqrt();

//     // if pivot * pivot == sum {
//     if pivot.powi(2) == sum {
//         return pivot as i32;
//     } else {
//         // not found
//         return -1;
//     }
// }

// reminder:

// `x` is the pivot integer
// 1 + 2 ... + x == x + x+1 + x+2 ... + n

//( x * (x + 1) / 2) + (x * (x + 1) / 2) - x == n * (n + 1) / 2
