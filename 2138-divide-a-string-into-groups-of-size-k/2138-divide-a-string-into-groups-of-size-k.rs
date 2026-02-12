impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let k = k as usize;
        let chars: Vec<char> = s.chars().collect();
        let mut res = Vec::new();

        let mut i = 0;
        while i < chars.len() {
            let mut group = String::new();

            for j in 0..k {
                if i + j < chars.len() {
                    group.push(chars[i + j]);
                } else {
                    group.push(fill);
                }
            }

            res.push(group);
            i += k;
        }

        res
    }
}
