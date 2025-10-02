impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let mut v = vec![0;26];
        let s  = s.chars().collect::<Vec<char>>();
        for i in 0..s.len(){
              let j = (s[i] as usize - 97);
              if v[j] > 0 &&  distance[j] as usize != i-v[j] as usize{
                return false
              }
              v[j] = i+1;
        }
        true
    }
}