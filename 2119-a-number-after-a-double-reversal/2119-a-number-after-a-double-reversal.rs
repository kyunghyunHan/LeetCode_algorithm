impl Solution {
    pub fn is_same_after_reversals(num: i32) -> bool {
        if num ==0 {
            return true
        }

        let num = num.to_string().chars().collect::<Vec<char>>();

        if num.last().unwrap()==&'0'{
            false
        }else {
            true
        }
    }
}