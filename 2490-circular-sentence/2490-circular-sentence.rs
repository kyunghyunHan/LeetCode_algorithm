impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let words: Vec<&str> = sentence.split_whitespace().collect();

        if words[0].chars().next().unwrap() != words[words.len()-1].chars().last().unwrap() {
            return false;
        }

        for i in 1..words.len() {
            let prev_last = words[i-1].chars().last().unwrap();
            let curr_first = words[i].chars().next().unwrap();

            if prev_last != curr_first {
                return false;
            }
        }

        true
    }
}