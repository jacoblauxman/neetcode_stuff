// 2678. Number of Senior Citizens
// note: `daily` for 8/1/24

pub fn count_seniors(details: Vec<String>) -> i32 {
    details
        .iter()
        .filter(|passenger| passenger[11..13].parse::<i32>().unwrap() > 60)
        .count() as i32
}
