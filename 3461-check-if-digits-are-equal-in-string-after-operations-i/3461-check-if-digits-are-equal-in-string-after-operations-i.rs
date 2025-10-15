impl Solution {
    pub fn has_same_digits(mut s: String) -> bool {
        while s.len() > 2 {
            let chars: Vec<char> = s.chars().collect();
            let mut result = String::new();

            for i in 0..chars.len() - 1 {
                let a = chars[i].to_digit(10).unwrap();
                let b = chars[i + 1].to_digit(10).unwrap();
                let digit = (a + b) % 10;
                result.push(char::from_digit(digit, 10).unwrap());
            }

            s = result;
        }

        s.chars().all(|c| c == s.chars().next().unwrap())
    }
}