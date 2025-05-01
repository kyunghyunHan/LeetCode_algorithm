impl Solution {
  pub fn divisor_game(n: i32) -> bool {
    let mut dp = vec![false; 1001];
    dp[0] = false;
    dp[1] = false;
    dp[2] = true;
    dp[3] = false;
    for i in 4..=n as usize {
        dp[i] = dp[i - 2];
    }
    dp[n as usize]
}

}