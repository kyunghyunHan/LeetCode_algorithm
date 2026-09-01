impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let mut n = num;

        while (n>=10){

           if n==0{
            return 0;
           }
             println!("{}",n);
           let s  = n.to_string().chars().collect::<Vec<char>>();
            let mut sum = 0;
            for   i in 0..s.len(){
              sum+=s[i].to_digit(10).unwrap() as i32;
            }
            n = sum;

        }
        return n;


              
    }
}