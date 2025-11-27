impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut n = grid.len();

let  mut res = vec![vec![0; n - 2]; n - 2];

        for i in 1..n-1{
            for j in 1..n-1{
                let mut temp = 0;
                for k in i-1..=i+1{
                    for l in j-1..=j+1{
                       temp = temp.max(grid[k][l]);
                    }
                }

               res[i-1][j-1]= temp;
            }
        }
        res
    }
}