impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        let mut result   =0;
        for i in 0..nums.len(){
            // println!("{}",nums[i].pow(2));
            // result+=(nums[i].pow(2));
           
            if nums.len() %(i +1) == 0  {
                result+=nums[i].pow(2);
                println!("{}",result);
            }
        }
        result
    }
}