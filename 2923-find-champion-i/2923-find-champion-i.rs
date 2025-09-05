impl Solution {
    pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
        let mut result  = 0;
        let mut maxsum = 0;
        for i in 0..grid.len(){
            let max = grid[i].iter().sum();        
            println!("{}",maxsum);
            println!("{}",max);

            if max > maxsum{
                maxsum = max;
                result = i;
            }
            
        }
        result as i32
    }
}