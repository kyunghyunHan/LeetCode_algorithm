impl Solution {
  
pub fn find_the_difference(s: String, t: String) -> char {
    let mut sorted_s = s.chars().collect::<Vec<char>>();
    let mut sorted_t: Vec<char> = t.chars().collect::<Vec<char>>();

    sorted_s.sort_by(|a, b| a.cmp(b));
    sorted_t.sort_by(|a, b| a.cmp(b));

    let mut i = 0;

    while i < s.len() {
        if sorted_s[i] != sorted_t[i] {
            return sorted_t[i];
        }
        i += 1;
    }
    return sorted_t[i];
}

}