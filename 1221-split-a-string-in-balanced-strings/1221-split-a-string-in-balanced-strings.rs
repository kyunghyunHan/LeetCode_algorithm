impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let s= s.chars().collect::<Vec<char>>();
        let mut cnt = 0;
        let mut rcnt = 0;
        let mut lcnt = 0;
        for i in 0..s.len(){
           if s[i] == 'R'{
             rcnt+=1;
           }if s[i]=='L'{
            lcnt +=1;
           }
           if rcnt ==lcnt{
             cnt+=1;
             lcnt =0;
             rcnt =0;
           }
        }
        cnt
    }
}