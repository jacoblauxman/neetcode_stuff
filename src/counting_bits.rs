// # 338 Counting Bits
// note: bit manipulation / dynamic programming

// "Brute Force" O(n log n) iterative solution
pub fn count_bits(n: i32) -> Vec<i32> {
    let mut ans = vec![0; n as usize + 1];
    for i in 1..ans.len() {
        let mut ones_count = 0;
        let mut num = i;
        while num != 0 {
            if num % 2 == 1 {
                ones_count += 1;
            }
            num = num / 2;
        }
        ans[i] = ones_count;
    }

    ans
}

//

//

// 0 -- 0000 -- 0
// 1 -- 0001 -- 1
// 2 -- 0010 -- 1
// 3 -- 0011 -- 2
// 4 -- 0100 -- 1 <-- 'offset point' :: 1 + dp[n - 4] -> use previous results to 'memoize' previous arr. values prior to offset idx
// 5 -- 0101 -- 2 ... '1 + dp[n - 4]'
// 6 -- 0110 -- 2
// 7 -- 0111 -- 3
// 8 -- 1000 -- 1 <-- 'new offset point' :: 1 + dp[n - 8]

// 'offset' can also be seen as 'significant bit' -- powers of 2 pattern!

// "Dynamic" O(n)
pub fn count_bits_dynamic(n: i32) -> Vec<i32> {
    let mut ans = vec![0; n as usize + 1];
    let mut offset = 1;

    for i in 1..ans.len() {
        // adjust offset based off mult/pow of 2 pattern re: binary + current `i` value
        if offset * 2 == i {
            offset = i;
        }
        //
        ans[i] = 1 + ans[i - offset];
    }
    ans
}

//

//

// very cheeky `count_ones` method provided via i32 in std lib

pub fn count_bits_with_method(n: i32) -> Vec<i32> {
    (0..=n)
        .into_iter()
        .map(|num| num.count_ones() as i32)
        .collect()
}
