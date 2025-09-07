impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let   mut result = Vec::new();

        for n in nums{
            let s = n.to_string();
            for c in s.chars(){
                let num = c.to_string().parse::<i32>().unwrap();
                result.push(num)
            }
       
        }
        result
    }
}