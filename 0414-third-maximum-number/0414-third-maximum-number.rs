impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut cnt = 1;
        let mut max = 0;
        let mut nums = nums;
        nums.sort_by(|a,b|b.cmp(&a));
        max = nums[0];
        //3221
        //max =0 cnt =0
        //cnt 1 cnt 2 cnt 3
        for i in 1..nums.len(){
            if nums[i]!=max && max > nums[i]{
                if nums[i] != nums[i-1]{
                    cnt+=1;
                }
                if cnt ==3 {
                   max  = nums[i];
                   return max;
                }
             
            }
        }
        max = nums[0];
        max
    }
}