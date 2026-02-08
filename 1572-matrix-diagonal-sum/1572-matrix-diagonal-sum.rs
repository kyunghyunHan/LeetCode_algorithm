impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let mut sum = 0;
        for i in 0..mat.len(){
            sum+=mat[i][i];
            let mut j  = mat.len() -1 -i;
            if i!=j{
                sum +=mat[i][j];
            }
        }
        sum
    }
}