impl Solution {
    pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
        let mut res = vec![];
        let n  = height.len();
        for i  in 1..n{
            if height[i-1] > threshold{
                  res.push(i as i32)
            }
        }
        res
    }
}