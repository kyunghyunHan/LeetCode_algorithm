
impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut v = vec![];

        for i in 0..names.len(){
            v.push((names[i].to_string(),heights[i]))
        }

        v.sort_by(|a,b|b.1.cmp(&a.1));
        let result = v.iter().map(|x|x.0.to_string()).collect::<Vec<String>>();
        result
    }
}