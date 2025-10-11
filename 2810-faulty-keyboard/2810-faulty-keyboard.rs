impl Solution {
    pub fn final_string(s: String) -> String {
        let mut result = String::new();
        for c in s.chars() {
            if c == 'i' {
                let reversed: String = result.chars().rev().collect();
                result = reversed;
            } else {
                result.push(c);
            }
        }
        result
    }
}