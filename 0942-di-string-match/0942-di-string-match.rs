impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let  s =s.chars().collect::<Vec<char>>();
        let n = s.len();
        let mut low :i32= 0;
        let mut high :i32= n as i32;
        let mut ans:Vec<i32> = vec![0;n+1];
        for i in 0..n{
        if  s[i]== 'I' {
            ans [i]  = low;
                        low+=1;

        }else{
            ans[i] = high;
                        high -=1; 

        }
        
        }
           ans[n] = low; 
        ans
    }
}