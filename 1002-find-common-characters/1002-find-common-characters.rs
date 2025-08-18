impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
      
    let mut freq = vec![100; 26];

    for word in words {
        let mut count = vec![0; 26];
        for ch in word.chars() {
            count[ch as usize - 'a' as usize] += 1;
        }

        for i in 0..26 {
            freq[i] = freq[i].min(count[i]);
        }
    }

    let mut result = vec![];
    for i in 0..26 {
        if freq[i] > 0 {
            let ch = (i + 'a' as usize) as u8 as char;
            for _ in 0..freq[i] {
                result.push(ch.to_string());
            }
        }
        
    }
    result
    }
}