impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
     let mut d = vec![];
    let mut s = 0;

    for i in 0..nums.len() {
        if i == 0 {
            s += nums[0];
            d.push(s);
            continue;
        }
        if nums[i - 1] < nums[i] {
            s += nums[i];
        } else {
            s = nums[i];
        }
        d.push(s);
    }
    *d.iter().max().unwrap() as i32
    }
}