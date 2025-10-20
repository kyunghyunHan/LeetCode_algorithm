impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        let mut result =0;
        let mut i  = n;
        let mut s = start;
        loop{
            if i==0{
                break;
            }
             result ^=s;
             s +=2;
             i-=1;
        }
        result
    }
}