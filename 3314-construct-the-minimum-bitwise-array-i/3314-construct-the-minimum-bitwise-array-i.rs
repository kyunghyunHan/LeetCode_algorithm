impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        for x in nums{
            let mut y  = -1;
            for a in 0..x+1{
                if a|(a+1) ==x{
                    y =a;
                    break;
                }
            }
            ans.push(y);
        }
        ans
    }
}