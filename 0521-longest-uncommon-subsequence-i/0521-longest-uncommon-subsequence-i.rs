impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        match a==b{
            true => {
                return -1;
            }
            _=>{
                a.len().max(b.len()) as i32
            }
        }
    }
}