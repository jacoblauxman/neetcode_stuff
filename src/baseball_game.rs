// # 682 Baseball Game

pub fn cal_points(operations: Vec<String>) -> i32 {
    let mut scores = Vec::new();

    for op in operations.iter() {
        match op.as_str() {
            "C" => {
                scores.pop();
            }
            "D" => scores.push(scores.last().unwrap_or(&0) * 2),
            "+" => {
                if let (Some(last_score), Some(second_last_score)) =
                    (scores.get(scores.len() - 1), scores.get(scores.len() - 2))
                {
                    scores.push(last_score + second_last_score)
                }
            }
            score_val => scores.push(score_val.parse::<i32>().unwrap_or(0)),
        }
    }

    scores.iter().sum()
}
