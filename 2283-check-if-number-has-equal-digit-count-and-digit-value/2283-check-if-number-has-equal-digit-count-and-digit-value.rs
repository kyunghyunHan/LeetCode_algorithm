use std::collections::HashMap;

impl Solution {
    pub fn digit_count(num: String) -> bool {
         let bytes = num.as_bytes();
        let mut count = vec![0; 10];

        for &b in bytes {
            count[(b - b'0') as usize] += 1;
        }

        for (i, &b) in bytes.iter().enumerate() {
            let expected = (b - b'0') as usize;
            if count[i] != expected {
                return false;
            }
        }

        true
    }
}