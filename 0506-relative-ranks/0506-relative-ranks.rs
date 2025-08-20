impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    let mut score2 = score.clone();
    let mut result = Vec::new();
    score2.sort_by(|a, b| b.cmp(&a));

    for i in score {
        if let Some(index) = score2.iter().position(|&x| x == i) {
            match index {
                0 => {
                    result.push("Gold Medal".to_string());
                }
                1 => {
                    result.push("Silver Medal".to_string());
                }
                2 => {
                    result.push("Bronze Medal".to_string());
                }
                _ => {
                    result.push((index + 1).to_string());
                }
            }
        }
    }
    result
    }
}