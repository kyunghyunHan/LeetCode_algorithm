impl Solution {
   pub fn max_profit(prices: Vec<i32>) -> i32 {
 
    let mut min = prices[0];
    let mut result = 0;
    for i in prices {
        if i < min {
            min = i;
        }
        if i - min > result {
            result = i - min
        }
    }
    result
}
}