impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
           let mut results = Vec::new();
    let max_candies = candies.iter().max().unwrap();

    for i in &candies {
        if i + extra_candies >= *max_candies {
            results.push(true);
        } else {
            results.push(false);
        }
    }
    results
    }
}