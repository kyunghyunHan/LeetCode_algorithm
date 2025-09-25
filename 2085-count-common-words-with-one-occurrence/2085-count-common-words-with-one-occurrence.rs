use std::collections::HashMap;

impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let mut w1 = HashMap::new();
        let mut w2 = HashMap::new();

        for w in words1.iter() {
        *w1.entry(w.clone()).or_insert(0) += 1;
    }

    for w in words2.iter() {
        if w1.contains_key(w) {
            *w2.entry(w.clone()).or_insert(0) += 1;
        }
    }

    let mut cnt = 0;
    for (word, &freq) in w1.iter() {
        if freq == 1 && w2.get(word) == Some(&1) {
            cnt += 1;
        }
    }

    cnt
    }
}