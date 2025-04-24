impl Solution {
  pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut dp: Vec<Vec<i32>> = vec![vec![0; 31]; 31];

    dp[1][1] = 1;
    dp[2][1] = 1;
    dp[2][2] = 1;
    for i in 3..=30 {
        for j in 1..=i {
            dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j];
        }
    }
    let mut result = vec![];
    for i in 1..=num_rows {
        let row: Vec<i32> = (1..=i).map(|j| dp[i as usize][j as usize]).collect();
        result.push(row);
    }
    result
}

}