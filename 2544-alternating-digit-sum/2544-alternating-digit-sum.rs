impl Solution {
    pub fn alternate_digit_sum(n: i32) -> i32 {
          let n = n.to_string().chars().collect::<Vec<char>>();
    let mut result = 0;
    for i in 0..n.len() {
        let s;
        if i % 2 != 0 {
            s = -n[i].to_string().parse::<i32>().unwrap();
        } else {
            s = n[i].to_string().parse::<i32>().unwrap();
        }
        result += s;
    }
    result
    }
}