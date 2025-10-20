impl Solution {
      pub fn decode_message(key: String, message: String) -> String {
        let mut map = [0u8; 26];       
        let mut used = [false; 26];   
        let mut next_char = b'a';      
        for ch in key.bytes() {
            if ch == b' ' {
                continue;
            }
            let idx = (ch - b'a') as usize;
            if !used[idx] {
                used[idx] = true;
                map[idx] = next_char;
                next_char += 1;
            }
        }

        let mut result = String::with_capacity(message.len());
        for ch in message.bytes() {
            if ch == b' ' {
                result.push(' ');
            } else {
                let decoded = map[(ch - b'a') as usize] as char;
                result.push(decoded);
            }
        }

        result
    }
}