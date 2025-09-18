impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        let mut v = vec![];
        for  i in 0..nums.len(){
            let s = nums[i].to_string();
            let mut sum = 0;
            for c in s.chars(){
               sum += c.to_string().parse::<i32>().unwrap();
            }
            v.push(sum);
        }
        *v.iter().min().unwrap()
        
    }
}