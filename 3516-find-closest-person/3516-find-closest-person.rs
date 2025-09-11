impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
         let x = (z -x).abs();
         let y =(z-y).abs();
         
         if x > y {
            2
         }else if x<y{
            1
         }else{
            0
         }

    }
}