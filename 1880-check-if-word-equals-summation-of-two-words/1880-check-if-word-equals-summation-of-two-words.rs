impl Solution {
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        
        let first_word  = {
            let mut result = "".to_string();
            for c in first_word.chars(){
                result+=  &(c as u32 - 'a' as u32).to_string()
            }
            result.parse::<u32>().unwrap()
        };
          let second_word  = {
            let mut result = "".to_string();
            for c in second_word.chars(){
                result+=  &(c as u32 - 'a' as u32).to_string()
            }
            result.parse::<u32>().unwrap()
        };
          let target_word  = {
            let mut result = "".to_string();
            for c in target_word.chars(){
                result+=  &(c as u32 - 'a' as u32).to_string()
            }
            result.parse::<u32>().unwrap()
        };

       first_word + second_word == target_word
    }
}