impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        
        for word in words{
            if word == word.chars().rev().collect::<String>()
{
                   return word
            }
        }
        "".to_string()
        
    }
}