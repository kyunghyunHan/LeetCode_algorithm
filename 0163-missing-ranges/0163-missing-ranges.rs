impl Solution {
  fn find_missing_ranges(nums: Vec<i32>, lower: i32, upper: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut prev = lower - 1;
    let mut extended_nums = nums.clone();
    extended_nums.push(upper + 1);

    for &num in &extended_nums {
        if num - prev >= 2 {
            let start = prev + 1;
            let end = num - 1;
            result.push(vec![start, end]);
        }
        prev = num;
    }

    result
}

}