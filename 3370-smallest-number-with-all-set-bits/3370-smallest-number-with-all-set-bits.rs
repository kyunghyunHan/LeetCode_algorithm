impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        let len = format!("{:b}",n).len();
        let result = (1 << len )-1;
        result
    }
}