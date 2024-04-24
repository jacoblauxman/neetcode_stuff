// # 1137 N-th Tribonacci Number
// note: `daily` for 4/23/24
use std::collections::HashMap;

// with helper / memoization
pub fn tribonacci(n: i32) -> i32 {
    let mut memo: HashMap<i32, i32> = HashMap::new();

    _tribonacci(n, &mut memo)
}

pub fn _tribonacci(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
    if let Some(&num) = memo.get(&n) {
        return num;
    }

    if n == 0 {
        return 0;
    }

    if n == 1 || n == 2 {
        return 1;
    }

    let num = _tribonacci(n - 1, memo) + _tribonacci(n - 2, memo) + _tribonacci(n - 3, memo);
    memo.insert(n, num);

    num
}

// using array 'bucket'
pub fn array_tribonacci(n: i32) -> i32 {
    let mut memo = vec![-1; n as usize + 1];

    _array_tribonacci(n, &mut memo)
}

pub fn _array_tribonacci(n: i32, memo: &mut Vec<i32>) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 || n == 2 {
        return 1;
    } else if memo[n as usize] != -1 {
        return memo[n as usize];
    }

    let num = _array_tribonacci(n - 1, memo)
        + _array_tribonacci(n - 2, memo)
        + _array_tribonacci(n - 3, memo);
    memo[n as usize] = num;

    num
}

// match statement
pub fn match_tribonacci(n: i32) -> i32 {
    let mut memo = HashMap::new();

    _match_tribonacci(n, &mut memo)
}

pub fn _match_tribonacci(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
    if let Some(&num) = memo.get(&n) {
        return num;
    }

    match n {
        0 => {
            memo.insert(0, 0);
            return 0;
        }
        1 | 2 => {
            memo.insert(n, 1);
            return 1;
        }
        _ => {
            let num = _match_tribonacci(n - 1, memo)
                + _match_tribonacci(n - 2, memo)
                + _match_tribonacci(n - 3, memo);
            memo.insert(n, num);
            return num;
        }
    }
}

// 'simple', no recursion (with array 'bucket')
pub fn simple_tribonacci(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    if n == 1 || n == 2 {
        return 1;
    }

    let mut memo = vec![0; n as usize + 1];

    memo[0] = 0;
    memo[1] = 1;
    memo[2] = 1;

    for i in 3..=n as usize {
        memo[i] = memo[i - 3] + memo[i - 2] + memo[i - 1];
    }

    memo[n as usize]
}
