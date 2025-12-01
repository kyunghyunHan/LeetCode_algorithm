impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut max_sum = 0;

        for g in gain{
            sum+=g;
            max_sum = max_sum.max(sum);
        }
        max_sum
    }
}