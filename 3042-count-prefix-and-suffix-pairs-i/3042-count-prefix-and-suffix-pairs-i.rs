impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let n = words.len();
        let mut count = 0;

        for i in 0..n {
            for j in (i + 1)..n {
                if Self::is_prefix_and_suffix(&words[i], &words[j]) {
                    count += 1;
                }
            }
        }

        count
    }

    fn is_prefix_and_suffix(str1: &str, str2: &str) -> bool {
        if str1.len() > str2.len() {
            return false;
        }

        let prefix = &str2[..str1.len()];
        let suffix = &str2[str2.len() - str1.len()..];

        prefix == str1 && suffix == str1
    }
}