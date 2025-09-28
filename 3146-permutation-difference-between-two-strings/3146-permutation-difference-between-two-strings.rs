impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let t = t.chars().collect::<Vec<char>>();
        let mut result = 0;
        for i in 0..s.len(){
           let n = t.iter().position(|&x| x == s[i]).unwrap()  as i32;
           println!("{}",i);
           println!("{}",n);
           result += (i as i32 -n).abs();
        }
        result
    }
}