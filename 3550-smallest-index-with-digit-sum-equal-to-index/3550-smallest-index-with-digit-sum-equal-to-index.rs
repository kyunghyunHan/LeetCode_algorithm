impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        for i in 0..nums.len(){
            let mut sum= Self::calculate(nums[i]);
            if sum ==i  as i32{
                return i as i32
            }
        }
        -1
    }
    fn calculate(mut n:i32) -> i32{
        let mut s = 0;
        while n > 0 {
           let d = n % 10;
           n = n / 10;
           s +=d;

        }
        s
    }
}