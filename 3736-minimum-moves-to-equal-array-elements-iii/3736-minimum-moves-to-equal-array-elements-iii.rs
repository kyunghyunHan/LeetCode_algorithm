impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let mut n = 0;
        let mut ttl = 0;
        let mut mx = nums[0];

        for num in nums{
            ttl +=num;
            if num > mx {
                mx = num;
            }
            n+=1;
        }
        n*mx-ttl
    }
}