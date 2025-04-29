impl Solution {
 pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut dp = [0; 1001];
    let n = cost.len();

    

    for i in 2..=cost.len() {
        dp[i] = (dp[i - 1] + cost[i - 1]).min(dp[i - 2] + cost[i - 2]);
    }
    dp[n]
}

}