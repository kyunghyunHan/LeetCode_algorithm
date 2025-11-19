impl Solution {
      pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut count = 0;
        let mut col = (n - 1) as i32; 

        for i in 0..m {
            while col >= 0 && grid[i][col as usize] < 0 {
                col -= 1;
            }
            count += (n as i32 - 1 - col) as i32;
        }

        count
    }
}