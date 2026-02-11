impl Solution {
    pub fn odd_cells(n: i32, m: i32, indices: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let m = m as usize;
        let mut cnt = 0;
        let mut row = vec![0;n as usize];
        let mut col = vec![0;m as usize];

        for indice in indices{ 
           row[indice[0]as usize]+=1;
           col[indice[1]as usize ]+=1;
        }
        for i in 0..n{
            for j in 0..m{
                if (row[i] + col[j]) %2 !=0{
                    cnt+=1;
                }
            }
        }
        cnt
    }
}
