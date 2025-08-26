use std::collections::HashMap;

impl Solution {
    pub fn filter_characters(s: String, k: i32) -> String {
           let mut v = HashMap::new();
    for i in s.chars() {
        if v.contains_key(&i) {
            *v.get_mut(&i).unwrap() += 1;
        } else {
            v.insert(i, 1);
        }
    }

    let mut result = "".to_string();

    for i in s.chars() {
        if v.get(&i).unwrap() < &k {
            result += &i.to_string();
        }
    }
    result
    }
}