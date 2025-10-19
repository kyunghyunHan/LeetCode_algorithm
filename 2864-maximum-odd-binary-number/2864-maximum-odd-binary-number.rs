impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let mut count = 0;
        for c in s.chars(){
            if c == '1' {
                count+=1;
            }
        }
        "1".repeat(count-1) + &("0".repeat(s.len()-count)) + "1"
    }
}