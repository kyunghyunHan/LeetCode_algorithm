impl Solution {
    pub fn even_number_bitwise_o_rs(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for num in nums{
            if num % 2==0{
                result  = result | num;
            }
        }
        result
    }
}