impl Solution {
    pub fn kth_character(k: i32) -> char {
  ((      'a' as u32 + (k-1).count_ones() )as u8)as char
    }
}