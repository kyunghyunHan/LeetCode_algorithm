impl Solution {
    pub fn sum_of_encrypted_int(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        for i in nums {
            let s = i.to_string().chars().map(|x|x.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();

            let s_len = s.len();
            let mut nm =s.iter().max().unwrap();
            let mut num = nm.to_string().repeat(s.len()).parse::<i32>().unwrap();
            sum+=num;
        }
         sum
    }
}