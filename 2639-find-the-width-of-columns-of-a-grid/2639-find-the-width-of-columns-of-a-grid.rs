impl Solution {
    pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut len =0;
        let mut st = "".to_string();
        let mut mxlen= 0;

        let mut ans = vec![];

        for col in 0..grid[0].len(){
            mxlen = 0;
            for row in 0..grid.len(){
                st = grid[row][col].to_string();
                len = st.len();
                mxlen= mxlen.max(len);
            }
            ans.push(mxlen as i32);
        }
        ans
    }
}