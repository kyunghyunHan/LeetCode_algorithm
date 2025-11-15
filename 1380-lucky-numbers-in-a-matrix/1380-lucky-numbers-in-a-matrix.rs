impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut res = Vec::new();

        for i in 0..m {
            let (min_col, &min_val) = matrix[i]
                .iter()
                .enumerate()
                .min_by_key(|&(_, v)| v)
                .unwrap();

            let mut max_in_col = i32::MIN;
            for row in 0..m {
                max_in_col = max_in_col.max(matrix[row][min_col]);
            }

            if min_val == max_in_col {
                res.push(min_val);
            }
        }

        res
    }
}
