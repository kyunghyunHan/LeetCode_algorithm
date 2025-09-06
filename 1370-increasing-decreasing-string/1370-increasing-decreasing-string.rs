impl Solution {
    pub fn sort_string(s: String) -> String {
         let mut count = vec![0; 26];
    for ch in s.chars() {
        count[(ch as u8 - b'a') as usize] += 1;
    }
    
    let mut result = String::new();
    while result.len() < s.len() {
        for i in 0..26 {
            if count[i] > 0 {
                result.push((b'a' + i as u8) as char);
                count[i] -= 1;
            }
        }
        for i in (0..26).rev() {
            if count[i] > 0 {
                result.push((b'a' + i as u8) as char);
                count[i] -= 1;
            }
        }
    }
    result
    }
}