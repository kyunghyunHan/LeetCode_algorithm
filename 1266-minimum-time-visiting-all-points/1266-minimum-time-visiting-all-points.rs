impl Solution {
pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
    let mut cnt = 0;

    for i in 0..points.len() - 1 {
        let curr_x = points[i][0];
        let curr_y = points[i][1];

        let target_x = points[i + 1][0];
        let target_y = points[i + 1][1];

        cnt += (target_x - curr_x).abs().max((target_y - curr_y).abs())
    }
    cnt
}
}