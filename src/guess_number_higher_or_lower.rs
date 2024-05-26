// # 374 Guess Number Higher or Lower
// note: uses external `guess` API that returns:
// - `-1` - guess higher than target
// - `0` - found target
// - `1` - guess lower than target
// for guessing between 1 to 'n'

// BINARY SEARCH
pub unsafe fn guess_number(n: i32) -> i32 {
    let mut lo = 1;
    let mut hi = n;

    while lo <= hi {
        let mid = lo + (hi - lo) / 2;

        match _guess(mid, _TARGET) {
            // approx. the api functionality with default values
            -1 => hi = mid - 1,
            0 => return mid,
            1 => lo = mid + 1,
            _ => unreachable!(),
        }
    }

    unreachable!() // needed to satisfy compiler issues(? doesn't know we WILL find an answer within the loop)
}

// approx. + making compiler happy
fn _guess(n: i32, target: i32) -> i32 {
    use std::cmp::Ordering;

    match n.cmp(&target) {
        Ordering::Equal => 0,
        Ordering::Less => 1,
        Ordering::Greater => -1,
    }
}

const _TARGET: i32 = 42;
