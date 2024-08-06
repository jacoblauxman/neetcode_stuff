// 1508. Range Sum of Sorted Subarray Sums
// note: `daily` for 8/5/24

const MOD: i32 = 10_i32.pow(9) + 7;

pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
    let mut sub_sums = Vec::new();
    let mut sum = 0;

    for i in 0..n {
        let mut curr_sum = 0;

        for j in i..n {
            curr_sum += nums[j as usize] as i32;
            sub_sums.push(curr_sum);
        }
    }

    sub_sums.sort_unstable();

    for i in (left - 1)..(right) {
        sum = (sum + sub_sums[i as usize]) % MOD;
    }

    sum
}
