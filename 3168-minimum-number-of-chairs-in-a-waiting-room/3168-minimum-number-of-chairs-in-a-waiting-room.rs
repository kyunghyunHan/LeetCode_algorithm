impl Solution {
    pub fn minimum_chairs(s: String) -> i32 {
        let mut max = 0;
        let mut cnt = 0;
        let s= s.chars().collect::<Vec<char>>();
        for i in 0..s.len(){
            if s[i] == 'E'{
                cnt+=1;
            }else if s[i] == 'L'{
                cnt-=1;
            }
            max = max.max(cnt);
        }
        max
    }
}