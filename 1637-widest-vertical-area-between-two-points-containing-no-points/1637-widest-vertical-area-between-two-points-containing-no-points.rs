impl Solution {
    pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by(|a,b|a.cmp(&b));

        let mut max = 0;

        for i in 1..points.len(){
            let  width = points[i][0] - points[i-1][0];
            max = max.max(width);
        }
        max
    }
}