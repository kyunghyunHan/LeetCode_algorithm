impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut stack: Vec<char> = Vec::new();

        for ch in s.chars() {
            if let Some(&top) = stack.last() {
                if (top == 'A' && ch == 'B') || (top == 'C' && ch == 'D') {
                    stack.pop();
                    continue;
                }
            }
            stack.push(ch);
        }

        stack.len() as i32
    }
}