impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
            let mut count = 0;

    for s in words {
        let mut result = true;
        for c in s.chars() {
            if !allowed.contains(c.to_owned()) {
                result = false;
            }
        }
        if result == true {
            count += 1;
        }
    }
    count
    }
}