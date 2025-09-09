impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        let mut result  = Vec::new();
        
        if original.len() != (n*m) as usize{
                return vec![];
        }
        for i in (0..original.len()).step_by(n as usize){
            let mut arr =vec![];
            for j in i..i+n as usize{
                arr.push(original[j as usize])
            }
            result.push(arr);
        }
      
        result
    }
}