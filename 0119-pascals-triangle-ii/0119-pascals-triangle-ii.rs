impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
    let mut dp: Vec<Vec<i32>> = vec![vec![0; 35]; 35];
   
    dp[1][1] = 1;
    dp[2][1] = 1;
    dp[2][2] = 1;
    for i in 3..=34 {
        for j in 1..=i {
            dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j];
        }
    }
    let row: Vec<i32> = (1..=row_index+1).map(|j| dp[row_index as usize +1][j as usize]).collect();

    row
}
}