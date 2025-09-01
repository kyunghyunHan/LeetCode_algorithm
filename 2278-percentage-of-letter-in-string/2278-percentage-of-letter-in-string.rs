impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
         let mut cnt = 0;
        for i in 0..s.len(){
            if s[i] == letter{
                cnt+=1;
            }
        }
        println!("{}", (cnt as f32 / s.len() as f32 * 100.));
        // (cnt / s.len() * 100)
        (cnt as f64 / s.len() as f64 * 100.).abs() as i32
    }
}