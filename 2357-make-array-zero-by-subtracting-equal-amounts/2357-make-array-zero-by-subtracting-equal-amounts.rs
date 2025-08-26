impl Solution {
    pub fn minimum_operations(mut nums: Vec<i32>) -> i32 {
        
    let mut count = 0;

    loop {
        if nums.iter().sum::<i32>() == 0 {
            return count
        } else {
            let min = *nums.iter().filter(|&&x| x > 0).min().unwrap();
            for i in 0..nums.len() {
                if nums[i] > 0 {
                    nums[i] -= min;
                }
            }
            count += 1;
        }
    }

    }
}