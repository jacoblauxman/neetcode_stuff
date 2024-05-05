// # 881 Boats to Save People
// note: `daily` for 5/4/24

// You are given an array people where people[i] is the weight of the ith person,
// and an infinite number of boats where each boat can carry a maximum weight of limit.
// Each boat carries at most two people at the same time,
// provided the sum of the weight of those people is at most limit.

// Return the minimum number of boats to carry every given person.

// first pass - two pointer
pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
    let mut sorted_people = people.to_vec();
    sorted_people.sort_unstable();

    let mut boats = 0;
    let (mut left, mut right) = (0, sorted_people.len() - 1);

    while left <= right {
        boats += 1;

        // bounds check
        if left == right {
            break;
        }

        if sorted_people[left] + sorted_people[right] <= limit {
            left += 1;
        }

        right -= 1;
    }

    boats
}

// adjusted signature - less allocation / mutable input + early return:
pub fn num_rescue_boats2(mut people: Vec<i32>, limit: i32) -> i32 {
    // early 'out' check
    if people.len() <= 1 {
        return people.len() as i32;
    }

    people.sort_unstable();
    let (mut left, mut right) = (0, people.len() - 1);
    let mut boats = 0;

    while left <= right {
        boats += 1;

        // bounds check (before below idx'ing)
        if left == right {
            break;
        }

        // both people's weight within limit - need to incr+decr pointers to 'next' heaviest/lightest
        if people[left] + people[right] <= limit {
            left += 1;
        }

        // always decr for next iteration ('heaviest' `right` in boat alone OR with 'lightest' `left`)
        right -= 1;
    }

    boats
}
