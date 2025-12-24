impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
      let w1: String = word1.concat();
       let w2: String = word2.concat();

        w1 == w2
    }
}