impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let mut odd = 0;
        let mut even = 0;

        let nums = num.chars().map(|x|x.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        for i in 0..num.len(){
            if i % 2  !=0{
                odd+=nums[i];
            }else{
                even+=nums[i];
            }
        }

        odd == even
    }
}