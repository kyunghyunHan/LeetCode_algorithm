impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        let c = a.max(b);
        let mut cnt = 0;
        for i in 1..=c{
            if a % i ==0 && b % i==0{
                cnt+=1;
            }
        }
        cnt
    }
}