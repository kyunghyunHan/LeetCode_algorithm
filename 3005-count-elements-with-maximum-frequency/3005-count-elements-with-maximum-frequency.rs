impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut freq = vec![0;101];
        let mut max = 0;
        let mut res = 0;
        for n in nums{
            freq[n as usize] += 1;               
            let f = freq[n as usize];
            if f > max {
                max = f;
                res = f;
            }
            else if f==max{
                res+=f;
            }
        }
        res
    }
}