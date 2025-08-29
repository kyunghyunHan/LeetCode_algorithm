impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
           let mut cnt = 0;
    for s in words {
        if s.contains(&pref) {
            if s[0..pref.len()] == pref {
                cnt += 1;
            }
        }
    }
    cnt
    }
}