impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let s  = s.chars().collect::<Vec<char>>();
        let mut sum =0;
        let mut c = 1;
        for i in 0..s.len(){
               sum +=widths[s[i] as usize - 'a' as usize];
               if sum > 100 {
                c+=1;
                sum= widths[s[i] as usize - 'a' as usize];
               }
        } 
        vec![c,sum]
    }
}