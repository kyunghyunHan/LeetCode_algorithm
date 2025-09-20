impl Solution {
    pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {

        let y = x.to_string().chars().collect::<Vec<char>>();
        let mut sum = 0;
        for c in y{
           let num = c.to_string().parse::<i32>().unwrap();
           sum+=num;
        }
        if x % sum ==0{
            sum
        }else{
            -1
        }
        
        
    }
}