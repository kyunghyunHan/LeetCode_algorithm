impl Solution {
    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        let mut freq = [0i32; 26];

        for c in word1.chars() {
            freq[(c as u8 - b'a') as usize] += 1;
        }
        for c in word2.chars() {
            freq[(c as u8 - b'a') as usize] -= 1;
        }

        freq.iter().all(|&diff| diff.abs() <= 3)
    }
}