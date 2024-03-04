// # 948 Bag of Tokens
// note: `daily` for 3/4/24

// Plan:
// sort `bag` -> enables L/R `two pointer` method:
// `hi_score` vs `score` total'D and compared through iteration
// low scores (`left`) are desireable for `face up` (++score) points vs. hi scores (`right`)
// -> hi scores (`right`) are desireable `face down` for more `power` to add more low scores (`left`) to total
// incr./decr. pointers towards midpoint -- if at any point `score` goes below 0 `break` iteration (we've hit the end of possible valid `score` values in loop)

pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
    // early return for empty input (checks bounds as well for below assignment of `right` to len() - 1)
    if tokens.is_empty() {
        return 0;
    }

    // sorts in place - worst case O(n * log n) - sets `power` values of tokens low -> hi
    tokens.sort_unstable();

    // init idx pointers and score "tracking"
    let (mut left, mut right) = (0, tokens.len() - 1);
    let (mut score, mut hi_score) = (0, 0);

    while left <= right {
        // we have enough power to `flip` current low value face up (++score)
        if power >= tokens[left] {
            score += 1;
            power -= tokens[left]; // decr. `power` by `left` ptr val
            hi_score = std::cmp::max(score, hi_score); // reassignment of `hi_score` to greatest value
            left += 1;
        } else if score > 0 {
            // note: no reassignment here (decr. `score` can't be > "last" score/hi_score)
            score -= 1;
            power += tokens[right]; // incr. `power` by `right` ptr val
            right -= 1;
        } else {
            // we've hit an invalid (< 0) `score` value, early return from iteration (no more possible valid "moves" to make)
            break;
        }
    }

    hi_score
}

// ALT: `match` style control flow

pub fn bag_of_tokens_score_match_version(mut tokens: Vec<i32>, mut power: i32) -> i32 {
    if tokens.is_empty() {
        return 0;
    }

    tokens.sort_unstable();

    let (mut left, mut right) = (0, tokens.len() - 1);
    let (mut score, mut hi_score) = (0, 0);

    while left <= right {
        match (power >= tokens[left], score > 0) {
            (true, _) => {
                power -= tokens[left];
                score += 1;
                hi_score = std::cmp::max(hi_score, score);
                left += 1;
            }
            (false, true) => {
                power += tokens[right];
                score -= 1;
                right -= 1;
            }
            _ => break,
        }
    }

    hi_score
}
