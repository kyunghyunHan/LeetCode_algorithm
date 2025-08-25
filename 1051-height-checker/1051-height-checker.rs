impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut heights_clone = heights.clone();
    let mut cnt = 0;
    heights_clone.sort_by(|a, b| a.cmp(&b));

    for i in 0..heights.len() {
        if heights[i] != heights_clone[i] {
            cnt += 1;
        }
    }
    cnt
    }
}