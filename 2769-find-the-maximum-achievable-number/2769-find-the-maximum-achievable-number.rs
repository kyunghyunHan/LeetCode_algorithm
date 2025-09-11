impl Solution {
    pub fn the_maximum_achievable_x(mut num: i32, t: i32) -> i32 {
        for _ in 0..t {
            num+=2;
        }
        num
    }
}