impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();

        let mut child = 0;
        let mut cookie = 0;

        while child < g.len() && cookie < s.len() {
            if s[cookie] >= g[child] {
                child += 1;
            }
            cookie += 1;
        }

        child as i32
    }
}