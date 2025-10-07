impl Solution {
    pub fn count_even(num: i32) -> i32 {
        let mut cnt = 0;
        for n in 2..=num{
            let s = n.to_string().chars().map(|x|x.to_string().parse::<i32>().unwrap()).sum::<i32>();
            if s % 2 ==0{
                cnt+=1;
            }
        }
        cnt
    }
}