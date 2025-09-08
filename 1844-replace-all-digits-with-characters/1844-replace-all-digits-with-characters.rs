impl Solution {
    pub fn replace_digits(s: String) -> String {
         let s = s.chars().collect::<Vec<char>>();
         let mut result = Vec::new();
             for i in 0..s.len() {
            if i % 2 == 0 {
                result.push(s[i]);
            } else {
                let shift = s[i] as u32 - '0' as u32;
                let new_c = std::char::from_u32(s[i - 1] as u32 + shift).unwrap();
                result.push(new_c);
            }
        }

         result.iter().collect::<String>()
    }
}