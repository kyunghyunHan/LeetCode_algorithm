impl Solution {
    pub fn is_armstrong(n: i32) -> bool {
     let s = n.to_string();
    let s = s.chars().collect::<Vec<char>>();
    let s_len = s.len();
    let sum = s
        .iter()
        .map(|x| x.to_string().parse::<i32>().unwrap().pow(s_len as u32))
        .sum::<i32>();
         n == sum
    }
    
}