impl Solution {
  pub fn valid_word_square(words: Vec<String>) -> bool {
        let n = words.len();

        for i in 0..n {
            for j in 0..words[i].len() {
                if j >= n || i >= words[j].len() {
                    return false;
                }

                let c1 = words[i].chars().nth(j).unwrap();
                let c2 = words[j].chars().nth(i).unwrap();

                if c1 != c2 {
                    return false;
                }
            }
        }

        true
    }
}