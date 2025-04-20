impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }
        
        let haystack_bytes = haystack.as_bytes();
        let needle_bytes = needle.as_bytes();
        let haystack_len = haystack_bytes.len();
        let needle_len = needle_bytes.len();
        
        if needle_len > haystack_len {
            return -1;
        }
        
        for i in 0..=haystack_len - needle_len {
            let mut matched = true;
            
            for j in 0..needle_len {
                if haystack_bytes[i + j] != needle_bytes[j] {
                    matched = false;
                    break;
                }
            }
            
            if matched {
                return i as i32;
            }
        }
        
        -1
    }
}