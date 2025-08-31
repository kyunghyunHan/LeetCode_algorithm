impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
       let n = strs.len();         
    let m = strs[0].len();       
    let chars: Vec<Vec<char>> = strs.iter().map(|s| s.chars().collect()).collect();

    let mut count = 0;
    for c in 0..m { 
        for r in 1..n { 
            if chars[r][c] < chars[r - 1][c] {
                count += 1;
                break; 
            }
        }
    }
    count
    }
}