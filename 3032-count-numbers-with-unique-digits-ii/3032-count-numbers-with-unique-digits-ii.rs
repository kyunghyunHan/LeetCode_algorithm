impl Solution {
    pub fn number_count(a: i32, b: i32) -> i32 {
        let mut count = 0;
      for i in a..=b {
            let s = i.to_string();
            let chars: Vec<char> = s.chars().collect();
            let unique: std::collections::HashSet<char> = chars.iter().cloned().collect();
            if unique.len() == chars.len() {
                count += 1;
            }
        }

        count
    }
}