impl Solution {
   pub fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
        
        let mut m = 0;
        let mut count = 0;

        for i in 0..nums.len(){
            m+=nums[i]; 
            if m ==0{
                count+=1;
            }
        }
        count
    }
}