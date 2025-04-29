impl Solution {
pub fn is_subsequence(s: String, t: String) -> bool {
    let (mut i, mut j) = (0, 0);
    let s_bytes = s.as_bytes();
    let t_bytes = t.as_bytes();

    while i < s_bytes.len() && j < t_bytes.len() {
        if s_bytes[i] == t_bytes[j] {
            i += 1;
        }
        j += 1;
    }
    
    i == s_bytes.len()
}

}