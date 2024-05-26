// # 278 First Bad Version
// notes:

// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

pub struct Solution;

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let (mut first, mut last) = (1, n);

        while first < last {
            let mid = first + (last - first) / 2;

            if self.isBadVersion(mid) {
                last = mid;
            } else {
                first = mid + 1;
            }
        }

        first
    }

    // for approx / making compiler happy:
    #[allow(non_snake_case)]
    pub fn isBadVersion(&self, n: i32) -> bool {
        return n == 2;
    }
}
