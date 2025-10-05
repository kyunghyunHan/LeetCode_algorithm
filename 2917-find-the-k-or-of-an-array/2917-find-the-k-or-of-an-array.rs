impl Solution {
    pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        for b in 0..32{
            let mut cnt = 0;
            for x in &nums{
                if (x >> b) & 1 !=0 {
                    cnt+=1;
                }
            }
            if cnt >=k{
                ans |= 1 <<b;
            }
        }
        ans
    }
}