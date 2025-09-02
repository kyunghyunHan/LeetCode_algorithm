impl Solution {
    pub fn maximum_value(strs: Vec<String>) -> i32 {
          let mut mx = 0;

    for s in strs {
        if let Ok(num) = s.parse::<i32>() {
            mx = mx.max(num);
        } else {
            mx = mx.max(s.len() as i32);
        }
    }
    mx
    }
}