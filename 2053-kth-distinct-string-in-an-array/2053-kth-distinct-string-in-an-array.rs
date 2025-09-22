use std::collections::HashMap;
impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
          let mut freq: HashMap<&str, i32> = HashMap::new();

        for s in &arr {
            *freq.entry(s.as_str()).or_insert(0) += 1;
        }

        let mut count = 0;
        for s in &arr {
            if freq[s.as_str()] == 1 {
                count += 1;
                if count == k {
                    return s.clone();
                }
            }
        }

        String::new()
    }
}