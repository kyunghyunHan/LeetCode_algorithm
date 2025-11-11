impl Solution {
    pub fn delete_greatest_value(mut grid: Vec<Vec<i32>>) -> i32 {

        let si = grid.len();
        let sj = grid[0].len();
        let mut res = 0;

        for row in grid.iter_mut() {
            row.sort_unstable_by(|a, b| b.cmp(a));  
        }

        for j in 0..sj {
            let mut max_row = 0;
            for i in 0..si {
                max_row = max_row.max(grid[i][j]);
            }
            res += max_row;
        }

        res

    }
}