impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let s = s.chars().collect::<Vec<char>>();
           let mut result = "".to_string();

    let mut balance = 0;

    for i in 0..s.len() {
        if s[i] == '(' {
            if balance > 0 {
                result += &format!("{}", s[i]);
            }
            balance += 1;
        } else {
            balance -= 1;

            if balance>0{
                result += &format!("{}", s[i]);
            }
        }
    }
    result
    }
}