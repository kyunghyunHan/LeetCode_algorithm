impl Solution {
    pub fn alternating_sum(nums: Vec<i32>) -> i32 {
        let mut even_sum = 0;
        let mut odd_sum = 0;
        let mut sum = nums[0];
        for  i in 1..nums.len(){
             if i ==0 || i%2==0{
                sum+=nums[i];
             }else{
                sum-=nums[i];
             }
        }
        sum
       
    }
}