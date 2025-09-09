impl Solution {
pub fn freq_alphabets(s: String) -> String {
    let s = s.as_bytes();
    let mut res = Vec::new();
    let mut i = s.len() as i32 - 1;

    while i >= 0 {
        if s[i as usize] == b'#' {
            let num = (s[(i-2) as usize] - b'0') * 10 + (s[(i-1) as usize] - b'0');
            res.push((b'a' + num - 1) as char);
            i -= 3;
        } else {
            let num = s[i as usize] - b'0';
            res.push((b'a' + num - 1) as char);
            i -= 1;
        }
    }

    res.iter().rev().collect()
}
}