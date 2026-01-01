impl Solution {
    pub fn max_containers(n: i32, w: i32, max_weight: i32) -> i32 {
        let mut k = n * n;
        let mut cnt = 0;
        for i in 1..=k{
            if w*i  > max_weight {
                break;
            }
            cnt+=1;
        }
        cnt
    }
}
