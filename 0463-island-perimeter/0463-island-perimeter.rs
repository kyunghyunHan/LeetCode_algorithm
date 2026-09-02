impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let row = grid.len();
        let col = grid[0].len();

        let mut arr = vec![vec![0; col + 2]; row + 2];
        let mut res = 0;

        for i in 0..row {
            for j in 0..col {
                if grid[i][j] == 1 {
                    arr[i + 1][j + 1] = 1;
                }
            }
        }

        for i in 1..=row {
            for j in 1..=col {
                if arr[i][j] == 1 {
                    if arr[i + 1][j] == 0 {
                        res += 1;
                    }

                    if arr[i - 1][j] == 0 {
                        res += 1;
                    }

                    if arr[i][j + 1] == 0 {
                        res += 1;
                    }

                    if arr[i][j - 1] == 0 {
                        res += 1;
                    }
                }
            }
        }

        res
    }
}