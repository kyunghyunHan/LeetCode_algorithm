impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut v = vec![0;46];
        let (mut i ,mut j , mut max)  =(0,0,0);

        for i in low_limit..=high_limit{
            let mut t = i;
            let mut s  = 0;
            while t!=0{
                s+=t%10;
                t/=10;
            }
            v[s as usize] = v[s as usize] +1;
        }
        for i in 1..46{
            if v[i]>max {
                max = v[i];
            }
        }
        max
    }
}