impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {

        let mut res= 0;

        let s = format!("{:b}",n);

        for c in s.chars(){
            if c=='1'{
                res+=1;
            }
        }
        res
    }
}