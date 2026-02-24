impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut res = 0;
        (x ^ y).count_ones() as i32

    }
}