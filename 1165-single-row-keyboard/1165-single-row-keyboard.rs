impl Solution {
    pub fn calculate_time(keyboard: String, word: String) -> i32 {
        let mut currnet_len = 0;
        let mut count = 0;
        let keyboard = keyboard.chars().collect::<Vec<char>>();
        for c in word.chars(){
          for i in 0..keyboard.len(){
            if c == keyboard[i]{
                count+=(i as i32-currnet_len as i32).abs();
                currnet_len = i;
                continue;
            }
          }
        }
        count
    }
}