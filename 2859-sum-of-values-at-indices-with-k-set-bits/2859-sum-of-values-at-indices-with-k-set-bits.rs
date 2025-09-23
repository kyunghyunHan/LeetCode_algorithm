impl Solution {
    pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        let mut result=0;
        for i in 0..nums.len(){
            let bit = format!("{:b}",i);
            let mut count =0;
            for c in bit.chars(){
                if c == '1'{
                    count+=1;
                }
            }
            if count == k {
                result+=nums[i];
            }
        }

        result
    }
}