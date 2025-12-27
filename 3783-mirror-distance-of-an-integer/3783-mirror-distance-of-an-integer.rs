impl Solution {
    pub fn mirror_distance(n: i32) -> i32 {
      let mut rn =   n.to_string().chars().rev().collect::<String>().parse::<i32>().unwrap();
      rn.max(n) - rn.min(n)
    }
}