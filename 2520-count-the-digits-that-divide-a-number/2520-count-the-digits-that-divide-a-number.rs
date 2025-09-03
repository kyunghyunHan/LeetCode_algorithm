impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        let s = num.to_string();
        let mut count  =0;
        for c in s.chars(){
           let n = c.to_string().parse::<i32>().unwrap();
           if num % n ==0{
             count+=1;
           }
        }
        count
    }
    
}