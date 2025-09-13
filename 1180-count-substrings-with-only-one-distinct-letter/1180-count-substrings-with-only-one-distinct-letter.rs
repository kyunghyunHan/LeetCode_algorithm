impl Solution {
     pub fn count_letters(s: String) -> i32 {
        let mut result = 0i32;
        let chars: Vec<char> = s.chars().collect();
        let mut count = 1i32;

        for i in 1..chars.len() {
            if chars[i] == chars[i - 1] {
                count += 1;
            } else {
                result += count * (count + 1) / 2;
                count = 1;
            }
        }
        result += count * (count + 1) / 2;
        result
    }
}