impl Solution {
    pub fn count_tested_devices(battery_percentages: Vec<i32>) -> i32 {
        battery_percentages.into_iter().fold(0,|acc,p|acc+(p>acc) as i32)
    }
}