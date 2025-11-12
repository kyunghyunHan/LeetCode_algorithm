impl Solution {
    pub fn minimum_average(mut nums: Vec<i32>) -> f64 {
        nums.sort_unstable(); 

        let mut min_avg = f64::INFINITY;
        let n = nums.len();

        for i in 0..(n / 2) {
            let avg = (nums[i] + nums[n - 1 - i]) as f64 / 2.0;
            if avg < min_avg {
                min_avg = avg;
            }
        }

        min_avg
    }
}
