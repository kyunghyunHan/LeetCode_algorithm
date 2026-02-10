impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut res= 0;
        let n = grid.len();
        let mut x;
        let mut y;

        for i in 0..n{
            x =0;
            y =0;
            for j in 0..n{
                x = x.max(grid[i][j]);
                y = y.max(grid[j][i]);
                if grid[i][j]!=0{
                    res+=1;
                }
            }
            res+=x+y;
        }
        res
    }
}