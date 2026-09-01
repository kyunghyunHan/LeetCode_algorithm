use std::collections::HashSet;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut n = n;
        let mut seen = HashSet::new();

        while n != 1 {
            if seen.contains(&n) {
                return false;
            }

            seen.insert(n);

            let s = n.to_string().chars().collect::<Vec<char>>();
            let mut sum = 0;

            for i in 0..s.len() {
                sum += (s[i] as i32 - '0' as i32).pow(2);
            }

            n = sum;
        }

        true
    }
}